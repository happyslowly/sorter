pub struct Heap<T>
where
    T: Ord,
{
    queue: Vec<T>,
    size: usize,
}

impl<T> Heap<T>
where
    T: Ord,
{
    pub fn from(v: Vec<T>) -> Heap<T> {
        let len = v.len();
        let mut heap = Heap {
            queue: v,
            size: len,
        };
        for i in (0..heap.size / 2).rev() {
            heap.siftdown(i);
        }
        heap
    }

    pub fn poll(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let item = self.queue.swap_remove(0);
        self.size -= 1;
        self.siftdown(0);
        Some(item)
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn left(&self, i: usize) -> Option<usize> {
        let j = i * 2 + 1;
        if j >= self.size {
            None
        } else {
            Some(j)
        }
    }

    fn right(&self, i: usize) -> Option<usize> {
        let j = i * 2 + 2;
        if j >= self.size {
            None
        } else {
            Some(j)
        }
    }

    fn siftdown(&mut self, i: usize) {
        let left = self.left(i);
        let right = self.right(i);
        let mut min = i;
        if let Some(left) = left {
            if self.queue[left] < self.queue[min] {
                min = left;
            }
        }
        if let Some(right) = right {
            if self.queue[right] < self.queue[min] {
                min = right;
            }
        }
        if min != i {
            self.queue.swap(min, i);
            self.siftdown(min);
        }
    }
}
