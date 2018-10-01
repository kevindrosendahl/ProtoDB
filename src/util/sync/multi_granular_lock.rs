use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use std::thread;
use std::thread::Thread;

/// A fair multi granular lock supporting both blocking and non-blocking
/// lock acquisition. Respects the following compatibility matrix:
///       IS  IX  S  SIX  X
///     ---------------------
///  IS | Y | Y | Y | Y | N |
///  IX | Y | Y | N | N | N |
///  S  | Y | N | Y | N | N |
/// SIX | Y | N | N | N | N |
///  X  | N | N | N | N | N |
///     ---------------------
/// The lock is fair in that it grants the lock based on a FIFO queue.
/// It will lock only for the first thread that attempts to acquire it.
/// After that, threads will queue up to acquire the lock and park
/// themselves. Once the original thread has relinquished the lock,
/// the first thread in the queue will be granted the lock and unparked,
/// along with any other threads in the queue that are compatible with
/// its granularity.
#[derive(Debug)]
pub struct MultiGranularLock {
    // locked indicates whether or not the lock is locked at any level.
    locked: AtomicBool,

    // wait_queue contains all of the threads that are currently waiting
    // to acquire the lock along with the granularity at which they
    // wish to acquire the lock.
    // The mutex for wait_queue must be acquired before the mutex
    // for state. The wait_queue mutex both guards access to the
    // WaitQueue but also serves as synchronization between threads that
    // are dropping guards for the lock as well as threads that are
    // attempting to acquire the lock.
    wait_queue: Mutex<WaitQueue>,

    // state contains information about the current granularity levels
    // the lock is locked with.
    // The mutex for state can only be acquired when the wait_queue
    // mutex is already acquired.
    state: Mutex<State>,
}

#[allow(dead_code)]
#[derive(Debug)]
struct State {
    intention_shared: usize,
    intention_exclusive: usize,
    shared: usize,
    shared_and_intention_exclusive: bool,
    exclusive: bool,
}

impl State {
    pub fn acquired(&self) -> bool {
        !self.can_acquire(Granularity::Exclusive)
    }

    pub fn acquire(&mut self, granularity: Granularity) {
        match granularity {
            Granularity::IntentionShared => self.intention_shared += 1,
            Granularity::IntentionExclusive => self.intention_exclusive += 1,
            Granularity::Shared => self.shared += 1,
            Granularity::SharedAndIntentionExclusive => self.shared_and_intention_exclusive = true,
            Granularity::Exclusive => self.exclusive = true,
        }
    }

    pub fn release(&mut self, granularity: Granularity) {
        match granularity {
            Granularity::IntentionShared => self.intention_shared -= 1,
            Granularity::IntentionExclusive => self.intention_exclusive -= 1,
            Granularity::Shared => self.shared -= 1,
            Granularity::SharedAndIntentionExclusive => self.shared_and_intention_exclusive = false,
            Granularity::Exclusive => self.exclusive = false,
        }
    }

    pub fn can_acquire(&self, granularity: Granularity) -> bool {
        match granularity {
            Granularity::IntentionShared => !self.exclusive,
            Granularity::IntentionExclusive => {
                !(self.exclusive || self.shared_and_intention_exclusive || self.shared > 0)
            }
            Granularity::Shared => {
                !(self.exclusive
                    || self.shared_and_intention_exclusive
                    || self.intention_exclusive > 0)
            }
            Granularity::SharedAndIntentionExclusive => {
                !(self.exclusive
                    || self.shared_and_intention_exclusive
                    || self.shared > 0
                    || self.intention_exclusive > 0)
            }
            Granularity::Exclusive => {
                !(self.exclusive
                    || self.shared_and_intention_exclusive
                    || self.shared > 0
                    || self.intention_exclusive > 0
                    || self.intention_shared > 0)
            }
        }
    }
}

#[derive(Debug)]
struct WaitQueue {
    inner: VecDeque<WaitListEntry>,
}

#[derive(Debug)]
struct WaitListEntry {
    pub thread: Thread,
    pub granularity: Granularity,
}

