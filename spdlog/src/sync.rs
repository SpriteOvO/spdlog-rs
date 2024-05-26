#![allow(unused_imports)]

pub use std::sync::{
    Arc, Condvar, Mutex, MutexGuard, RwLock, RwLockReadGuard, RwLockWriteGuard, Weak,
};

pub use arc_swap::{ArcSwap, ArcSwapOption};
pub use once_cell::sync::{Lazy, OnceCell};
pub use spin::{
    Mutex as SpinMutex, MutexGuard as SpinMutexGuard, RwLock as SpinRwLock,
    RwLockReadGuard as SpinRwLockReadGuard, RwLockWriteGuard as SpinRwLockWriteGuard,
};

pub mod atomic {
    pub use std::sync::atomic::*;

    pub use atomic::Atomic;
}
pub use self::atomic::*;

pub trait MutexExtend<'a> {
    type LockReturn;

    #[must_use]
    fn lock_expect(&'a self) -> Self::LockReturn;
}

pub trait RwLockExtend<'a> {
    type ReadReturn;
    type WriteReturn;

    #[allow(dead_code)]
    #[must_use]
    fn read_expect(&'a self) -> Self::ReadReturn;

    #[must_use]
    fn write_expect(&'a self) -> Self::WriteReturn;
}

const LOCK_POISONED_MESSAGE: &str = "lock is poisoned";

impl<'a, T: ?Sized + 'a> MutexExtend<'a> for Mutex<T> {
    type LockReturn = MutexGuard<'a, T>;

    fn lock_expect(&'a self) -> Self::LockReturn {
        self.lock().expect(LOCK_POISONED_MESSAGE)
    }
}

impl<'a, T: ?Sized + 'a> RwLockExtend<'a> for RwLock<T> {
    type ReadReturn = RwLockReadGuard<'a, T>;
    type WriteReturn = RwLockWriteGuard<'a, T>;

    fn read_expect(&'a self) -> Self::ReadReturn {
        self.read().expect(LOCK_POISONED_MESSAGE)
    }

    fn write_expect(&'a self) -> Self::WriteReturn {
        self.write().expect(LOCK_POISONED_MESSAGE)
    }
}
