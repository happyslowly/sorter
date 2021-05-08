use super::Sorter;

pub struct MergeSort;

impl Sorter for MergeSort {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord + Clone,
    {
        if slice.len() > 1 {
            let mid = slice.len() / 2;
            Self::sort(&mut slice[0..mid]);
            Self::sort(&mut slice[mid..]);
            MergeSort::merge(slice, mid);
        }
    }
}

impl MergeSort {
    fn merge<T>(slice: &mut [T], mid: usize)
    where
        T: Ord + Clone,
    {
        let left = slice[0..mid].to_vec();
        let right = slice[mid..].to_vec();
        let mut i = 0;
        let mut j = 0;
        let mut u = 0;
        while i < left.len() && j < right.len() {
            if left[i] < right[j] {
                slice[u] = left[i].clone();
                i += 1;
            } else {
                slice[u] = right[j].clone();
                j += 1;
            }
            u += 1;
        }
        while i < left.len() {
            slice[u] = left[i].clone();
            i += 1;
            u += 1;
        }
        while j < right.len() {
            slice[u] = right[j].clone();
            j += 1;
            u += 1;
        }
    }
}
