mod async_pool_sink;

pub use async_pool_sink::*;

/// Overflow policy for [asynchronous sinks].
///
/// When the channel is full, an incoming operation is handled according to the
/// specified policy.
///
/// [asynchronous sinks]: index.html#asynchronous-combined-sink
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum OverflowPolicy {
    /// Blocks until the channel is not full.
    Block,
    /// Drops the incoming operation.
    DropIncoming,
    // DropOldest, // waiting for https://github.com/crossbeam-rs/crossbeam/issues/400
}
