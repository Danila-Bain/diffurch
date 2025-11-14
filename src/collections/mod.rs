//! Custom collections that are used in a crate

pub mod hlists;

/// Module for [VecDeque] wrapper [StableIndexVecDeque] that maintains indexes when popping from or
/// pushing to its front.
pub mod stable_index_deque;

