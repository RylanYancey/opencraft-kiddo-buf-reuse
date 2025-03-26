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
}

impl<'v, A: Axis, T: Content> ResultCollection<A, T> for BinaryHeapRef<'v, A, T> {
    fn new_with_capacity(_capacity: usize) -> Self {
        unimplemented!()
    }

    fn add(&mut self, entry: NearestNeighbour<A, T>) {
        let k = self.buf.capacity();
        if self.buf.len() < k {
            self.buf.push(entry);
        } else {
            let max_heap_value = &mut self.buf[0];
            if entry < *max_heap_value {
                *max_heap_value = entry;
            }
        }
    }

    fn max_dist(&self) -> A {
        if self.buf.len() < self.buf.capacity() {
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
}

impl<'v, A: Axis, T: Content> ResultCollection<A, T> for SortedVecRef<'v, A, T> {
    fn new_with_capacity(_capacity: usize) -> Self {
        unimplemented!()
    }

    fn add(&mut self, entry: NearestNeighbour<A, T>) {
        let len = self.buf.len();
        if len < self.buf.capacity() {
            match self.buf.binary_search (&entry) {
                Ok (insert_at) | Err (insert_at) => self.buf.insert(insert_at, entry),
            };
        } else if entry < *self.buf.last().unwrap() {
            self.buf.pop();
            self.buf.push(entry);
        }
    }

    fn max_dist(&self) -> A {
        if self.buf.len() < self.buf.capacity() {
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