/// Returns the index of the partition point according to the given predicate
/// (the index of the first element of the second partition).
/// The deque is assumed to be partitioned according to the given predicate.
/// This means that all elements for which the predicate returns true are
/// at the start of the deque and all elements for which the predicate
/// returns false are at the end.
///
/// This implementation differs from analogous function
/// [`std::collections::VecDeque::partition_point`]. Instead of binary search,
/// it uses linear search, which in the scope of this crate may (or may not) be beneficial in
/// terms of speed.
/// ```ignore
/// use std::collections::VecDeque;
///
/// let deque: VecDeque<_> = [1, 2, 3, 3, 5, 6, 7].into();
///
/// let mut count_steps = 0;
///
/// let i = partition_point_linear(deque, 0, |&x| {count_steps+=1; x < 5});
/// assert_eq!(count_steps, 5);
/// assert_eq!(i, 4);
/// assert!(deque.iter().take(i).all(|&x| x < 5));
/// assert!(deque.iter().skip(i).all(|&x| !(x < 5)));
///
/// ```
pub fn partition_point_linear<T, P: FnMut(&T) -> bool>(
    deque: &std::collections::VecDeque<T>,
    start: usize,
    mut pred: P,
) -> usize {
    if deque.is_empty() {
        return 0;
    } else {
        let mut i = start.min(deque.len() - 1);
        if pred(deque.get(i).unwrap()) {
            i += 1;
            while let Some(elem) = deque.get(i)
                && pred(elem)
            {
                i += 1;
            }
        } else {
            while i > 0
                && let Some(elem) = deque.get(i - 1)
                && !pred(elem)
            {
                i -= 1;
            }
        }
        return i;
    }
}



#[cfg(test)]
mod test_partition_point_linear {
    use super::partition_point_linear;
    use std::collections::VecDeque;

    #[test]
    fn test1() {
        let deque: VecDeque<_> = [1, 2, 3, 3, 5, 6, 7].into();
        let i = partition_point_linear(&deque, 0, |&x| x < 5);

        assert_eq!(i, 4);
        assert!(deque.iter().take(i).all(|&x| x < 5));
        assert!(deque.iter().skip(i).all(|&x| !(x < 5)));
    }
    #[test]
    fn test2() {
        let deque: VecDeque<_> = [1, 2, 3, 3, 5, 6, 7].into();
        let i = partition_point_linear(&deque, 4, |&x| x < 10);
        assert_eq!(i, 7);
    }

    #[test]
    fn test3() {
        let deque: VecDeque<_> = [1, 2, 3, 3, 5, 6, 7].into();
        let i = partition_point_linear(&deque, 4, |&x| x < -4);
        assert_eq!(i, 0);
    }

    #[test]
    fn count_1() {
        let deque: VecDeque<_> = [1, 2, 3, 3, 5, 6, 7].into();
        let mut count_steps = 0;
        let i = partition_point_linear(&deque, 0, |&x| {
            count_steps += 1;
            x < 5
        });
        assert_eq!(count_steps, 5);
        assert_eq!(i, 4);
    }
    #[test]
    fn count_2() {
        let deque: VecDeque<_> = [1, 2, 3, 3, 5, 6, 7].into();
        let mut count_steps = 0;
        let i = partition_point_linear(&deque, 2, |&x| {
            count_steps += 1;
            x < 5
        });
        assert_eq!(count_steps, 3);
        assert_eq!(i, 4);
    }
    #[test]
    fn count_3() {
        let deque: VecDeque<_> = [1, 2, 3, 3, 5, 6, 7].into();
        let mut count_steps = 0;
        let i = partition_point_linear(&deque, 6, |&x| {
            count_steps += 1;
            x < 5
        });
        assert_eq!(count_steps, 4);
        assert_eq!(i, 4);
    }

    #[test]
    fn count_4() {
        let deque: VecDeque<_> = [1, 2, 3, 3, 5, 6, 7].into();
        let mut count_steps = 0;
        let i = partition_point_linear(&deque, 666, |&x| {
            count_steps += 1;
            x < 5
        });
        assert_eq!(count_steps, 4);
        assert_eq!(i, 4);
    }
}
