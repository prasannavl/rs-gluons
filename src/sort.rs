use std::ptr::copy_nonoverlapping;
use std::mem;
use std::cmp::Ordering;
use std::cmp::Ordering::*;

// Basically, swap each item with it's next, and 'bubble'
// the highest element up to the right, one by one.
//
// For i=0..n, run 0..(n-i); swap i to i+1
//
// - compares each item to every item that's not sorted to the right.
// - partials: sorted right partition (until n-i)
//
pub fn bubble_sort<T, F>(slice: &mut [T], mut compare: F)
where
    F: FnMut(&T, &T) -> Ordering,
{
    let len = slice.len();
    for i in 0..len {
        for j in 0..len - i - 1 {
            // if &slice[j] > &slice[j + 1]
            if compare(&slice[j], &slice[j + 1]) == Greater {
                slice.swap(j, j + 1);
            }
        }
    }
}

// Basically, swap each item with it's previous until sorted.
//
// For i=0..n, start j = i; Compare j to j-1, and swap
// repeatedly until element-j is sorted.
//
// - performs repeated compares only if it's not sorted.
// - no partials
// - partial extent: in a 0..n pass, items until n is sorted to the left.
//
pub fn insert_sort<T, F>(slice: &mut [T], mut compare: F)
where
    F: FnMut(&T, &T) -> Ordering,
{
    let len = slice.len();
    // i=0 can be skipped
    for i in 1..len {
        let mut j = i;
        // while j > 0 && &slice[j] < &slice[j - 1]
        while j > 0 && compare(&slice[j], &slice[j - 1]) == Less {
            slice.swap(j, j - 1);
            j -= 1;
        }
    }
}

// Basically, swap each item after i, directly with i, until sorted.
//
// For i=0..n, run i=i+1..n;
// Compare j to i and swap once.
//
// - compares all elements after item to the item, and swap.
// - partials: sorted left partition until i
//
pub fn select_sort<T, F>(slice: &mut [T], mut compare: F)
where
    F: FnMut(&T, &T) -> Ordering,
{
    let len = slice.len();
    for i in 0..len {
        for j in i + 1..len {
            // if &slice[j] < &slice[i]
            if compare(&slice[j], &slice[i]) == Less {
                slice.swap(j, i);
            }
        }
    }
}

fn should_sort<T>(slice: &[T]) -> bool {
    // Sorting has no meaningful behavior on zero-sized types.
    if ::std::mem::size_of::<T>() == 0 {
        return false;
    }
    let len = slice.len();
    return len > 1;
}

fn create_slice<T>(len: usize) -> Box<[T]> {
    let mut v = Vec::<T>::with_capacity(len);
    unsafe {
        v.set_len(len);
    }
    return v.into_boxed_slice();
}

pub fn merge_sort<T, F>(slice: &mut [T], compare: F)
where
    F: FnMut(&T, &T) -> Ordering,
{
    // Slices of up to this length get sorted using insertion sort.
    const DEFAULT_INSERT_SORT_MAX_THRESHOLD: usize = 20;
    merge_sort_with_insert(slice, compare, DEFAULT_INSERT_SORT_MAX_THRESHOLD);
}

pub fn merge_sort_pure<T, F>(slice: &mut [T], compare: F)
where
    F: FnMut(&T, &T) -> Ordering,
{
    merge_sort_with_insert(slice, compare, 0);
}

pub fn merge_sort_with_insert<T, F>(slice: &mut [T], mut compare: F, insert_sort_max_threshold: usize)
where
    F: FnMut(&T, &T) -> Ordering,
{
    if !should_sort(&slice) {
        return;
    }

    let len = slice.len();

    if len <= insert_sort_max_threshold {
        insert_sort(slice, compare);
        return;
    }

    let mut temp = create_slice(len);
    merge_sort_with_buf(slice, temp.as_mut(), &mut compare, insert_sort_max_threshold);
}

pub fn merge_sort_with_buf<T, F>(slice: &mut [T], buf: &mut [T], compare: &mut F, insert_sort_max_threshold: usize)
where
    F: FnMut(&T, &T) -> Ordering,
{
    if !should_sort(&slice) {
        return;
    }
    
    let len = slice.len();

    if len <= insert_sort_max_threshold {
        insert_sort(slice, compare);
        return;
    }

    let middle = len / 2;

    merge_sort_with_buf(&mut slice[..middle], &mut buf[..middle], compare, insert_sort_max_threshold);
    merge_sort_with_buf(&mut slice[middle..], &mut buf[middle..], compare, insert_sort_max_threshold);
    merge_sorted_halves(slice, buf, compare);
}

// Merges slice of n from 0..n/2 to n/2..n, using buf as scratch space.
// buf has to be atleast the same size as slice.
fn merge_sorted_halves<T, F>(slice: &mut [T], buf: &mut [T], compare: &mut F)
where
    F: FnMut(&T, &T) -> Ordering,
{
    if !should_sort(&slice) {
        return;
    }
    let len = slice.len();
    let mut left = 0 as usize;
    let left_end = len / 2;
    let mut right = left_end + 1;
    let right_end = len - 1;

    let mut current = left;

    while left <= left_end && right <= right_end {
        // The safety of the unchecked ops are verified by the loop above.
        unsafe {
            // if slice.get_unchecked(left) < slice.get_unchecked(right)
            if compare(slice.get_unchecked(left), slice.get_unchecked(right)) == Less {
                mem::swap(
                    buf.get_unchecked_mut(current),
                    slice.get_unchecked_mut(left),
                );
                left += 1;
            } else {
                mem::swap(
                    buf.get_unchecked_mut(current),
                    slice.get_unchecked_mut(right),
                );
                right += 1;
            }
        }
        current += 1;
    }

    let left_rem_ptr;
    let right_rem_ptr;
    let buf_rem_ptr;

    unsafe {
        // CAUTION: Careful with the safety if tweaking any of this.
        // Their safety is inter-linked. One or more of left, right and current
        // pointers could be len + 1, but in that case safety
        // is ensured by correctly offsetting the `count` in `copy_nonoverlapping` methods.

        left_rem_ptr = slice.as_ptr().offset(left as isize);
        right_rem_ptr = slice.as_ptr().offset(right as isize);
        buf_rem_ptr = buf.as_mut_ptr().offset(current as isize);

        // Copy left overs
        // Note: add 1 to left or right to start from the next item.
        copy_nonoverlapping(left_rem_ptr, buf_rem_ptr, left_end + 1 - left);
        copy_nonoverlapping(right_rem_ptr, buf_rem_ptr, right_end + 1 - right);

        // Copy the entire buf back into slice
        copy_nonoverlapping(buf.as_ptr(), slice.as_mut_ptr(), len);
    }
}