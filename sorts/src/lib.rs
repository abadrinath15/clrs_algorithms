/// Insertion sort as specified in CLRS. A slight pain point for me was to adjust the boundaries;
/// CLRS actually uses 1.. indexing, but uses a breaking condition that the loop variable goes to 0
/// and then inserts a value at index + 1. Rust uses usize for indexing, which is 0 bound... That was
/// irritating. As such instead of using a while loop with two breaking condtions, I used a loop expression
/// and checked the conditions manually.
///
/// # Examples
/// ```
/// let y = &mut vec![5, 2, 4, 6, 1, 3];
/// let answer = &mut vec![1, 2, 3, 4, 5, 6];
/// assert_eq!(answer, sorts::insertion_sort(y));
/// ```
pub fn insertion_sort<T>(a: &mut Vec<T>) -> &mut Vec<T>
where
    T: Ord + Copy,
{
    for j in 1..a.len() {
        let key = a[j];
        // Insert a[j] into the sorted sequence a[1..j - 1]
        let mut i = j - 1;
        i = loop {
            if a[i] <= key {
                break i + 1;
            }
            a[i + 1] = a[i];
            if i == 0 {
                break i;
            }
            i -= 1;
        };
        a[i] = key;
    }
    a
}

/// Selection sort as detailed in exercise 2-2 from CLRS. This was a lot less painful than selection sort, although I
/// still had to be careful with indexing: the inner loop goes to a farther range than the outer loop. Also used the swap
/// method just ot make it a bit cleaner :)
///
/// # Examples
/// ```
/// let y = &mut vec![5, 2, 4, 6, 1, 3];
/// let answer = &mut vec![1, 2, 3, 4, 5, 6];
/// assert_eq!(answer, sorts::selection_sort(y));
/// ```
pub fn selection_sort<T>(a: &mut Vec<T>) -> &mut Vec<T>
where
    T: Copy + Ord,
{
    for i in 0..(a.len() - 1) {
        let mut min = i;
        for j in i..a.len() {
            if a[j] < a[min] {
                min = j;
            }
        }
        a.swap(min, i);
    }
    a
}
