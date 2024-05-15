use crate::graph::{Direction, GraphData, NodeIndex, Point, INVALID_NODE_INDEX};
use crate::{HashMap, HashSet};
use std::cmp::Reverse;

type PriorityQueue<I, P> = priority_queue::PriorityQueue<I, P, ahash::RandomState>;

#[derive(Debug, Clone, Copy)]
pub enum PathFindResult<T> {
    Found(T),
    NotFound,
    InvalidStartPoint,
    InvalidEndPoint,
}

impl<T> PathFindResult<T> {
    #[inline]
    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> PathFindResult<U> {
        match self {
            Self::Found(value) => PathFindResult::Found(f(value)),
            Self::NotFound => PathFindResult::NotFound,
            Self::InvalidStartPoint => PathFindResult::InvalidStartPoint,
            Self::InvalidEndPoint => PathFindResult::InvalidEndPoint,
        }
    }
}

#[derive(Default, Clone)]
pub struct Path {
    points: Vec<Point>,
    dirs: Vec<Option<Direction>>,
}

impl Path {
    #[inline]
    fn clear(&mut self) {
        self.points.clear();
        self.dirs.clear();
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = Point> + '_ {
        self.points.iter().copied()
    }

    pub fn iter_pruned(&self) -> impl Iterator<Item = Point> + '_ {
        assert_eq!(self.points.len(), self.dirs.len());

        let mut prev_dir: Option<Direction> = None;
        self.points
            .iter()
            .zip(&self.dirs)
            .filter_map(move |(&point, &dir)| {
                let include = match (dir, prev_dir) {
                    (Some(dir), Some(prev_dir)) => dir != prev_dir,
                    _ => true,
                };

                prev_dir = dir;

                if include {
                    Some(point)
                } else {
                    None
                }
            })
    }
}

#[derive(Default)]
pub(crate) struct PathFinder {
    end_indices: HashSet<NodeIndex>,
    g_score: HashMap<NodeIndex, u32>,
    predecessor: HashMap<NodeIndex, NodeIndex>,
    open_queue: PriorityQueue<NodeIndex, Reverse<u32>>,
    path: Path,
}

impl PathFinder {
    #[cfg(debug_assertions)]
    fn assert_data_is_valid(&self, graph: &GraphData) {
        for (&node_index, &pred_index) in &self.predecessor {
            assert_ne!(node_index, INVALID_NODE_INDEX);
            assert_ne!(pred_index, INVALID_NODE_INDEX);

            let node = &graph.nodes[node_index];
            let pred = &graph.nodes[pred_index];

            let node_to_pred_dir = node.neighbors.find(pred_index);
            let pred_to_node_dir = pred.neighbors.find(node_index);

            assert!(node_to_pred_dir.is_some());
            assert!(pred_to_node_dir.is_some());
            assert_eq!(
                node_to_pred_dir.unwrap(),
                pred_to_node_dir.unwrap().opposite()
            );
        }
    }

    #[cfg(not(debug_assertions))]
    fn assert_data_is_valid(&self, _graph: &GraphData) {}

    fn build_path(&mut self, graph: &GraphData, end_index: NodeIndex) {
        self.path.clear();
        self.path.points.push(graph.nodes[end_index].position);

        let mut current_index = end_index;
        loop {
            // The final node in the path has no predecessor.
            let Some(&pred_index) = self.predecessor.get(&current_index) else {
                break;
            };

            let pred_dir = graph.nodes[current_index]
                .neighbors
                .find(pred_index)
                .expect("invalid predecessor");

            self.path.dirs.push(Some(pred_dir));
            self.path.points.push(graph.nodes[pred_index].position);

            current_index = pred_index;
        }

        self.path.dirs.push(None);
    }

    /// A* path finding.
    pub(crate) fn find_path<'a>(
        &'a mut self,
        graph: &GraphData,
        start: Point,
        ends: impl IntoIterator<Item = Point>,
    ) -> PathFindResult<&'a Path> {
        self.end_indices.clear();
        self.g_score.clear();
        self.predecessor.clear();
        self.open_queue.clear();

        let Some(start_index) = graph.find_node(start) else {
            return PathFindResult::InvalidStartPoint;
        };

        let mut total_neighbor_count = 0;
        for end in ends {
            let Some(end_index) = graph.find_node(end) else {
                return PathFindResult::InvalidEndPoint;
            };

            let end_node = &graph.nodes[end_index];
            let neighbor_count = end_node.neighbor_count();
            total_neighbor_count += neighbor_count;

            if neighbor_count > 0 {
                self.end_indices.insert(end_index);
            }
        }

        if total_neighbor_count == 0 {
            // There cannot possibly be a path, abort.
            return PathFindResult::NotFound;
        }

        self.g_score.insert(start_index, 0);
        self.open_queue.push(start_index, Reverse(0));

        while let Some((current_index, _)) = self.open_queue.pop() {
            // Shortest path found, construct it and return.
            if self.end_indices.contains(&current_index) {
                self.assert_data_is_valid(graph);
                self.build_path(graph, current_index);
                return PathFindResult::Found(&self.path);
            }

            let current_node = &graph.nodes[current_index];
            let pred = self
                .predecessor
                .get(&current_index)
                .map(|&pred_index| (pred_index, &graph.nodes[pred_index]));

            // Find which direction is straight ahead to apply weights.
            let straight_dir = if let Some((pred_index, pred_node)) = pred {
                let pred_to_current_dir = pred_node
                    .neighbors
                    .find(current_index)
                    .expect("invalid predecessor");

                let current_to_pred_dir = current_node.neighbors.find(pred_index);
                debug_assert_eq!(current_to_pred_dir, Some(pred_to_current_dir.opposite()));

                Some(pred_to_current_dir)
            } else {
                None
            };

            for dir in Direction::ALL {
                if Some(dir.opposite()) == straight_dir {
                    debug_assert_eq!(current_node.neighbors[dir], pred.unwrap().0);

                    // The path came from here.
                    continue;
                }

                let neighbor_index = current_node.neighbors[dir];
                if neighbor_index == INVALID_NODE_INDEX {
                    continue;
                }

                let neighbor_node = &graph.nodes[neighbor_index];
                debug_assert_eq!(neighbor_node.neighbors[dir.opposite()], current_index);

                // Calculate the new path lenght.
                let new_g_score = self.g_score[&current_index]
                    + current_node
                        .position
                        .manhatten_distance_to(neighbor_node.position)
                        * if Some(dir) == straight_dir { 1 } else { 2 };

                // Check whether the new path length is shorter than the previous one.
                let update = match self.g_score.get(&neighbor_index) {
                    Some(&g_score) => new_g_score < g_score,
                    None => true,
                };

                if update {
                    // Shorter path found, update it.
                    self.g_score.insert(neighbor_index, new_g_score);
                    self.predecessor.insert(neighbor_index, current_index);

                    // Calculate the new approximate total cost.
                    let new_f_score = new_g_score
                        + self
                            .end_indices
                            .iter()
                            .map(|&end_index| &graph.nodes[end_index])
                            .map(|end| neighbor_node.position.manhatten_distance_to(end.position))
                            .min()
                            .expect("empty end point list");

                    self.open_queue.push(neighbor_index, Reverse(new_f_score));
                }
            }
        }

        PathFindResult::NotFound
    }
}
