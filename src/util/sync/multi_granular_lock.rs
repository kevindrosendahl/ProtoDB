use std::sync::Mutex;
use std::thread;
use std::thread::Thread;

/// A multi granular lock supporting both blocking and non-blocking
/// lock acquisition. Respects the following compatibility:
///       IS  IX  S  SIX  X
///     ---------------------
///  IS | Y | Y | Y | Y | N |
///  IX | Y | Y | N | N | N |
///  S  | Y | N | Y | N | N |
/// SIX | Y | N | N | N | N |
///  X  | N | N | N | N | N |
///     ---------------------
#[derive(Debug)]
pub struct MultiGranularLock {
    lock: Mutex<()>,

    waiting: Mutex<Vec<(Thread, Granularity)>>,
    state: Mutex<MultiGranularLockState>,
}

#[derive(Debug)]
struct MultiGranularLockState {
    intention_shared: usize,
    intention_exclusive: usize,
    shared: usize,
    shared_and_intention_exclusive: usize,
    exclusive: bool,
}

/// The different granularities that can be requested
/// to acquire the lock with.
#[derive(Debug, Clone, Copy)]
pub enum Granularity {
    IntentionShared,
    IntentionExclusive,
    Shared,
    SharedAndIntentionExclusive,
    Exclusive,
}

/// The value returned by a successful acquisition of the lock.
/// When the guard goes out of scope, it will clean up the necessary
/// bookkeeping about the granularity it had been acquired with,
/// and attempt to wake up any threads that can try again to acquire
/// the lock under the new semantics.
#[derive(Debug)]
pub struct MultiGranularLockGuard<'a> {
    lock: &'a MultiGranularLock,
    granularity: Granularity,
}

impl<'a> Drop for MultiGranularLockGuard<'a> {
    #[inline]
    fn drop(&mut self) {
        let _l = self.lock.lock.lock().unwrap();

        {
            let mut state = self.lock.state.lock().unwrap();
            match self.granularity {
                Granularity::IntentionShared => (*state).intention_shared -= 1,
                Granularity::IntentionExclusive => (*state).intention_exclusive -= 1,
                Granularity::Shared => (*state).shared -= 1,
                Granularity::SharedAndIntentionExclusive => {
                    (*state).shared_and_intention_exclusive -= 1
                }
                Granularity::Exclusive => (*state).exclusive = false,
            }
        }

        // While still holding the mutex, find all threads that can possibly acquire the lock
        // with its desired granularity now that this guard has been dropped, and remove
        // the threads from the list of waiting threads.
        //
        // This will essentially end up leading to a race where the first thread
        // to acquire the mutex will determine the new semantics for the lock.
        // This could potentially lead to starvation. An alternative could be finding
        // the most exclusive granularity thread that's attempting to acquire the lock
        // and unpark it or something similar. However, that thread would still
        // potentially be racing against any new thread that are attempting to acquire
        // the lock. A different approach also could involve having this guard choose
        // a winning thread or threads from the waiting threads and do their bookkeeping
        // for them, and unpark them after. Here, unparking would mean "you've acquired
        // the lock," not "when you were unparked, you could acquire the lock, that
        // may or may not still be true." Seems like the best way forward is the simplest
        // for now though.
        self.lock.waiting.lock().unwrap().retain(|(t, g)| {
            if !self.lock.can_acquire(g.clone()) {
                return true;
            }

            t.unpark();
            false
        });
    }
}

impl MultiGranularLock {
    pub fn new() -> MultiGranularLock {
        MultiGranularLock {
            lock: Mutex::new(()),

            waiting: Mutex::new(Vec::new()),
            state: Mutex::new(MultiGranularLockState {
                intention_shared: 0,
                intention_exclusive: 0,
                shared: 0,
                shared_and_intention_exclusive: 0,
                exclusive: false,
            }),
        }
    }

    /// Blocks until the lock can be acquired with the specified granularity.
    pub fn acquire(&self, granularity: Granularity) -> MultiGranularLockGuard {
        // Acquire the mutex and attempt to lock with the specified granularity.
        // If it does not succeed, park the thread and wait to be unparked by
        // a guard that sees that the granularity is acquirable.
        loop {
            {
                let _l = self.lock.lock().unwrap();
                match self.try_acquire_locked(granularity) {
                    Some(g) => return g,
                    None => {
                        self.waiting
                            .lock()
                            .unwrap()
                            .push((thread::current(), granularity));
                    }
                };
            }

            thread::park();
        }
    }

    /// Attempts to acquire the lock at the specified granularity without blocking.
    pub fn try_acquire(&self, granularity: Granularity) -> Option<MultiGranularLockGuard> {
        let _l = self.lock.lock().unwrap();
        self.try_acquire_locked(granularity)
    }

    fn try_acquire_locked(&self, granularity: Granularity) -> Option<MultiGranularLockGuard> {
        if !self.can_acquire(granularity) {
            return None;
        }

        let mut state = self.state.lock().unwrap();
        match granularity {
            Granularity::IntentionShared => (*state).intention_shared += 1,
            Granularity::IntentionExclusive => (*state).intention_exclusive += 1,
            Granularity::Shared => (*state).shared += 1,
            Granularity::SharedAndIntentionExclusive => {
                (*state).shared_and_intention_exclusive += 1
            }
            Granularity::Exclusive => (*state).exclusive = true,
        }

        Some(MultiGranularLockGuard {
            lock: &self,
            granularity,
        })
    }

