use crate::graph::{Direction, GraphData, NodeIndex, Point, INVALID_NODE_INDEX};
use crate::{HashMap, HashSet, ReplayCapture};
use std::borrow::Borrow;
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
    pub bend_direction: Option<Direction>,
}

#[derive(Default, Clone)]
#[repr(transparent)]
pub struct Path {
    nodes: Vec<PathNode>,
}

impl Path {
    #[inline]
    fn clear(&mut self) {
        self.nodes.clear();
    }

    #[inline]
    pub fn nodes(&self) -> &[PathNode] {
        &self.nodes
    }

    pub fn iter_pruned(&self) -> impl Iterator<Item = (usize, PathNode)> + '_ {
        let mut prev_dir: Option<Direction> = None;
        self.nodes
            .iter()
            .enumerate()
            .filter_map(move |(index, &node)| {
                let include = if node.kind == PathNodeKind::Normal {
                    match (node.bend_direction, prev_dir) {
                        (Some(dir), Some(prev_dir)) => dir != prev_dir,
                        _ => true,
                    }
                } else {
                    true
                };

                prev_dir = node.bend_direction;

                if include {
                    Some((index, node))
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

    fn build_path(
        &mut self,
        graph: &GraphData,
        start_index: NodeIndex,
        end_index: NodeIndex,
        replay: &mut impl ReplayCapture,
    ) {
        // If there was a previous path segment, don't duplicate the joining point.
        if self.path.nodes.len() > 0 {
            replay.path_finding_remove_path_node(self.path.nodes.len());

            let prev_end = self.path.nodes.pop();
            assert!(
                matches!(
                    prev_end,
                    Some(PathNode {
                        kind: PathNodeKind::End,
                        bend_direction: None,
                        ..
                    }),
                ),
                "invalid end node",
            );
        }

        let insert_index = self.path.nodes.len();
        self.path.nodes.push(PathNode {
            position: graph.nodes[end_index].position,
            kind: PathNodeKind::End,
            bend_direction: None,
        });

        replay.path_finding_insert_path_node(insert_index, end_index);

        if end_index == start_index {
            return;
        }

        let mut current_index = end_index;
        loop {
            let pred_index = *self.predecessor.get(&current_index).expect("invalid path");
            let pred = &graph.nodes[pred_index];

            let dir = pred
                .neighbors
                .find(current_index)
                .expect("invalid predecessor");

            if pred_index == start_index {
                let kind = if insert_index == 0 {
                    PathNodeKind::Start
                } else {
                    PathNodeKind::Waypoint
                };

                self.path.nodes.insert(
                    insert_index,
                    PathNode {
                        position: pred.position,
                        kind,
                        bend_direction: Some(dir),
                    },
                );

                replay.path_finding_insert_path_node(insert_index, pred_index);

                break;
            } else {
                self.path.nodes.insert(
                    insert_index,
                    PathNode {
                        position: pred.position,
                        kind: PathNodeKind::Normal,
                        bend_direction: Some(dir),
                    },
                );

                replay.path_finding_insert_path_node(insert_index, pred_index);

                current_index = pred_index;
            }
        }
    }

    /// A* path finding.
    pub(crate) fn find_path<'a>(
        &'a mut self,
        graph: &GraphData,
        start: Point,
        start_straight_dir: Option<Direction>,
        ends: impl IntoIterator<Item = Point>,
        visit_all: bool,
        replay: &mut impl ReplayCapture,
    ) -> PathFindResult<&'a Path> {
        let Some(mut start_index) = graph.find_node(start) else {
            println!(
                "Start point ({}, {}) does not exist in the graph",
                start.x, start.y
            );
            return PathFindResult::InvalidStartPoint;
        };

        self.end_indices.clear();
        self.path.clear();

        let mut total_neighbor_count = 0;
        for end in ends {
            let end = *end.borrow();

            let Some(end_index) = graph.find_node(end) else {
                println!(
                    "End point ({}, {}) does not exist in the graph",
                    end.x, end.y
                );
                return PathFindResult::InvalidEndPoint;
            };

            let end_node = &graph.nodes[end_index];
            let neighbor_count = end_node.neighbor_count();

            if neighbor_count > 0 {
                if self.end_indices.insert(end_index) {
                    total_neighbor_count += neighbor_count;
                }
            } else {
                #[cfg(debug_assertions)]
                println!("Waypoint ({}, {}) unreachable, skipping", end.x, end.y);
            }
        }

        replay.begin_path_finding(start_index, self.end_indices.iter().copied(), visit_all);

        self.g_score.clear();
        self.predecessor.clear();
        self.open_queue.clear();

        self.g_score.insert(start_index, 0);
        replay.path_finding_set_g_score(start_index, 0);
        self.open_queue.push(start_index, Reverse(0));
        replay.path_finding_push_open_queue(start_index, 0);

        'outer: loop {
            if total_neighbor_count == 0 {
                // There cannot possibly be a path, abort.
                break 'outer;
            }

            while let Some((current_index, _)) = self.open_queue.pop() {
                replay.path_finding_pop_open_queue();

                let current_node = &graph.nodes[current_index];

                let pred_index = self.predecessor.get(&current_index).copied();

                // Shortest path to one end found, construct it.
                if self.end_indices.contains(&current_index) {
                    self.assert_data_is_valid(graph);
                    self.build_path(graph, start_index, current_index, replay);

                    if visit_all {
                        self.end_indices.remove(&current_index);
                        total_neighbor_count -= current_node.neighbor_count();
                        start_index = current_index;

                        replay.path_finding_clear_state();

                        self.g_score.clear();
                        self.predecessor.clear();
                        self.open_queue.clear();

                        self.g_score.insert(start_index, 0);
                        replay.path_finding_set_g_score(start_index, 0);
                        self.open_queue.push(start_index, Reverse(0));
                        replay.path_finding_push_open_queue(start_index, 0);

                        if let Some(pred_index) = pred_index {
                            self.predecessor.insert(start_index, pred_index);
                            replay.path_finding_set_predecessor(start_index, pred_index);
                        }

                        continue 'outer;
                    } else {
                        break 'outer;
                    }
                }

                let pred = pred_index.map(|pred_index| (pred_index, &graph.nodes[pred_index]));

                // Find which direction is straight ahead to apply weights.
                let straight_dir = pred
                    .map(|(pred_index, pred_node)| {
                        let pred_to_current_dir = pred_node
                            .neighbors
                            .find(current_index)
                            .expect("invalid predecessor");

                        let current_to_pred_dir = current_node.neighbors.find(pred_index);
                        debug_assert_eq!(current_to_pred_dir, Some(pred_to_current_dir.opposite()));

                        pred_to_current_dir
                    })
                    .or(start_straight_dir);

                for dir in Direction::ALL {
                    if Some(dir.opposite()) == straight_dir {
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
                        replay.path_finding_set_g_score(neighbor_index, new_g_score);
                        self.predecessor.insert(neighbor_index, current_index);
                        replay.path_finding_set_predecessor(neighbor_index, current_index);

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
                        replay.path_finding_push_open_queue(neighbor_index, new_f_score);
                    }
                }
            }

            #[cfg(debug_assertions)]
            {
                print!("Unable to find path to remaining waypoints [");
                for (i, &end_index) in self.end_indices.iter().enumerate() {
                    if i > 0 {
                        print!(", ");
                    }

                    let end_node = &graph.nodes[end_index];
                    print!("({}, {})", end_node.position.x, end_node.position.y);
                }
                println!("]");
            }

            break 'outer;
        }

        if self.path.nodes.len() > 0 {
            replay.end_path_finding(true);
            PathFindResult::Found(&self.path)
        } else {
            replay.end_path_finding(false);
            PathFindResult::NotFound
        }
    }
}
