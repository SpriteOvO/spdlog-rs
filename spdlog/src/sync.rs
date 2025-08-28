#![allow(unused_imports)]

pub use std::sync::{
    Arc, Condvar, Mutex, MutexGuard, RwLock, RwLockReadGuard, RwLockWriteGuard, Weak,
};

pub use arc_swap::{ArcSwap, ArcSwapOption};
pub use once_cell::sync::{Lazy, OnceCell};

pub mod atomic {
    pub use std::sync::atomic::*;

    pub use atomic::Atomic;
}
pub use self::atomic::*;

pub trait MutexExtend<T: ?Sized> {
    fn lock_expect(&self) -> MutexGuard<'_, T>;
    fn get_mut_expect(&mut self) -> &mut T;
}

pub trait RwLockExtend<T: ?Sized> {
    fn read_expect(&self) -> RwLockReadGuard<'_, T>;
    fn write_expect(&self) -> RwLockWriteGuard<'_, T>;
}

const LOCK_POISONED_MESSAGE: &str = "lock is poisoned";

impl<T: ?Sized> MutexExtend<T> for Mutex<T> {
    fn lock_expect(&self) -> MutexGuard<'_, T> {
        self.lock().expect(LOCK_POISONED_MESSAGE)
    }
    fn get_mut_expect(&mut self) -> &mut T {
        self.get_mut().expect(LOCK_POISONED_MESSAGE)
    }
}

impl<T: ?Sized> RwLockExtend<T> for RwLock<T> {
    fn read_expect(&self) -> RwLockReadGuard<'_, T> {
        self.read().expect(LOCK_POISONED_MESSAGE)
    }
    fn write_expect(&self) -> RwLockWriteGuard<'_, T> {
        self.write().expect(LOCK_POISONED_MESSAGE)
    }
}
