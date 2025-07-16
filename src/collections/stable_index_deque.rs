use std::{
    collections::VecDeque,
    fmt::Debug,
    ops::{Deref, DerefMut},
};

/// A limited `VecDeque` wrapper that keeps the indexes pointing to the same elements during
/// `pop_front` and `push_front` operations.
///
/// ```
/// use diffurch::collections::stable_index_deque::StableIndexVecDeque;
///
/// let mut q = StableIndexVecDeque::new();
///
/// q.push_back(1);
/// q.push_back(2);
/// q.push_back(3);
///
/// assert_eq!(q[0], 1);
/// assert_eq!(q[1], 2);
/// assert_eq!(q[2], 3);
///
/// assert_eq!(q.pop_front(), Some(1));
/// assert_eq!(q[1], 2);
/// assert_eq!(q[2], 3);
///
/// ```
#[derive(Debug, Clone)]
pub struct StableIndexVecDeque<T> {
    offset: usize,
    deque: VecDeque<T>,
}

impl<T> Deref for StableIndexVecDeque<T> {
    type Target = VecDeque<T>;
    fn deref(&self) -> &Self::Target {
        &self.deque
    }
}
impl<T> DerefMut for StableIndexVecDeque<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deque
    }
}

impl<T> StableIndexVecDeque<T> {
    /// Creates a new [StableIndexVecDeque] initializing deque by [VecDeque::new]
    pub const fn new() -> Self {
        Self {
            offset: 0,
            deque: VecDeque::new(),
        }
    }

    /// Creates a new [StableIndexVecDeque] initializing deque by [VecDeque::with_capacity]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            offset: 0,
            deque: VecDeque::with_capacity(capacity),
        }
    }

    /// Stable index version of [VecDeque::get]
    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.offset {
            self.deque.get(index - self.offset)
        } else {
            None
        }
    }

    /// Stable index version of [VecDeque::get_mut]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.deque.get_mut(index - self.offset)
    }

    /// Stable index version of [VecDeque::swap]
    pub fn swap(&mut self, i: usize, j: usize) {
        self.deque.swap(i - self.offset, j - self.offset)
    }

    /// Stable index version of [VecDeque::pop_front]
    pub fn pop_front(&mut self) -> Option<T> {
        let ret = self.deque.pop_front();
        if ret.is_some() {
            self.offset += 1;
        }
        ret
    }

    /// Stable index version of [VecDeque::push_front]
    pub fn push_front(&mut self, value: T) {
        self.deque.push_front(value);
        self.offset -= 1;
    }

    /// Stable index version of [VecDeque::insert]
    pub fn insert(&mut self, index: usize, value: T) {
        self.deque.insert(index - self.offset, value)
    }

    /// Stable index version of [VecDeque::remove]
    pub fn remove(&mut self, index: usize, value: T) {
        self.deque.insert(index - self.offset, value)
    }

    /// Get the index of the front of the deque.
    pub fn front_idx(&self) -> usize {
        self.offset
    }

    /// Get the index of the (non-inclusive) end of the deque.
    pub fn back_idx(&self) -> usize {
        self.offset + self.deque.len()
    }
}

impl<T, const N: usize> From<[T; N]> for StableIndexVecDeque<T> {
    fn from(arr: [T; N]) -> Self {
        Self {
            offset: 0,
            deque: VecDeque::from(arr),
        }
    }
}

impl<T> core::ops::Index<usize> for StableIndexVecDeque<T> {
    type Output = T;

    #[inline]

    fn index(&self, index: usize) -> &T {
        StableIndexVecDeque::get(&self, index).expect("Out of bounds access")
    }
}
