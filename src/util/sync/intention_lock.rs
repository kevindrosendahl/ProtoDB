use std::sync::Mutex;

#[derive(Debug)]
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

    intention_shared: Mutex<usize>,
    intention_exclusive: Mutex<usize>,
    shared: Mutex<usize>,
    shared_and_intention_exclusive: Mutex<usize>,
    exclusive:  Mutex<bool>,
}

#[derive(Debug)]
pub struct MultiGranularLockGuard<'a> {
    lock: &'a MultiGranularLock,
    granularity: Granularity,
}

impl<'a> Drop for MultiGranularLockGuard<'a> {
    #[inline]
    fn drop(&mut self) {
        let _ = self.lock.lock.lock().unwrap();
        match self.granularity {
            Granularity::IntentionShared => *self.lock.intention_shared.lock().unwrap() -= 1,
            Granularity::IntentionExclusive => *self.lock.intention_exclusive.lock().unwrap() -= 1,
            Granularity::Shared => *self.lock.shared.lock().unwrap() -= 1,
            Granularity::SharedAndIntentionExclusive => *self.lock.shared_and_intention_exclusive.lock().unwrap() -= 1,
            Granularity::Exclusive => *self.lock.exclusive.lock().unwrap() = false,
        }
    }
}

impl MultiGranularLock {
    pub fn new() -> MultiGranularLock {
        MultiGranularLock {
            lock: Mutex::new(()),

            intention_shared: Mutex::new(0),
            intention_exclusive: Mutex::new(0),
            shared: Mutex::new(0),
            shared_and_intention_exclusive: Mutex::new(0),
            exclusive: Mutex::new(false),
        }
    }

    pub fn try_acquire(&self, granularity: Granularity) -> Option<MultiGranularLockGuard> {
        let _ = self.lock.lock().unwrap();
        match granularity {
            Granularity::IntentionShared => {
                if *self.exclusive.lock().unwrap() {
                    None
                } else {
                    *self.intention_shared.lock().unwrap() += 1;
                    Some(MultiGranularLockGuard {
                        lock: self,
                        granularity: Granularity::IntentionShared,
                    })
                }
            }
            Granularity::IntentionExclusive => {
               if *self.exclusive.lock().unwrap() ||
                   *self.shared_and_intention_exclusive.lock().unwrap() > 0 ||
                   *self.shared.lock().unwrap() > 0 {
                   None
               } else {
                   *self.intention_exclusive.lock().unwrap() += 1;
                   Some(MultiGranularLockGuard {
                       lock: self,
                       granularity: Granularity::IntentionExclusive,
                   })
               }
            }
            Granularity::Shared => {
                if *self.exclusive.lock().unwrap() ||
                    *self.shared_and_intention_exclusive.lock().unwrap() > 0 ||
                    *self.intention_exclusive.lock().unwrap() > 0 {
                    None
                } else {
                    *self.shared.lock().unwrap() += 1;
                    Some(MultiGranularLockGuard {
                        lock: self,
                        granularity: Granularity::Shared,
                    })
                }
            }
            Granularity::SharedAndIntentionExclusive => {
                if *self.exclusive.lock().unwrap() ||
                    *self.shared_and_intention_exclusive.lock().unwrap() > 0 ||
                    *self.shared.lock().unwrap() > 0 ||
                    *self.intention_exclusive.lock().unwrap() > 0 {
                    None
                } else {
                    *self.shared_and_intention_exclusive.lock().unwrap() += 1;
                    Some(MultiGranularLockGuard {
                        lock: self,
                        granularity: Granularity::SharedAndIntentionExclusive,
                    })
                }
            }
            Granularity::Exclusive => {
                if *self.exclusive.lock().unwrap() ||
                    *self.shared_and_intention_exclusive.lock().unwrap() > 0 ||
                    *self.shared.lock().unwrap() > 0 ||
                    *self.intention_exclusive.lock().unwrap() > 0 ||
                    *self.intention_shared.lock().unwrap() > 0 {
                    None
                } else {
                    *self.exclusive.lock().unwrap() = true;
                    Some(MultiGranularLockGuard {
                        lock: self,
                        granularity: Granularity::Exclusive,
                    })
                }
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{MultiGranularLock, Granularity};

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
        assert!(l.try_acquire(Granularity::SharedAndIntentionExclusive).is_none());
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
        assert!(l.try_acquire(Granularity::SharedAndIntentionExclusive).is_none());
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
        assert!(l.try_acquire(Granularity::SharedAndIntentionExclusive).is_none());
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
        assert!(l.try_acquire(Granularity::SharedAndIntentionExclusive).is_none());
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
        assert!(l.try_acquire(Granularity::SharedAndIntentionExclusive).is_some());
        assert!(l.try_acquire(Granularity::Shared).is_some());
        assert!(l.try_acquire(Granularity::IntentionExclusive).is_some());
        assert!(l.try_acquire(Granularity::IntentionExclusive).is_some());
        assert!(l.try_acquire(Granularity::IntentionShared).is_some());
        assert!(l.try_acquire(Granularity::IntentionShared).is_some());
    }
}