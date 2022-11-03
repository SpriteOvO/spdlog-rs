use std::thread::{self, JoinHandle};

use crossbeam::channel::{self as mpmc, Receiver, Sender};
use once_cell::sync::Lazy;

use crate::{
    sink::{OverflowPolicy, Task},
    sync::*,
    Error, Result,
};

/// A thread pool for processing operations asynchronously.
///
/// Currently only used in [`AsyncPoolSink`].
///
/// # Examples
///
/// ```
/// # use std::sync::Arc;
/// use spdlog::{sink::AsyncPoolSink, ThreadPool};
///
/// # fn main() -> Result<(), spdlog::Error> {
/// # let underlying_sink = spdlog::default_logger().sinks().first().unwrap().clone();
/// let thread_pool: Arc<ThreadPool> = Arc::new(ThreadPool::new()?);
/// let async_pool_sink: AsyncPoolSink = AsyncPoolSink::builder()
///     .sink(underlying_sink)
///     .thread_pool(thread_pool)
///     .build()?;
/// # Ok(()) }
/// ```
///
/// [`AsyncPoolSink`]: crate::sink::AsyncPoolSink
pub struct ThreadPool {
    threads: Vec<Option<JoinHandle<()>>>,
    sender: Option<Sender<Task>>,
}

/// The builder of [`ThreadPool`].
pub struct ThreadPoolBuilder {
    capacity: usize,
    threads: usize,
}

struct Worker {
    receiver: Receiver<Task>,
}

impl ThreadPool {
    /// Constructs a builder of `ThreadPool`.
    #[must_use]
    pub fn builder() -> ThreadPoolBuilder {
        ThreadPoolBuilder {
            capacity: 8192,
            threads: 1,
        }
    }

    /// Constructs a `ThreadPool` with default parameters.
    pub fn new() -> Result<Self> {
        Self::builder().build()
    }

    pub(super) fn assign_task(&self, task: Task, overflow_policy: OverflowPolicy) -> Result<()> {
        let sender = self.sender.as_ref().unwrap();

        match overflow_policy {
            OverflowPolicy::Block => sender.send(task).map_err(Error::from_crossbeam_send),
            OverflowPolicy::DropIncoming => sender
                .try_send(task)
                .map_err(Error::from_crossbeam_try_send),
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // drop our sender, threads will break the loop after receiving and processing
        // the remaining tasks
        self.sender.take();

        for thread in &mut self.threads {
            thread
                .take()
                .unwrap()
                .join()
                .expect("failed to join a thread from pool");
        }
    }
}

impl ThreadPoolBuilder {
    /// Specifies the capacity of the operation channel.
    ///
    /// This parameter is **optional**, and defaults to 8192 (The value may
    /// change in the future).
    ///
    /// When a new operation is incoming, but the channel is full, it will be
    /// handled by sink according to the [`OverflowPolicy`] that has been set.
    ///
    /// # Panics
    ///
    /// Panics if the value is zero.
    pub fn capacity(&mut self, capacity: usize) -> &mut Self {
        self.capacity = capacity;
        self
    }

    // The current Sinks are not beneficial with more than one thread, so the method
    // is not public.
    #[allow(dead_code)]
    fn threads(&mut self, threads: usize) -> &mut Self {
        self.threads = threads;
        self
    }

    /// Builds a [`ThreadPool`].
    pub fn build(&self) -> Result<ThreadPool> {
        if self.capacity < 1 {
            panic!("capacity of ThreadPool cannot be 0");
        }

        if self.threads < 1 {
            panic!("threads of ThreadPool cannot be 0");
        }

        let (sender, receiver) = mpmc::bounded(self.capacity);

        let mut threads = Vec::new();
        threads.resize_with(self.threads, || {
            let receiver = receiver.clone();
            Some(thread::spawn(move || Worker { receiver }.run()))
        });

        Ok(ThreadPool {
            threads,
            sender: Some(sender),
        })
    }
}

impl Worker {
    fn run(&self) {
        while let Ok(task) = self.receiver.recv() {
            task.exec();
        }
    }
}

#[must_use]
pub(crate) fn default_thread_pool() -> Arc<ThreadPool> {
    static POOL_WEAK: Lazy<Mutex<Weak<ThreadPool>>> = Lazy::new(|| Mutex::new(Weak::new()));

    let mut pool_weak = POOL_WEAK.lock_expect();

    match pool_weak.upgrade() {
        Some(pool) => pool,
        None => {
            let pool = Arc::new(ThreadPool::builder().build().unwrap());
            *pool_weak = Arc::downgrade(&pool);
            pool
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn panic_capacity_0() {
        ThreadPool::builder().capacity(0).build().unwrap();
    }

    #[test]
    #[should_panic]
    fn panic_thread_0() {
        ThreadPool::builder().threads(0).build().unwrap();
    }
}
