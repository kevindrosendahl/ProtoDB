use std::cell::RefCell;
use std::sync::{Arc, Mutex};

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

    intention_shared: RefCell<usize>,
    intention_exclusive: RefCell<usize>,
    shared: RefCell<usize>,
    shared_and_intention_exclusive: RefCell<usize>,
    exclusive:  RefCell<bool>,
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
            Granularity::IntentionShared => *self.lock.intention_shared.borrow_mut() -= 1,
            Granularity::IntentionExclusive => *self.lock.intention_exclusive.borrow_mut() -= 1,
            Granularity::Shared => *self.lock.shared.borrow_mut() -= 1,
            Granularity::SharedAndIntentionExclusive => *self.lock.shared_and_intention_exclusive.borrow_mut() -= 1,
            Granularity::Exclusive => *self.lock.exclusive.borrow_mut() = false,
        }
    }
}

impl MultiGranularLock {
    pub fn new() -> MultiGranularLock {
        MultiGranularLock {
            lock: Mutex::new(()),

            intention_shared: RefCell::new(0),
            intention_exclusive: RefCell::new(0),
            shared: RefCell::new(0),
            shared_and_intention_exclusive: RefCell::new(0),
            exclusive: RefCell::new(false),
        }
    }

    pub fn try_acquire(&self, granularity: Granularity) -> Option<MultiGranularLockGuard> {
        let _ = self.lock.lock().unwrap();
        match granularity {
            Granularity::IntentionShared => {
                if *self.exclusive.borrow() {
                    None
                } else {
                    *self.intention_shared.borrow_mut() += 1;
                    Some(MultiGranularLockGuard {
                        lock: self,
                        granularity: Granularity::IntentionShared,
                    })
                }
            }
            Granularity::IntentionExclusive => {
               if *self.exclusive.borrow() ||
                   *self.shared_and_intention_exclusive.borrow() > 0 ||
                   *self.shared.borrow() > 0 {
                   None
               } else {
                   *self.intention_exclusive.borrow_mut() += 1;
                   Some(MultiGranularLockGuard {
                       lock: self,
                       granularity: Granularity::IntentionExclusive,
                   })
               }
            }
            Granularity::Shared => {
                if *self.exclusive.borrow() ||
                    *self.shared_and_intention_exclusive.borrow() > 0 ||
                    *self.intention_exclusive.borrow() > 0 {
                    None
                } else {
                    *self.shared.borrow_mut() += 1;
                    Some(MultiGranularLockGuard {
                        lock: self,
                        granularity: Granularity::Shared,
                    })
                }
            }
            Granularity::SharedAndIntentionExclusive => {
                if *self.exclusive.borrow() ||
                    *self.shared_and_intention_exclusive.borrow() > 0 ||
                    *self.shared.borrow() > 0 ||
                    *self.intention_exclusive.borrow() > 0 {
                    None
                } else {
                    *self.shared_and_intention_exclusive.borrow_mut() += 1;
                    Some(MultiGranularLockGuard {
                        lock: self,
                        granularity: Granularity::SharedAndIntentionExclusive,
                    })
                }
            }
            Granularity::Exclusive => {
                if *self.exclusive.borrow() ||
                    *self.shared_and_intention_exclusive.borrow() > 0 ||
                    *self.shared.borrow() > 0 ||
                    *self.intention_exclusive.borrow() > 0 ||
                    *self.intention_shared.borrow() > 0 {
                    None
                } else {
                    *self.exclusive.borrow_mut() = true;
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
    fn exclusive() {
        let l = MultiGranularLock::new();
        {
            let g = l.try_acquire(Granularity::Exclusive);
            assert!(g.is_some());
            assert!(l.try_acquire(Granularity::Exclusive).is_none());
        }
        assert!(l.try_acquire(Granularity::Exclusive).is_some());
    }
}