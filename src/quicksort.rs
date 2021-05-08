use super::Sorter;
use rand::prelude::*;

pub struct QuickSort;

impl Sorter for QuickSort {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord + Clone,
    {
        if slice.len() > 1 {
            let p = QuickSort::partition(slice);
            Self::sort(&mut slice[..p]);
            Self::sort(&mut slice[p + 1..]);
        }
    }
}

impl QuickSort {
    fn partition<T>(slice: &mut [T]) -> usize
    where
        T: Ord,
    {
        let r = thread_rng().gen_range(0..slice.len());
        slice.swap(slice.len() - 1, r);
        let pivot = slice.len() - 1;
        let mut j = 0;
        for i in 0..pivot {
            if slice[i] < slice[pivot] {
                slice.swap(i, j);
                j += 1;
            }
        }
        slice.swap(pivot, j);
        j
    }
}
