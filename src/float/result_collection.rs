use crate::float::kdtree::Axis;
use crate::nearest_neighbour::NearestNeighbour;
use crate::traits::Content;
use sorted_vec::SortedVec;
use std::collections::BinaryHeap;

pub trait ResultCollection<A: Axis, T: Content> {
    fn new_with_capacity(capacity: usize) -> Self;
    fn add(&mut self, entry: NearestNeighbour<A, T>);
    fn max_dist(&self) -> A;
    fn into_vec(self) -> Vec<NearestNeighbour<A, T>>;
    fn into_sorted_vec(self) -> Vec<NearestNeighbour<A, T>>;
}

impl<A: Axis, T: Content> ResultCollection<A, T> for BinaryHeap<NearestNeighbour<A, T>> {
    fn new_with_capacity(capacity: usize) -> Self {
        BinaryHeap::with_capacity(capacity)
    }
    fn add(&mut self, entry: NearestNeighbour<A, T>) {
        let k = self.capacity();
        if self.len() < k {
            self.push(entry);
        } else {
            let mut max_heap_value = self.peek_mut().unwrap();
            if entry < *max_heap_value {
                *max_heap_value = entry;
            }
        }
    }
    fn max_dist(&self) -> A {
        if self.len() < self.capacity() {
            A::infinity()
        } else {
            self.peek().map_or(A::infinity(), |n| n.distance)
        }
    }
    fn into_vec(self) -> Vec<NearestNeighbour<A, T>> {
        BinaryHeap::into_vec(self)
    }
    fn into_sorted_vec(self) -> Vec<NearestNeighbour<A, T>> {
        BinaryHeap::into_sorted_vec(self)
    }
}

impl<A: Axis, T: Content> ResultCollection<A, T> for Vec<NearestNeighbour<A, T>> {
    fn new_with_capacity(capacity: usize) -> Self {
        Vec::with_capacity(capacity)
    }

    fn add(&mut self, entry: NearestNeighbour<A, T>) {
        self.push(entry)
    }

    fn max_dist(&self) -> A {
        A::infinity()
    }

    fn into_vec(self) -> Vec<NearestNeighbour<A, T>> {
        self
    }

    fn into_sorted_vec(mut self) -> Vec<NearestNeighbour<A, T>> {
        self.sort();
        self
    }
}

impl<A: Axis, T: Content> ResultCollection<A, T> for SortedVec<NearestNeighbour<A, T>> {
    fn new_with_capacity(capacity: usize) -> Self {
        SortedVec::with_capacity(capacity)
    }

    fn add(&mut self, entry: NearestNeighbour<A, T>) {
        let len = self.len();
        if len < self.capacity() {
            self.insert(entry);
        } else if entry < *self.last().unwrap() {
            self.pop();
            self.push(entry);
        }
    }

    fn max_dist(&self) -> A {
        if self.len() < self.capacity() {
            A::infinity()
        } else {
            self.last().map_or(A::infinity(), |n| n.distance)
        }
    }

    fn into_vec(self) -> Vec<NearestNeighbour<A, T>> {
        self.into_vec()
    }

    fn into_sorted_vec(self) -> Vec<NearestNeighbour<A, T>> {
        self.into_vec()
    }
}

pub struct BinaryHeapRef<'v, A: Axis, T: Content> {
    pub buf: &'v mut Vec<NearestNeighbour<A, T>>,
    pub cap: usize,
}

impl<'v, A: Axis, T: Content> ResultCollection<A, T> for BinaryHeapRef<'v, A, T> {
    fn new_with_capacity(_capacity: usize) -> Self {
        unimplemented!()
    }

    fn add(&mut self, entry: NearestNeighbour<A, T>) {
        if self.buf.len() < self.cap {
            self.buf.push(entry);
        } else {
            let max_heap_value = &mut self.buf[0];
            if entry < *max_heap_value {
                *max_heap_value = entry;
            }
        }
    }

    fn max_dist(&self) -> A {
        if self.buf.len() < self.cap {
            A::infinity()
        } else {
            self.buf.get(0).map_or(A::infinity(), |n| n.distance)
        }
    }

    fn into_vec(self) -> Vec<NearestNeighbour<A, T>> {
        unimplemented!()
    }

    fn into_sorted_vec(self) -> Vec<NearestNeighbour<A, T>> {
        unimplemented!()
    }
}

