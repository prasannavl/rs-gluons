// Basically, swap each item with it's next, and 'bubble'
// the highest element up to the right, one by one.
//
// For i=0..n, run 0..(n-i); swap i to i+1
//
// - compares each item to every item that's not sorted to the right.
// - partials: sorted right partition (until n-i)
//
pub fn bubble_sort<T:PartialOrd>(slice: &mut [T]) {
    let len = slice.len();
    for i in 0..len {
        for j in 0..len-i-1 {
            if &slice[j] > &slice[j + 1]  {
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
pub fn insert_sort<T:PartialOrd>(slice: &mut [T]) {
    let len = slice.len();
    // i=0 can be skipped
    for i in 1..len {
        let mut j = i;
        while j > 0 && &slice[j] < &slice[j - 1] {
            slice.swap(j, j-1);
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
pub fn select_sort<T:PartialOrd>(slice: &mut [T]) {
    let len = slice.len();
    for i in 0..len {
        for j in i+1..len {
            if &slice[j] < &slice[i] {
                slice.swap(j, i);
            }
        }
    }
}