impl WaitQueue {
    pub fn new() -> WaitQueue {
        WaitQueue {
            inner: VecDeque::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn push(&mut self, thread: Thread, granularity: Granularity) {
        self.inner.push_back(WaitListEntry {
            thread,
            granularity,
        })
    }

    pub fn pop(&mut self) -> Option<WaitListEntry> {
        self.inner.pop_front()
    }

    pub fn wake_compatible(&mut self, state: &mut State) {
        self.inner.retain(|entry| {
            if !state.can_acquire(entry.granularity.clone()) {
                return true;
            }

            state.acquire(entry.granularity.clone());
            entry.thread.unpark();
            false
        })
    }
}

/// The different granularities that can be requested
/// to acquire the lock with.
#[derive(Debug, Clone, Copy)]
pub enum Granularity {
    #[allow(dead_code)]
    IntentionShared,
    #[allow(dead_code)]
    IntentionExclusive,
    #[allow(dead_code)]
    Shared,
    #[allow(dead_code)]
    SharedAndIntentionExclusive,
    #[allow(dead_code)]
    Exclusive,
}

/// The value returned by a successful acquisition of the lock.
/// When the guard goes out of scope, it will clean up the necessary
/// bookkeeping about the granularity it had been acquired with,
/// and attempt to wake up any threads that can try again to acquire
/// the lock under the new semantics.
#[allow(dead_code)]
#[derive(Debug)]
pub struct MultiGranularLockGuard<'a> {
    lock: &'a MultiGranularLock,
    granularity: Granularity,
}

impl<'a> Drop for MultiGranularLockGuard<'a> {
    #[inline]
    fn drop(&mut self) {
        // Acquire the state lock. This serves as a point of synchronization
        // ensuring that all of the guards' drop methods are serialized both
        // with each other as well as with other threads attempting to
        // acquire the lock.
        let mut wait_queue = self.lock.wait_queue.lock().unwrap();

        let mut state = self.lock.state.lock().unwrap();
        state.release(self.granularity);

        // If the state is still acquired at some level, there is another
        // guard that has yet to be dropped. When it is dropped it will
        // finish cleaning up, so we can return out early here.
        if state.acquired() {
            return;
        }

        if let Some(first) = wait_queue.pop() {
            state.acquire(first.granularity);
            first.thread.unpark();

            wait_queue.wake_compatible(&mut state);
            return;
        }

        // If there were no waiting threads, then we can mark the lock as
        // unlocked, freeing up new threads to come acquire the lock.
        // It is possible that there are other threads in acquire() right
        // now, but they would be waiting to acquire the wait_queue lock.
        // Once we set locked to false, we'll release the wait_queue lock,
        // and the winning thread will acquire the lock and re-check locked,
        // see that it is false and acquire the lock.
        self.lock.locked.swap(false, Ordering::SeqCst);
    }
}

impl MultiGranularLock {
    #[allow(dead_code)]
    pub fn new() -> MultiGranularLock {
        MultiGranularLock {
            locked: AtomicBool::new(false),

            wait_queue: Mutex::new(WaitQueue::new()),
            state: Mutex::new(State {
                intention_shared: 0,
                intention_exclusive: 0,
                shared: 0,
                shared_and_intention_exclusive: false,
                exclusive: false,
            }),
        }
    }

    /// Blocks until the lock can be acquired with the specified granularity.
    #[allow(dead_code)]
    pub fn acquire(&self, granularity: Granularity) -> MultiGranularLockGuard<'_> {
        // If we can acquire the lock, do so and return the guard.
        if let Some(g) = self.try_acquire(granularity) {
            return g;
        }

        {
            // Otherwise, grab the lock on self.wait_queue. This lock will synchronize
            // both threads attempting to acquire the lock as well as threads
            // dropping guards from the lock.
            let mut wait_queue = self.wait_queue.lock().unwrap();

            // If self.wait_queue is empty, then it's possible that between our first attempt
            // to acquire the lock and now that the final guard that was given out from a
            // previous acquisition was just released, and that the lock is now actually
            // available. So now that we're holding the self.wait_queue lock, we'll check
            // again to see if the lock is available.
            // If it's not, then we'll add ourselves to self.wait_queue and we should be the
            // next in line to acquire the lock.
            if wait_queue.len() == 0 {
                let still_locked = self.locked.compare_and_swap(false, true, Ordering::SeqCst);
                if !still_locked {
                    let mut state = self.state.try_lock().expect("unable to acquire internal state lock when freshly acquiring previously locked lock");
                    state.acquire(granularity);
                    return MultiGranularLockGuard {
                        lock: &self,
                        granularity,
                    };
                }
            }

            wait_queue.push(thread::current(), granularity);
        }

        // Park our thread for now. When we get woken back up that means that
        // during the drop of the prior lock this thread was determined to either
        // be the first in line for the lock or compatible with the lock
        // that was first in line.
        thread::park();