pub struct SortedVecRef<'v, A: Axis, T: Content> {
    pub buf: &'v mut Vec<NearestNeighbour<A, T>>,
    pub cap: usize,
}

impl<'v, A: Axis, T: Content> ResultCollection<A, T> for SortedVecRef<'v, A, T> {
    fn new_with_capacity(_capacity: usize) -> Self {
        unimplemented!()
    }

    fn add(&mut self, entry: NearestNeighbour<A, T>) {
        let len = self.buf.len();
        if len < self.cap {
            match self.buf.binary_search (&entry) {
                Ok (insert_at) | Err (insert_at) => self.buf.insert(insert_at, entry),
            };
        } else if entry < *self.buf.last().unwrap() {
            self.buf.pop();
            self.buf.push(entry);
        }
    }

    fn max_dist(&self) -> A {
        if self.buf.len() < self.cap {
            A::infinity()
        } else {
            self.buf.last().map_or(A::infinity(), |n| n.distance)
        }
    }

    fn into_vec(self) -> Vec<NearestNeighbour<A, T>> {
        unimplemented!()
    }

    fn into_sorted_vec(self) -> Vec<NearestNeighbour<A, T>> {
        unimplemented!()
    }
}

pub struct ArrayRef<'v, A: Axis, T: Content, const N: usize> {
    pub array: &'v mut [NearestNeighbour<A, T>; N],
    pub len: usize,
}

impl<'v, A: Axis, T: Content, const N: usize> ResultCollection<A, T> for ArrayRef<'v, A, T, N> {
    fn new_with_capacity(_capacity: usize) -> Self {
        unimplemented!()
    }

    fn add(&mut self, entry: NearestNeighbour<A, T>) {
        if self.len < N {
            self.array[self.len] = entry;
            self.len += 1;
        }
    }

    fn max_dist(&self) -> A {
        A::infinity()
    }

    fn into_vec(self) -> Vec<NearestNeighbour<A, T>> {
        unimplemented!()
    }

    fn into_sorted_vec(self) -> Vec<NearestNeighbour<A, T>> {
        unimplemented!()
    }
}

pub struct SortedArrayRef<'v, A: Axis, T: Content, const N: usize> {
    pub array: &'v mut [NearestNeighbour<A, T>; N],
    pub len: usize,
}

impl<'v, A: Axis, T: Content, const N: usize> ResultCollection<A, T> for SortedArrayRef<'v, A, T, N> {
    fn new_with_capacity(_capacity: usize) -> Self {
        unimplemented!()
    }

    fn add(&mut self, entry: NearestNeighbour<A, T>) {
        let len = self.len;
        if len < N {
            for i in 0..self.len {
                if entry <= self.array[i] {
                    for k in (i+1)..N {
                        self.array[k] = self.array[k-1];
                    }

                    self.array[i] = entry;
                    if self.len < N { self.len += 1 }
                    break;
                }
            }
        } else if entry < self.array[N-1] {
            self.array[N-1] = entry;
        }
    }

    fn max_dist(&self) -> A {
        if self.len < N {
            A::infinity()
        } else {
            self.array[N-1].distance
        }
    }

    fn into_vec(self) -> Vec<NearestNeighbour<A, T>> {
        unimplemented!()
    }

    fn into_sorted_vec(self) -> Vec<NearestNeighbour<A, T>> {
        unimplemented!()
    }
}

pub struct BinaryHeapArray<'v, A: Axis, T: Content, const N: usize> {
    pub array: &'v mut [NearestNeighbour<A, T>; N],
    pub len: usize,
}

impl<'v, A: Axis, T: Content, const N: usize> ResultCollection<A, T> for BinaryHeapArray<'v, A, T, N> {
    fn new_with_capacity(_capacity: usize) -> Self {
        unimplemented!()
    }

    fn add(&mut self, entry: NearestNeighbour<A, T>) {
        if self.len < N {
            self.array[self.len] = entry;
            self.len += 1;
        } else {
            let max_heap_value = &mut self.array[0];
            if entry < *max_heap_value {
                *max_heap_value = entry;
            }
        }
    }

    fn max_dist(&self) -> A {
        if self.len < N {
            A::infinity()
        } else {
            self.array[0].distance
        }
    }

    fn into_vec(self) -> Vec<NearestNeighbour<A, T>> {
        unimplemented!()
    }

    fn into_sorted_vec(self) -> Vec<NearestNeighbour<A, T>> {
        unimplemented!()
    }
}