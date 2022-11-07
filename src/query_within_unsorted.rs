use crate::sok::{Axis, Content, LEAF_OFFSET};
use crate::KdTree;
use std::ops::Rem;

impl<A: Axis, T: Content, const K: usize, const B: usize> KdTree<A, T, K, B> {
    #[inline]
    pub fn within_unsorted<F>(&self, query: &[A; K], radius: A, distance_fn: &F) -> Vec<(A, T)>
    where
        F: Fn(&[A; K], &[A; K]) -> A,
    {
        let mut matching_items = Vec::new();

        self.within_unsorted_recurse(
            query,
            radius,
            distance_fn,
            self.root_index,
            0,
            &mut matching_items,
        );

        matching_items
    }

    fn within_unsorted_recurse<F>(
        &self,
        query: &[A; K],
        radius: A,
        distance_fn: &F,
        curr_node_idx: usize,
        split_dim: usize,
        matching_items: &mut Vec<(A, T)>,
    ) where
        F: Fn(&[A; K], &[A; K]) -> A,
    {
        if KdTree::<A, T, K, B>::is_stem_index(curr_node_idx) {
            let node = &self.stems[curr_node_idx];

            let child_node_indices = if query[split_dim] < node.split_val {
                [node.left, node.right]
            } else {
                [node.right, node.left]
            };
            let next_split_dim = (split_dim + 1).rem(K);

            for node_idx in child_node_indices {
                let child_node_dist = self.child_dist_to_bounds(query, node_idx, distance_fn);
                if child_node_dist <= radius {
                    self.within_unsorted_recurse(
                        query,
                        radius,
                        distance_fn,
                        node_idx,
                        next_split_dim,
                        matching_items,
                    );
                }
            }
        } else {
            let leaf_node = &self.leaves[curr_node_idx - LEAF_OFFSET];

            leaf_node
                .content
                .iter()
                .take(leaf_node.size)
                .for_each(|entry| {
                    let distance = distance_fn(query, &entry.point);
                    if distance < radius {
                        matching_items.push((distance, entry.item))
                    }
                });
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::distance::squared_euclidean;
    use crate::KdTree;
    use rand::Rng;
    use std::cmp::Ordering;
    use num_traits::Float;

    #[test]
    fn can_query_items_within_unsorted_radius() {
        // let mut tree: KdTree<f64, i32, 2, 4> = KdTree::new();
        // let content_to_add = [
        //     ([9f64, 0f64], 9),
        //     ([4f64, 500f64], 4),
        //     ([12f64, -300f64], 12),
        //     ([7f64, 200f64], 7),
        //     ([13f64, -400f64], 13),
        //     ([6f64, 300f64], 6),
        //     ([2f64, 700f64], 2),
        //     ([14f64, -500f64], 14),
        //     ([3f64, 600f64], 3),
        //     ([10f64, -100f64], 10),
        //     ([16f64, -700f64], 16),
        //     ([1f64, 800f64], 1),
        //     ([15f64, -600f64], 15),
        //     ([5f64, 400f64], 5),
        //     ([8f64, 100f64], 8),
        //     ([11f64, -200f64], 11),
        // ];

        let mut tree: KdTree<f32, i32, 4, 4> = KdTree::new();
        let content_to_add: [([f32; 4], i32); 16] = [
            ([0.9f32, 0.0f32, 0.9f32, 0.0f32], 9),
            ([0.4f32, 0.5f32, 0.4f32, 0.5f32], 4),
            ([0.12f32, 0.3f32, 0.12f32, 0.3f32], 12),
            ([0.7f32, 0.2f32, 0.7f32, 0.2f32], 7),
            ([0.13f32, 0.4f32, 0.13f32, 0.4f32], 13),
            ([0.6f32, 0.3f32, 0.6f32, 0.3f32], 6),
            ([0.2f32, 0.7f32, 0.2f32, 0.7f32], 2),
            ([0.14f32, 0.5f32, 0.14f32, 0.5f32], 14),
            ([0.3f32, 0.6f32, 0.3f32, 0.6f32], 3),
            ([0.10f32, 0.1f32, 0.10f32, 0.1f32], 10),
            ([0.16f32, 0.7f32, 0.16f32, 0.7f32], 16),
            ([0.1f32, 0.8f32, 0.1f32, 0.8f32], 1),
            ([0.15f32, 0.6f32, 0.15f32, 0.6f32], 15),
            ([0.5f32, 0.4f32, 0.5f32, 0.4f32], 5),
            ([0.8f32, 0.1f32, 0.8f32, 0.1f32], 8),
            ([0.11f32, 0.2f32, 0.11f32, 0.2f32], 11),
        ];

        for (point, item) in content_to_add {
            tree.add(&point, item);
        }

        assert_eq!(tree.size(), 16);

        // let query_point = [9f64, 0f64];
        // let radius = 20000f64;

        let query_point = [
            0.78f32,
            0.55f32,
            0.78f32,
            0.55f32,
        ];
        let radius = 0.2;

        let expected = linear_search(&content_to_add, &query_point, radius);

        let mut result = tree.within(&query_point, radius, &squared_euclidean);
        sort_result(&mut result);
        assert_eq!(result, expected);

        let mut rng = rand::thread_rng();
        for _i in 0..1000 {
            // let query_point = [
            //     rng.gen_range(-10f64..20f64),
            //     rng.gen_range(-1000f64..1000f64),
            // ];
            // let radius = 10000f64;

            let query_point = [
                rng.gen_range(0f32..1f32),
                rng.gen_range(0f32..1f32),
                rng.gen_range(0f32..1f32),
                rng.gen_range(0f32..1f32),
            ];
            let radius = 0.2;

            let expected = linear_search(&content_to_add, &query_point, radius);

            let mut result = tree.within(&query_point, radius, &squared_euclidean);
            sort_result(&mut result);

            assert_eq!(result, expected);
        }
    }

    fn linear_search<F: Float, const K: usize>(
        content: &[([F; K], i32)],
        query_point: &[F; K],
        radius: F,
    ) -> Vec<(F, i32)> {
        let mut matching_items = vec![];

        for &(p, item) in content {
            let dist = squared_euclidean(query_point, &p);
            if dist < radius {
                matching_items.push((dist, item));
            }
        }

        sort_result(&mut matching_items);
        matching_items
    }

    fn sort_result<F: Float>(vec: &mut Vec<(F, i32)>) {
        vec.sort_unstable_by(|a, b| {
            let dist_cmp = a.0.partial_cmp(&b.0).unwrap();
            if dist_cmp == Ordering::Equal {
                a.1.cmp(&b.1)
            } else {
                dist_cmp
            }
        });
    }
}
