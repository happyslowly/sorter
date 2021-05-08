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
    use std::time::SystemTime;

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

    fn test<S>()
    where
        S: Sorter,
    {
        let mut a = generate(5000);
        let now = SystemTime::now();
        sort::<_, S>(&mut a);
        let t = now.elapsed().unwrap().as_millis();
        println!("{} duration: {}(ms)", std::any::type_name::<S>(), t);
        assert!(sorted(&a));
    }

    #[test]
    fn test_bubblesort() {
        test::<bubblesort::BubbleSort>();
    }

    #[test]
    fn test_insertionsort() {
        test::<insertionsort::InsertionSort>();
    }

    #[test]
    fn test_selectionsort() {
        test::<selectionsort::SelectionSort>();
    }

    #[test]
    fn test_quicksort() {
        test::<quicksort::QuickSort>();
    }

    #[test]
    fn test_mergesort() {
        test::<mergesort::MergeSort>();
    }

    #[test]
    fn test_heapsort() {
        test::<heapsort::HeapSort>();
    }
}
