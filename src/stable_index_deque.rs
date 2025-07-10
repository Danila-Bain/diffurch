use std::{
    alloc::{Allocator, Global},
    collections::VecDeque,
    ops::{Deref, DerefMut},
};

/// A limited `VecDeque` wrapper that keeps the indexes pointing to the same elements during
/// `pop_front` and `push_front` operations.
///
/// ```
/// use diffurch::stable_index_deque::StableIndexVecDeque;
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
pub struct StableIndexVecDeque<T, A: Allocator = Global> {
    pub offset: usize,
    pub deque: VecDeque<T, A>,
}

impl<T, A: Allocator> Deref for StableIndexVecDeque<T, A> {
    type Target = VecDeque<T, A>;
    fn deref(&self) -> &Self::Target {
        &self.deque
    }
}
impl<T, A: Allocator> DerefMut for StableIndexVecDeque<T, A> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deque
    }
}

impl<T> StableIndexVecDeque<T> {
    pub const fn new() -> Self {
        Self {
            offset: 0,
            deque: VecDeque::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            offset: 0,
            deque: VecDeque::with_capacity(capacity),
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.deque.get(index - self.offset)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.deque.get_mut(index - self.offset)
    }

    pub fn swap(&mut self, i: usize, j: usize) {
        self.deque.swap(i - self.offset, j - self.offset)
    }

    pub fn pop_front(&mut self) -> Option<T> {
        let ret = self.deque.pop_front();
        if ret.is_some() {
            self.offset += 1;
        }
        ret
    }

    pub fn push_front(&mut self, value: T) {
        self.deque.push_front(value);
        self.offset -= 1;
    }

    pub fn insert(&mut self, index: usize, value: T) {
        self.deque.insert(index - self.offset, value)
    }

    pub fn remove(&mut self, index: usize, value: T) {
        self.deque.insert(index - self.offset, value)
    }

    pub fn front_idx(&self) -> usize {
        self.offset
    }

    pub fn back_idx(&self) -> usize {
        self.offset + self.deque.len()
    }
}

impl<T, A: Allocator> core::ops::Index<usize> for StableIndexVecDeque<T, A> {
    type Output = T;

    #[inline]

    fn index(&self, index: usize) -> &T {
        self.get(index - self.offset).expect("Out of bounds access")
    }
}
