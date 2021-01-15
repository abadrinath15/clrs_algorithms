/// Insertion Sort as specified in CLRS
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
