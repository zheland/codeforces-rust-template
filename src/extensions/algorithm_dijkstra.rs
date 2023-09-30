pub use algorithm_dijkstra::*;
mod algorithm_dijkstra {
    use core::fmt::{self, Debug, Formatter};
    use std::collections::BTreeSet;

    use crate::Max;

    #[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct NonMax<T>(T);

    impl<T: Debug + Eq + Max> NonMax<T> {
        #[track_caller]
        pub fn new(value: T) -> Self {
            assert_ne!(value, T::max());
            Self(value)
        }

        #[must_use]
        pub fn none() -> Self {
            Self(T::max())
        }

        pub fn get_ref(&self) -> Option<&T> {
            if self.0 == T::max() {
                None
            } else {
                Some(&self.0)
            }
        }

        pub fn get_mut(&mut self) -> Option<&mut T> {
            if self.0 == T::max() {
                None
            } else {
                Some(&mut self.0)
            }
        }

        pub fn into_inner(self) -> Option<T> {
            if self.0 == T::max() {
                None
            } else {
                Some(self.0)
            }
        }

        pub fn set_none(&mut self) {
            self.0 = T::max();
        }

        #[track_caller]
        pub fn set(&mut self, value: T) {
            assert_ne!(value, T::max());
            self.0 = value;
        }

        pub fn take(&mut self) -> Option<T> {
            let value = core::mem::replace(&mut self.0, T::max());
            Self(value).into_inner()
        }

        #[track_caller]
        pub fn replace(&mut self, value: T) -> Option<T> {
            assert_ne!(value, T::max());
            let old = core::mem::replace(&mut self.0, value);
            Self(old).into_inner()
        }
    }

    impl<T: Debug + Eq + Max> Default for NonMax<T> {
        fn default() -> Self {
            Self::none()
        }
    }

    impl<T: Debug + Eq + Max> From<NonMax<T>> for Option<T> {
        fn from(value: NonMax<T>) -> Option<T> {
            value.into_inner()
        }
    }

    impl<T: Debug + Eq + Max> From<T> for NonMax<T> {
        fn from(value: T) -> NonMax<T> {
            Self::new(value)
        }
    }

    impl<T: Debug + Eq + Max> From<Option<T>> for NonMax<T> {
        fn from(value: Option<T>) -> NonMax<T> {
            match value {
                Some(value) => Self::new(value),
                None => Self::none(),
            }
        }
    }

    impl<T: Debug + Eq + Max> Debug for NonMax<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            Debug::fmt(&self.get_ref(), f)
        }
    }

    #[derive(Clone, Debug)]
    pub struct Dijkstra<T> {
        pub nodes: Vec<usize>,
        pub dists: Vec<Option<T>>,
        pub prevs: Vec<NonMax<usize>>,
    }

    impl<T> Dijkstra<T> {
        pub fn new<I, F, E>(num_nodes: usize, initial: I, mut edges: F) -> Self
        where
            T: Clone + PartialOrd + Ord,
            I: IntoIterator<Item = (usize, T)>,
            F: FnMut(usize, T) -> Option<E>,
            E: IntoIterator<Item = (usize, T)>,
        {
            let mut queue = BTreeSet::new();
            let mut nodes = Vec::new();
            let mut dists = vec![None; num_nodes];
            let mut prevs = vec![NonMax::none(); num_nodes];
            for (node_id, dist) in initial {
                let _ = queue.insert((dist, node_id, None));
            }
            while let Some(item) = queue.pop_first() {
                let (source_dist, source_node_id, parent_node_id) = item;
                if dists[source_node_id].is_some() {
                    continue;
                }
                nodes.push(source_node_id);
                dists[source_node_id] = Some(source_dist.clone());
                prevs[source_node_id] = NonMax::from(parent_node_id);
                let edges = if let Some(edges) = edges(source_node_id, source_dist.clone()) {
                    edges
                } else {
                    break;
                };
                for edge in edges {
                    let target_node_id = edge.0;
                    if dists[target_node_id].is_some() {
                        continue;
                    }
                    let target_dist = edge.1;
                    assert!(
                        source_dist <= target_dist,
                        "Negative distance is not supported by the algorithm"
                    );
                    let _ = queue.insert((target_dist, target_node_id, Some(source_node_id)));
                }
            }
            Dijkstra {
                nodes,
                dists,
                prevs,
            }
        }
    }
}
