use std::num::NonZeroUsize;
use std::thread::{self, JoinHandle};

use crossbeam::channel::{self as mpmc, Receiver, Sender};
use once_cell::sync::Lazy;

use crate::{
    error::Error,
    sink::{OverflowPolicy, Task},
    sync::*,
    Result,
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
/// let thread_pool = Arc::new(ThreadPool::new()?);
/// let async_pool_sink = AsyncPoolSink::builder()
///     .sink(underlying_sink)
///     .thread_pool(thread_pool)
///     .build()?;
/// # Ok(()) }
/// ```
///
/// [`AsyncPoolSink`]: crate::sink::AsyncPoolSink
pub struct ThreadPool(ArcSwapOption<ThreadPoolInner>);

struct ThreadPoolInner {
    threads: Vec<Option<JoinHandle<()>>>,
    sender: Option<Sender<Task>>,
}

type Callback = Arc<dyn Fn() + Send + Sync + 'static>;

#[allow(missing_docs)]
pub struct ThreadPoolBuilder {
    capacity: NonZeroUsize,
    threads: NonZeroUsize,
    on_thread_spawn: Option<Callback>,
    on_thread_finish: Option<Callback>,
}

struct Worker {
    receiver: Receiver<Task>,
}

impl ThreadPool {
    /// Gets a builder of `ThreadPool` with default parameters:
    ///
    /// | Parameter          | Default Value                     |
    /// |--------------------|-----------------------------------|
    /// | [capacity]         | `8192` (may change in the future) |
    /// | [on_thread_spawn]  | `None`                            |
    /// | [on_thread_finish] | `None`                            |
    ///
    /// [capacity]: ThreadPoolBuilder::capacity
    /// [on_thread_spawn]: ThreadPoolBuilder::on_thread_spawn
    /// [on_thread_finish]: ThreadPoolBuilder::on_thread_finish
    #[must_use]
    pub fn builder() -> ThreadPoolBuilder {
        ThreadPoolBuilder {
            capacity: NonZeroUsize::new(8192).unwrap(),
            threads: NonZeroUsize::new(1).unwrap(),
            on_thread_spawn: None,
            on_thread_finish: None,
        }
    }

    /// Constructs a `ThreadPool` with default parameters (see documentation of
    /// [`ThreadPool::builder`]).
    pub fn new() -> Result<Self> {
        Self::builder().build()
    }

    pub(super) fn assign_task(&self, task: Task, overflow_policy: OverflowPolicy) -> Result<()> {
        let inner = self.0.load();
        let sender = inner.as_ref().unwrap().sender.as_ref().unwrap();

        match overflow_policy {
            OverflowPolicy::Block => sender.send(task).map_err(Error::from_crossbeam_send),
            OverflowPolicy::DropIncoming => sender
                .try_send(task)
                .map_err(Error::from_crossbeam_try_send),
        }
    }

    pub(super) fn destroy(&self) {
        if let Some(mut inner) = self.0.swap(None) {
            // Or use `Arc::into_inner`, but it requires us to bump MSRV.
            let inner = Arc::get_mut(&mut inner).unwrap();

            // drop our sender, threads will break the loop after receiving and processing
            // the remaining tasks
            inner.sender.take();

            for thread in &mut inner.threads {
                if let Some(thread) = thread.take() {
                    thread.join().expect("failed to join a thread from pool");
                }
            }
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        self.destroy();
    }
}

impl ThreadPoolBuilder {
    /// Specifies the capacity of the operation channel.
    ///
    /// This parameter is **optional**.
    ///
    /// When a new operation is incoming, but the channel is full, it will be
    /// handled by sink according to the [`OverflowPolicy`] that has been set.
    #[must_use]
    pub fn capacity(&mut self, capacity: NonZeroUsize) -> &mut Self {
        self.capacity = capacity;
        self
    }

    // The current Sinks are not beneficial with more than one thread, so the method
    // is not public.
    #[must_use]
    #[allow(dead_code)]
    fn threads(&mut self, threads: NonZeroUsize) -> &mut Self {
        self.threads = threads;
        self
    }

    /// Provide a function that will be called on each thread of the thread pool
    /// immediately after it is spawned. This can, for example, be used to set
    /// core affinity for each thread.
    #[must_use]
    pub fn on_thread_spawn<F>(&mut self, f: F) -> &mut Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.on_thread_spawn = Some(Arc::new(f));
        self
    }

    /// Provide a function that will be called on each thread of the thread pool
    /// just before the thread finishes.
    #[must_use]
    pub fn on_thread_finish<F>(&mut self, f: F) -> &mut Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.on_thread_finish = Some(Arc::new(f));
        self
    }

    /// Builds a [`ThreadPool`].
    pub fn build(&self) -> Result<ThreadPool> {
        let (sender, receiver) = mpmc::bounded(self.capacity.get());

        let mut threads = Vec::new();
        threads.resize_with(self.threads.get(), || {
            let receiver = receiver.clone();
            let on_thread_spawn = self.on_thread_spawn.clone();
            let on_thread_finish = self.on_thread_finish.clone();

            Some(thread::spawn(move || {
                if let Some(f) = on_thread_spawn {
                    f();
                }

                Worker { receiver }.run();

                if let Some(f) = on_thread_finish {
                    f();
                }
            }))
        });

        Ok(ThreadPool(ArcSwapOption::new(Some(Arc::new(
            ThreadPoolInner {
                threads,
                sender: Some(sender),
            },
        )))))
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
