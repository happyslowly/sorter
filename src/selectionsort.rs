use super::Sorter;

pub struct SelectionSort;

impl Sorter for SelectionSort {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord + Clone,
    {
        for i in 0..slice.len() {
            let mut min = i;
            for j in i + 1..slice.len() {
                if slice[j] < slice[min] {
                    min = j;
                }
            }

            if min != i {
                slice.swap(min, i);
            }
        }
    }
}
