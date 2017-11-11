use std::cmp::Ordering;
use std::cmp::Ordering::*;

pub fn binary_search<T, F>(val: &T, slice: &[T], mut compare: F) -> Option<usize>
where
    F: FnMut(&T, &T) -> Ordering,
{
    let len = slice.len();
    let mut start = 0usize;
    let mut end = len;
    let mut middle;
    while start < end {
        middle = start + (end - start) / 2;
        let current = &slice[middle];
        match compare(val, current) {
            Greater => start = middle + 1,
            Less => end = middle,
            Equal => return Some(middle),
        }
    }
    return None;
}