        MultiGranularLockGuard {
            lock: &self,
            granularity,
        }
    }

    /// Attempts to acquire the lock at the specified granularity without blocking.
    #[allow(dead_code)]
    pub fn try_acquire(&self, granularity: Granularity) -> Option<MultiGranularLockGuard<'_>> {
        // Check to see if the lock is already acquired at any granularity.
        // If it is, return None.
        let already_locked = self.locked.compare_and_swap(false, true, Ordering::SeqCst);
        if already_locked {
            return None;
        }

        // We need the self.wait_queue lock for later to call wake_compatible, but
        // we need to acquire it first to adhere to the lock acquisition order
        // requirements.
        let mut wait_queue = self.wait_queue.lock().unwrap();

        // If the lock was not already acquired at any granularity, then we just
        // acquired it by setting self.locked to true.
        // Our thread should be the only thread that can possibly be trying to acquire the state
        // lock right now. All other threads calling try_acquire should have failed to set
        // self.locked, and self.locked should only be false if either this lock has never
        // been acquired, or if the last MultiGranularLockGuard has had drop invoked and
        // it has switched self.locked to false, at which point it should have dropped
        // the self.state mutex.
        // So if self.state.try_lock() fails, we have a bug with either our state bookkeeping
        // logic or with the MultiGranularLockGuard drop logic.
        let mut state = self
            .state
            .try_lock()
            .expect("unable to acquire internal state lock when freshly acquiring lock");
        state.acquire(granularity);

        // We were able to acquire the lock first, but it's possible that other thread
        // have queued up for the lock in the meantime. Wake any other threads who are
        // compatible with the granularity we're locking at.
        wait_queue.wake_compatible(&mut state);

