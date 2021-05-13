pub trait Sorter {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord + Clone;
}

pub fn sort<T, S>(slice: &mut [T])
where
    T: Ord + Clone,
    S: Sorter,
{
    S::sort(slice);
}

mod bubblesort;

mod insertionsort;

mod selectionsort;

mod quicksort;

mod mergesort;

mod heapsort;

pub use bubblesort::BubbleSort;
pub use heapsort::HeapSort;
pub use insertionsort::InsertionSort;
pub use mergesort::MergeSort;
pub use quicksort::QuickSort;
pub use selectionsort::SelectionSort;

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;

    fn generate(count: usize) -> Vec<i32> {
        let mut v = vec![0; count];
        thread_rng().fill(&mut v[..]);
        v
    }

    fn sorted<T>(a: &[T]) -> bool
    where
        T: Ord,
    {
        for i in 1..a.len() {
            if a[i - 1] > a[i] {
                return false;
            }
        }
        true
    }

    fn do_test<S>()
    where
        S: Sorter,
    {
        let mut a = generate(5000);
        sort::<_, S>(&mut a);
        assert!(sorted(&a));
    }

    #[test]
    fn test_bubblesort() {
        do_test::<bubblesort::BubbleSort>();
    }

    #[test]
    fn test_insertionsort() {
        do_test::<insertionsort::InsertionSort>();
    }

    #[test]
    fn test_selectionsort() {
        do_test::<selectionsort::SelectionSort>();
    }

    #[test]
    fn test_quicksort() {
        do_test::<quicksort::QuickSort>();
    }

    #[test]
    fn test_mergesort() {
        do_test::<mergesort::MergeSort>();
    }

    #[test]
    fn test_heapsort() {
        do_test::<heapsort::HeapSort>();
    }
}
