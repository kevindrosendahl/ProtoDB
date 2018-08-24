use std::sync::Mutex;
use std::thread;
use std::thread::Thread;

#[derive(Debug, Clone, Copy)]
pub enum Granularity {
    IntentionShared,
    IntentionExclusive,
    Shared,
    SharedAndIntentionExclusive,
    Exclusive,
}

#[derive(Debug)]
pub struct MultiGranularLock {
    lock: Mutex<()>,

    waiting: Mutex<Vec<(Thread, Granularity)>>,

    intention_shared: Mutex<usize>,
    intention_exclusive: Mutex<usize>,
    shared: Mutex<usize>,
    shared_and_intention_exclusive: Mutex<usize>,
    exclusive: Mutex<bool>,
}

#[derive(Debug)]
pub struct MultiGranularLockGuard<'a> {
    lock: &'a MultiGranularLock,
    granularity: Granularity,
}

impl<'a> Drop for MultiGranularLockGuard<'a> {
    #[inline]
    fn drop(&mut self) {
        let _lg = self.lock.lock.lock().unwrap();
        match self.granularity {
            Granularity::IntentionShared => *self.lock.intention_shared.lock().unwrap() -= 1,
            Granularity::IntentionExclusive => *self.lock.intention_exclusive.lock().unwrap() -= 1,
            Granularity::Shared => *self.lock.shared.lock().unwrap() -= 1,
            Granularity::SharedAndIntentionExclusive => {
                *self.lock.shared_and_intention_exclusive.lock().unwrap() -= 1
            }
            Granularity::Exclusive => *self.lock.exclusive.lock().unwrap() = false,
        }

        let idx = self.lock
            .waiting
            .lock()
            .unwrap()
            .iter()
            .enumerate()
            .find(|(i, (t, g))| self.lock.can_acquire(g.clone()))
            .map(|(i, _)| i);

        if let Some(i) = idx {
            let (t, _) = self.lock.waiting.lock().unwrap().remove(i);
            t.unpark();
        }
    }
}

impl MultiGranularLock {
    pub fn new() -> MultiGranularLock {
        MultiGranularLock {
            lock: Mutex::new(()),

            waiting: Mutex::new(Vec::new()),

            intention_shared: Mutex::new(0),
            intention_exclusive: Mutex::new(0),
            shared: Mutex::new(0),
            shared_and_intention_exclusive: Mutex::new(0),
            exclusive: Mutex::new(false),
        }
    }

    pub fn acquire(&self, granularity: Granularity) -> MultiGranularLockGuard {
        {
            let _lg = self.lock.lock().unwrap();
            let g = self.try_acquire_locked(granularity);
            if g.is_some() {
                return g.unwrap();
            }

            self.waiting
                .lock()
                .unwrap()
                .push((thread::current(), granularity));
        }

        thread::park();
        self.acquire(granularity)
    }

    pub fn try_acquire(&self, granularity: Granularity) -> Option<MultiGranularLockGuard> {
        let _lg = self.lock.lock().unwrap();
        self.try_acquire_locked(granularity)
    }

    fn try_acquire_locked(&self, granularity: Granularity) -> Option<MultiGranularLockGuard> {
        if !self.can_acquire(granularity) {
            return None;
        }

        match granularity {
            Granularity::IntentionShared => *self.intention_shared.lock().unwrap() += 1,
            Granularity::IntentionExclusive => *self.intention_exclusive.lock().unwrap() += 1,
            Granularity::Shared => *self.shared.lock().unwrap() += 1,
            Granularity::SharedAndIntentionExclusive => {
                *self.shared_and_intention_exclusive.lock().unwrap() += 1
            }
            Granularity::Exclusive => *self.exclusive.lock().unwrap() = true,
        }

        Some(MultiGranularLockGuard {
            lock: &self,
            granularity,
        })
    }

    fn can_acquire(&self, granularity: Granularity) -> bool {
        match granularity {
            Granularity::IntentionShared => !*self.exclusive.lock().unwrap(),
            Granularity::IntentionExclusive => {
                !(*self.exclusive.lock().unwrap()
                    || *self.shared_and_intention_exclusive.lock().unwrap() > 0
                    || *self.shared.lock().unwrap() > 0)
            }
            Granularity::Shared => {
                !(*self.exclusive.lock().unwrap()
                    || *self.shared_and_intention_exclusive.lock().unwrap() > 0
                    || *self.intention_exclusive.lock().unwrap() > 0)
            }
            Granularity::SharedAndIntentionExclusive => {
                !(*self.exclusive.lock().unwrap()
                    || *self.shared_and_intention_exclusive.lock().unwrap() > 0
                    || *self.shared.lock().unwrap() > 0
                    || *self.intention_exclusive.lock().unwrap() > 0)
            }
            Granularity::Exclusive => {
                !(*self.exclusive.lock().unwrap()
                    || *self.shared_and_intention_exclusive.lock().unwrap() > 0
                    || *self.shared.lock().unwrap() > 0
                    || *self.intention_exclusive.lock().unwrap() > 0
                    || *self.intention_shared.lock().unwrap() > 0)
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
        assert!(l.try_acquire(Granularity::IntentionExclusive).is_some());
        assert!(l.try_acquire(Granularity::IntentionShared).is_some());
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
        assert!(l.try_acquire(Granularity::IntentionExclusive).is_some());
        assert!(l.try_acquire(Granularity::IntentionShared).is_some());
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
