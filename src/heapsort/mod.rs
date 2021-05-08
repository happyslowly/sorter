mod heap;

use super::Sorter;

pub struct HeapSort;

impl Sorter for HeapSort {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord + Clone,
    {
        let mut h = heap::Heap::from(slice.to_vec());
        for i in 0..h.size() {
            slice[i] = h.poll().unwrap();
        }
    }
}