        Some(MultiGranularLockGuard {
            lock: &self,
            granularity,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};
    use std::sync::{Arc, Condvar, Mutex};
    use std::thread;

    use super::{Granularity, MultiGranularLock, State};

    #[test]
    fn drop() {
        let l = MultiGranularLock::new();
        {
            let g = l.try_acquire(Granularity::Exclusive);
            assert!(g.is_some());
        }

        {
            let g = l.try_acquire(Granularity::SharedAndIntentionExclusive);
            assert!(g.is_some());
        }

        {
            let g = l.try_acquire(Granularity::Shared);
            assert!(g.is_some());
        }

        {
            let g = l.try_acquire(Granularity::IntentionExclusive);
            assert!(g.is_some());
        }

        {
            let g = l.try_acquire(Granularity::IntentionShared);
            assert!(g.is_some());
        }
    }

    struct AcquireTest {
        queued_granularities: Vec<Granularity>,
        expected_success: HashSet<usize>,
        expected_state: State,
    }

    #[test]
    fn acquire_exclusive() {
        test_acquire(AcquireTest {
            queued_granularities: vec![
                Granularity::Exclusive,
                Granularity::Exclusive,
                Granularity::SharedAndIntentionExclusive,
                Granularity::SharedAndIntentionExclusive,
                Granularity::Shared,
                Granularity::Shared,
                Granularity::IntentionExclusive,
                Granularity::IntentionExclusive,
                Granularity::IntentionShared,
                Granularity::IntentionShared,
            ],
            expected_success: HashSet::new(),
            expected_state: State {
                exclusive: true,
                shared_and_intention_exclusive: false,
                shared: 0,
                intention_exclusive: 0,
                intention_shared: 0,
            },
        })
    }

    #[test]
    fn acquire_shared_intention_exclusive() {
        let mut expected_success = HashSet::new();
        expected_success.insert(9);
        expected_success.insert(10);
        test_acquire(AcquireTest {
            queued_granularities: vec![
                Granularity::SharedAndIntentionExclusive,
                Granularity::Exclusive,
                Granularity::Exclusive,
                Granularity::SharedAndIntentionExclusive,
                Granularity::SharedAndIntentionExclusive,
                Granularity::Shared,
                Granularity::Shared,
                Granularity::IntentionExclusive,
                Granularity::IntentionExclusive,
                Granularity::IntentionShared,
                Granularity::IntentionShared,
            ],
            expected_success,
            expected_state: State {
                exclusive: false,
                shared_and_intention_exclusive: true,
                shared: 0,
                intention_exclusive: 0,
                intention_shared: 2,
            },
        })
    }

    #[test]
    fn acquire_shared() {
        let mut expected_success = HashSet::new();
        expected_success.insert(5);
        expected_success.insert(6);
        expected_success.insert(9);
        expected_success.insert(10);
        test_acquire(AcquireTest {
            queued_granularities: vec![
                Granularity::Shared,
                Granularity::Exclusive,
                Granularity::Exclusive,
                Granularity::SharedAndIntentionExclusive,
                Granularity::SharedAndIntentionExclusive,
                Granularity::Shared,
                Granularity::Shared,
                Granularity::IntentionExclusive,
                Granularity::IntentionExclusive,
                Granularity::IntentionShared,
                Granularity::IntentionShared,
            ],
            expected_success,
            expected_state: State {
                exclusive: false,
                shared_and_intention_exclusive: false,
                shared: 3,
                intention_exclusive: 0,
                intention_shared: 2,
            },
        })
    }

    #[test]
    fn acquire_intention_exclusive() {
        let mut expected_success = HashSet::new();
        expected_success.insert(7);
        expected_success.insert(8);
        expected_success.insert(9);
        expected_success.insert(10);
        test_acquire(AcquireTest {
            queued_granularities: vec![
                Granularity::IntentionExclusive,
                Granularity::Exclusive,
                Granularity::Exclusive,
                Granularity::SharedAndIntentionExclusive,
                Granularity::SharedAndIntentionExclusive,
                Granularity::Shared,
                Granularity::Shared,
                Granularity::IntentionExclusive,
                Granularity::IntentionExclusive,
                Granularity::IntentionShared,
                Granularity::IntentionShared,
            ],
            expected_success,
            expected_state: State {
                exclusive: false,
                shared_and_intention_exclusive: false,
                shared: 0,
                intention_exclusive: 3,
                intention_shared: 2,
            },
        })
    }

    #[test]
    fn acquire_intention_shared() {
        // SharedAndIntentionExclusive is the highest priority
        {
            let mut expected_success = HashSet::new();
            expected_success.insert(3);
            expected_success.insert(9);
            expected_success.insert(10);
            test_acquire(AcquireTest {
                queued_granularities: vec![
                    Granularity::IntentionShared,
                    Granularity::Exclusive,
                    Granularity::Exclusive,
                    Granularity::SharedAndIntentionExclusive,
                    Granularity::SharedAndIntentionExclusive,
                    Granularity::Shared,
                    Granularity::Shared,
                    Granularity::IntentionExclusive,
                    Granularity::IntentionExclusive,
                    Granularity::IntentionShared,
                    Granularity::IntentionShared,
                ],
                expected_success,
                expected_state: State {
                    exclusive: false,
                    shared_and_intention_exclusive: true,
                    shared: 0,
                    intention_exclusive: 0,
                    intention_shared: 3,
                },
            })
        }

        // Shared is the highest priority
        {
            let mut expected_success = HashSet::new();
            expected_success.insert(3);
            expected_success.insert(6);
            expected_success.insert(9);
            expected_success.insert(10);
            test_acquire(AcquireTest {
                queued_granularities: vec![
                    Granularity::IntentionShared,
                    Granularity::Exclusive,
                    Granularity::Exclusive,
                    Granularity::Shared,
                    Granularity::SharedAndIntentionExclusive,
                    Granularity::SharedAndIntentionExclusive,
                    Granularity::Shared,
                    Granularity::IntentionExclusive,
                    Granularity::IntentionExclusive,
                    Granularity::IntentionShared,
                    Granularity::IntentionShared,
                ],
                expected_success,
                expected_state: State {
                    exclusive: false,
                    shared_and_intention_exclusive: false,
                    shared: 2,
                    intention_exclusive: 0,
                    intention_shared: 3,
                },
            })
        }

        // IntentionExclusive is the highest priority
        {
            let mut expected_success = HashSet::new();
            expected_success.insert(3);
            expected_success.insert(8);
            expected_success.insert(9);
            expected_success.insert(10);
            test_acquire(AcquireTest {
                queued_granularities: vec![
                    Granularity::IntentionShared,
                    Granularity::Exclusive,
                    Granularity::Exclusive,
                    Granularity::IntentionExclusive,
                    Granularity::Shared,
                    Granularity::Shared,
                    Granularity::SharedAndIntentionExclusive,
                    Granularity::SharedAndIntentionExclusive,
                    Granularity::IntentionExclusive,
                    Granularity::IntentionShared,
                    Granularity::IntentionShared,
                ],
                expected_success,
                expected_state: State {
                    exclusive: false,
                    shared_and_intention_exclusive: false,
                    shared: 0,
                    intention_exclusive: 2,
                    intention_shared: 3,
                },
            })
        }
    }

    fn test_acquire(mut test: AcquireTest) {
        test.expected_success.insert(0);

        let l = Arc::new(MultiGranularLock::new());
        let l1 = l.clone();

        let acquired_pair = Arc::new((Mutex::new(0), Condvar::new()));
        let checked_pair = Arc::new((Mutex::new(false), Condvar::new()));
        let mut threads = Vec::new();
        let thread_ids = Arc::new(Mutex::new(HashMap::new()));
        {
            // First, acquire the lock exclusively. It doesn't really matter what granularity
            // we acquire the lock originally with, since it will only ever be acquired by one
            // thread initially, essentially having exclusive lock semantics.
            let g = l1.try_acquire(Granularity::Exclusive);
            assert!(g.is_some());

            // For each of the desired queued granularities, spawn a thread.
            // The thread will then spin until all of the previous threads have queued
            // their granularity and attempted to acquire the lock.
            // Then, the thread will attempt to acquire the lock.
            // Once the thread has acquired the lock, it will increment the acquired
            // counter and notify on the condition variable.
            // Then it will wait for the main test thread to notify it that it is done
            // doing its checks, at which point it will exit the thread, dropping the lock.
            for (i, g) in test.queued_granularities.iter().enumerate() {
                let l2 = l.clone();
                let g2 = g.clone();
                let acquired_pair = acquired_pair.clone();
                let checked_pair = checked_pair.clone();
                let thread_ids = thread_ids.clone();

                let t = thread::spawn(move || {
                    loop {
                        let wait_queue = l2.wait_queue.lock().unwrap();
                        if wait_queue.len() >= i {
                            break;
                        }
                    }

                    {
                        let mut thread_ids = thread_ids.lock().unwrap();
                        thread_ids.insert(thread::current().id(), i);
                    }

                    let _l = l2.acquire(g2.clone());

                    {
                        let &(ref lock, ref cvar) = &*acquired_pair;
                        let mut acquired = lock.lock().unwrap();
                        *acquired += 1;
                        cvar.notify_one();
                    }

                    let &(ref lock, ref cvar) = &*checked_pair;
                    let mut checked = lock.lock().unwrap();
                    while !*checked {
                        checked = cvar.wait(checked).unwrap();
                    }
                });

                threads.push(t);
            }

            loop {
                let wait_queue = l1.wait_queue.lock().unwrap();
                if wait_queue.len() == test.queued_granularities.len() {
                    break;
                }
            }
        } // drop the lock

        // Wait for the expected number of lock acquisitions to happen.
        {
            let &(ref lock, ref cvar) = &*acquired_pair;
            let mut acquired = lock.lock().unwrap();
            while *acquired < 1 {
                acquired = cvar.wait(acquired).unwrap();
            }
        }

        // Ensure that the correct threads were woken up and that the state is as expected.
        {
            let expected_waiting = test.queued_granularities.len() - test.expected_success.len();

            let wait_queue = l
                .wait_queue
                .try_lock()
                .expect("unable to get wait_queue lock");

            let thread_ids = thread_ids.clone();
            let thread_ids = thread_ids.lock().unwrap();

            let entries = &wait_queue.inner;
            for entry in entries {
                let idx = thread_ids.get(&entry.thread.id()).unwrap();
                if test.expected_success.contains(idx) {
                    panic!("expected thread {} to have acquired the lock", idx);
                }
            }

            assert_eq!(wait_queue.len(), expected_waiting);

            let state = l.state.lock().expect("unable to get state lock");
            assert!(state.acquired());

            assert_eq!(state.exclusive, test.expected_state.exclusive);
            assert_eq!(
                state.shared_and_intention_exclusive,
                test.expected_state.shared_and_intention_exclusive
            );
            assert_eq!(state.shared, test.expected_state.shared);
            assert_eq!(
                state.intention_exclusive,
                test.expected_state.intention_exclusive
            );
            assert_eq!(state.intention_shared, test.expected_state.intention_shared);
        }

        // Alert the threads we are done checking so they can exit.
        {
            let &(ref lock, ref cvar) = &*checked_pair;
            let mut checked = lock.lock().unwrap();
            *checked = true;
            cvar.notify_all();
        }

        for thread in threads {
            thread.join().unwrap();
        }
    }
}
