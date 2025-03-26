#[doc(hidden)]
#[macro_export]
macro_rules! generate_immutable_within {
    ($comments:tt) => {
        doc_comment! {
            concat!$comments,
            #[inline]
            pub fn within<D>(&self, query: &[A; K], dist: A) -> Vec<NearestNeighbour<A, T>>
            where
                A: LeafSliceFloat<T> + LeafSliceFloatChunk<T, K>,
                D: DistanceMetric<A, K>,
                usize: Cast<T>,            {
                self.nearest_n_within::<D>(query, dist, std::num::NonZero::new(usize::MAX).unwrap(), true)
            }

            /// Helper function created by opencraft team for re-using a buffer.
            #[inline]
            pub fn collect_within<D>(&self, query: &[A; K], dist: A, buf: &mut Vec<NearestNeighbour<A, T>>) 
            where
                A: LeafSliceFloat<T> + LeafSliceFloatChunk<T, K>,
                D: DistanceMetric<A, K>,
                usize: Cast<T>,            
            {
                self.collect_nearest_n_within::<D>(query, dist, std::num::NonZero::new(usize::MAX).unwrap(), true, buf)
            }
        }
    };
}
