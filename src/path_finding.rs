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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PathNodeKind {
    Normal,
    Start,
    End,
    Waypoint,
}

#[derive(Debug, Clone, Copy)]
pub struct PathNode {
    pub position: Point,
    pub kind: PathNodeKind,
}

#[derive(Default, Clone)]
pub struct Path {
    points: Vec<Point>,
    meta: Vec<(PathNodeKind, Option<Direction>)>,
}

impl Path {
    #[inline]
    fn clear(&mut self) {
        self.points.clear();
        self.meta.clear();
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = PathNode> + '_ {
        self.points
            .iter()
            .zip(&self.meta)
            .map(move |(&point, &(kind, _))| PathNode {
                position: point,
                kind,
            })
    }

    pub fn iter_pruned(&self) -> impl Iterator<Item = PathNode> + '_ {
        assert_eq!(self.points.len(), self.meta.len());

        let mut prev_dir: Option<Direction> = None;
        self.points
            .iter()
            .zip(&self.meta)
            .filter_map(move |(&point, &(kind, dir))| {
                let include = if kind == PathNodeKind::Normal {
                    match (dir, prev_dir) {
                        (Some(dir), Some(prev_dir)) => dir != prev_dir,
                        _ => true,
                    }
                } else {
                    true
                };

                prev_dir = dir;

                if include {
                    Some(PathNode {
                        position: point,
                        kind,
                    })
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
        assert_eq!(self.path.points.len(), self.path.meta.len());

        // If there was a previous path segment, don't duplicate the joining point.
        if self.path.points.len() > 0 {
            self.path.points.pop();
            assert_eq!(
                self.path.meta.pop(),
                Some((PathNodeKind::End, None)),
                "invalid end node",
            );
        }

        let insert_index = self.path.points.len();
        self.path.points.push(graph.nodes[end_index].position);
        self.path.meta.push((PathNodeKind::End, None));

        let mut current_index = end_index;
        // The final node in the path has no predecessor.
        while let Some(&pred_index) = self.predecessor.get(&current_index) {
            let pred = &graph.nodes[pred_index];

            let dir = pred
                .neighbors
                .find(current_index)
                .expect("invalid predecessor");

            self.path.points.insert(insert_index, pred.position);
            self.path
                .meta
                .insert(insert_index, (PathNodeKind::Normal, Some(dir)));

            current_index = pred_index;
        }

        self.path.meta[insert_index].0 = if insert_index == 0 {
            PathNodeKind::Start
        } else {
            PathNodeKind::Waypoint
        };
    }

    /// A* path finding.
    pub(crate) fn find_path<'a>(
        &'a mut self,
        graph: &GraphData,
        start: Point,
        ends: impl IntoIterator<Item = Point>,
        visit_all: bool,
    ) -> PathFindResult<&'a Path> {
        let Some(mut start_index) = graph.find_node(start) else {
            return PathFindResult::InvalidStartPoint;
        };

        self.end_indices.clear();
        self.path.clear();

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

        'outer: loop {
            if total_neighbor_count == 0 {
                // There cannot possibly be a path, abort.
                break 'outer;
            }

            self.g_score.clear();
            self.predecessor.clear();
            self.open_queue.clear();

            self.g_score.insert(start_index, 0);
            self.open_queue.push(start_index, Reverse(0));

            while let Some((current_index, _)) = self.open_queue.pop() {
                let current_node = &graph.nodes[current_index];

                // Shortest path to one end found, construct it.
                if self.end_indices.contains(&current_index) {
                    self.assert_data_is_valid(graph);
                    self.build_path(graph, current_index);

                    if visit_all {
                        self.end_indices.remove(&current_index);
                        total_neighbor_count -= current_node.neighbor_count();
                        start_index = current_index;

                        continue 'outer;
                    } else {
                        break 'outer;
                    }
                }

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
                                .map(|end| {
                                    neighbor_node.position.manhatten_distance_to(end.position)
                                })
                                .min()
                                .expect("empty end point list");

                        self.open_queue.push(neighbor_index, Reverse(new_f_score));
                    }
                }
            }

            break 'outer;
        }

        if self.path.points.len() > 0 {
            PathFindResult::Found(&self.path)
        } else {
            PathFindResult::NotFound
        }
    }
}