    fn can_acquire(&self, granularity: Granularity) -> bool {
        let state = self.state.lock().unwrap();
        match granularity {
            Granularity::IntentionShared => !state.exclusive,
            Granularity::IntentionExclusive => {
                !(state.exclusive || state.shared_and_intention_exclusive > 0 || state.shared > 0)
            }
            Granularity::Shared => {
                !(state.exclusive
                    || state.shared_and_intention_exclusive > 0
                    || state.intention_exclusive > 0)
            }
            Granularity::SharedAndIntentionExclusive => {
                !(state.exclusive
                    || state.shared_and_intention_exclusive > 0
                    || state.shared > 0
                    || state.intention_exclusive > 0)
            }
            Granularity::Exclusive => {
                !(state.exclusive
                    || state.shared_and_intention_exclusive > 0
                    || state.shared > 0
                    || state.intention_exclusive > 0
                    || state.intention_shared > 0)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Condvar, Mutex};
    use std::thread;

    use super::{Granularity, MultiGranularLock};

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

    #[test]
    fn exclusive() {
        let l = MultiGranularLock::new();
        let g = l.try_acquire(Granularity::Exclusive);
        assert!(g.is_some());

        assert!(l.try_acquire(Granularity::Exclusive).is_none());
        assert!(
            l.try_acquire(Granularity::SharedAndIntentionExclusive)
                .is_none()
        );
        assert!(l.try_acquire(Granularity::Shared).is_none());
        assert!(l.try_acquire(Granularity::IntentionExclusive).is_none());
        assert!(l.try_acquire(Granularity::IntentionShared).is_none());
    }

    #[test]
    fn shared_and_intention_exclusive() {
        let l = MultiGranularLock::new();
        let g = l.try_acquire(Granularity::SharedAndIntentionExclusive);
        assert!(g.is_some());

        assert!(l.try_acquire(Granularity::Exclusive).is_none());
        assert!(
            l.try_acquire(Granularity::SharedAndIntentionExclusive)
                .is_none()
        );
        assert!(l.try_acquire(Granularity::Shared).is_none());
        assert!(l.try_acquire(Granularity::IntentionExclusive).is_none());
        assert!(l.try_acquire(Granularity::IntentionShared).is_some());
    }

    #[test]
    fn shared() {
        let l = MultiGranularLock::new();
        let g = l.try_acquire(Granularity::Shared);
        assert!(g.is_some());

        assert!(l.try_acquire(Granularity::Exclusive).is_none());
        assert!(
            l.try_acquire(Granularity::SharedAndIntentionExclusive)
                .is_none()
        );
        assert!(l.try_acquire(Granularity::Shared).is_some());
        assert!(l.try_acquire(Granularity::IntentionExclusive).is_none());
        assert!(l.try_acquire(Granularity::IntentionShared).is_some());
    }

    #[test]
    fn intention_exclusive() {
        let l = MultiGranularLock::new();
        let g = l.try_acquire(Granularity::IntentionExclusive);
        assert!(g.is_some());

        assert!(l.try_acquire(Granularity::Exclusive).is_none());
        assert!(
            l.try_acquire(Granularity::SharedAndIntentionExclusive)
                .is_none()
        );
        assert!(l.try_acquire(Granularity::Shared).is_none());
        assert!(l.try_acquire(Granularity::IntentionExclusive).is_some());
        assert!(l.try_acquire(Granularity::IntentionShared).is_some());
    }

    #[test]
    fn intention_shared() {
        let l = MultiGranularLock::new();
        let g = l.try_acquire(Granularity::IntentionShared);
        assert!(g.is_some());

        assert!(l.try_acquire(Granularity::Exclusive).is_none());
        assert!(
            l.try_acquire(Granularity::SharedAndIntentionExclusive)
                .is_some()
        );
        assert!(l.try_acquire(Granularity::Shared).is_some());
        assert!(l.try_acquire(Granularity::IntentionExclusive).is_some());
        assert!(l.try_acquire(Granularity::IntentionShared).is_some());
    }

    #[test]
    fn acquire() {
        let l = Arc::new(MultiGranularLock::new());

        let h;
        {
            let pair = Arc::new((Mutex::new(false), Condvar::new()));
            let pair2 = pair.clone();

            let l1 = l.clone();
            let g = l1.try_acquire(Granularity::Exclusive);
            assert!(g.is_some());

            let l2 = l.clone();
            h = thread::spawn(move || {
                assert!(l2.try_acquire(Granularity::Exclusive).is_none());

                {
                    let &(ref lock, ref cvar) = &*pair2;
                    let mut tried = lock.lock().unwrap();
                    *tried = true;
                    cvar.notify_one();
                }

                l2.acquire(Granularity::Exclusive);
            });

            let &(ref lock, ref cvar) = &*pair;
            let mut tried = lock.lock().unwrap();
            while !*tried {
                tried = cvar.wait(tried).unwrap();
            }
        }

        h.join().unwrap();
    }
}
