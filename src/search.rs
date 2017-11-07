pub fn binary_search<T: PartialOrd>(val: &T, slice: &[T]) -> Option<usize> {
    let len = slice.len();
    let mut start = 0usize;
    let mut end = len;
    let mut middle;
    while start < end {
        middle = start + (end - start) / 2;
        let current = &slice[middle];
        if val > current {
            start = middle + 1;
        } else if val < current {
            end = middle;
        } else {
            return Some(middle);
        }
    }
    return None;
}