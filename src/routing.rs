use crate::graph::{NodeIndex, INVALID_NODE_INDEX};
use crate::*;
use std::mem::MaybeUninit;

pub(crate) struct Array<'a, T> {
    pub(crate) data: &'a mut [MaybeUninit<T>],
    pub(crate) len: usize,
}

impl<'a, T> Array<'a, T> {
    #[inline]
    fn push(&mut self, val: T) -> Result<(), ()> {
        let new_len = self.len.checked_add(1).ok_or(())?;
        if self.data.len() < new_len {
            return Err(());
        }

        self.data[self.len].write(val);
        self.len = new_len;
        Ok(())
    }
}

impl<'a, T> From<&'a mut [MaybeUninit<T>]> for Array<'a, T> {
    #[inline]
    fn from(data: &'a mut [MaybeUninit<T>]) -> Self {
        Self { data, len: 0 }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Vertex {
    /// The X coordinate of the vertex.
    pub x: f32,
    /// The Y coordinate of the vertex.
    pub y: f32,
}

impl From<Point> for Vertex {
    #[inline]
    fn from(value: Point) -> Self {
        Self {
            x: value.x as f32,
            y: value.y as f32,
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
#[repr(C)]
pub struct WireView {
    /// The number of vertices in this wire.
    pub vertex_count: u16,
}

#[derive(Debug, Default, Clone, Copy)]
#[repr(C)]
pub struct NetView {
    /// The offset into `wire_views` this nets wires start at.
    pub wire_offset: u32,
    /// The number of wires in this net.
    pub wire_count: u32,
    /// The offset into `vertices` this nets  vertices start at.
    pub vertex_offset: u32,
}

fn pick_root_path<I>(endpoints: I) -> Result<(Point, Point), ()>
where
    I: IntoIterator<Item = Point>,
    I::IntoIter: Clone,
{
    let mut max_dist = 0;
    let mut max_pair = (Point::ZERO, Point::ZERO);

    let mut count = 0;
    let mut iter = endpoints.into_iter();
    while let Some(a) = iter.next() {
        count += 1;

        for b in iter.clone() {
            let dist = a.manhatten_distance_to(b);
            if dist > max_dist {
                max_dist = dist;
                max_pair = (a, b);
            }
        }
    }

    if count < 2 {
        Err(())
    } else {
        Ok(max_pair)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ConnectionKind {
    Connected,
    ConnectedThroughAnchor,
    Unconnected,
}

fn are_connected_vertically(graph: &GraphData, mut a: NodeIndex, b: NodeIndex) -> ConnectionKind {
    let node_a = &graph.nodes[a];
    let node_b = &graph.nodes[b];

    let dir = if node_a.position.y < node_b.position.y {
        Direction::PosY
    } else {
        assert!(node_a.position.y > node_b.position.y);
        Direction::NegY
    };

    let mut through_anchor = node_a.is_anchor || node_b.is_anchor;
    a = node_a.neighbors[dir];
    while a != INVALID_NODE_INDEX {
        if a == b {
            return if through_anchor {
                ConnectionKind::ConnectedThroughAnchor
            } else {
                ConnectionKind::Connected
            };
        }

        let node = &graph.nodes[a];
        if node.is_anchor {
            through_anchor = true;
        }

        a = node.neighbors[dir];
    }

    ConnectionKind::Unconnected
}

fn are_connected_horizontally(graph: &GraphData, mut a: NodeIndex, b: NodeIndex) -> ConnectionKind {
    let node_a = &graph.nodes[a];
    let node_b = &graph.nodes[b];

    let dir = if node_a.position.x < node_b.position.x {
        Direction::PosX
    } else {
        assert!(node_a.position.x > node_b.position.x);
        Direction::NegX
    };

    let mut through_anchor = node_a.is_anchor || node_b.is_anchor;
    a = node_a.neighbors[dir];
    while a != INVALID_NODE_INDEX {
        if a == b {
            return if through_anchor {
                ConnectionKind::ConnectedThroughAnchor
            } else {
                ConnectionKind::Connected
            };
        }

        let node = &graph.nodes[a];
        if node.is_anchor {
            through_anchor = true;
        }

        a = node.neighbors[dir];
    }

    ConnectionKind::Unconnected
}

#[derive(Debug, Clone, Copy)]
enum NudgeOffset {
    Horizontal(i32),
    Vertical(i32),
}

fn center_in_alley(
    graph: &GraphData,
    node_a: &Node,
    a: &mut Point,
    node_b: &Node,
    b: &mut Point,
) -> NudgeOffset {
    if node_a.position.x == node_b.position.x {
        let mut min_x = node_a.position.x;
        let mut max_x = node_a.position.x;

        let mut current_node_a = node_a;
        let mut current_node_b = node_b;

        loop {
            let next_a_index = current_node_a.neighbors[Direction::NegX];
            let next_b_index = current_node_b.neighbors[Direction::NegX];

            if (next_a_index == INVALID_NODE_INDEX) || (next_b_index == INVALID_NODE_INDEX) {
                break;
            }

            current_node_a = &graph.nodes[next_a_index];
            current_node_b = &graph.nodes[next_b_index];

            if current_node_a.position.x != current_node_b.position.x {
                break;
            }

            match are_connected_vertically(graph, next_a_index, next_b_index) {
                ConnectionKind::Connected => {
                    min_x = current_node_a.position.x;
                    continue;
                }
                ConnectionKind::ConnectedThroughAnchor => {
                    min_x = current_node_a.position.x;
                    break;
                }
                ConnectionKind::Unconnected => break,
            }
        }

        current_node_a = node_a;
        current_node_b = node_b;

        loop {
            let next_a_index = current_node_a.neighbors[Direction::PosX];
            let next_b_index = current_node_b.neighbors[Direction::PosX];

            if (next_a_index == INVALID_NODE_INDEX) || (next_b_index == INVALID_NODE_INDEX) {
                break;
            }

            current_node_a = &graph.nodes[next_a_index];
            current_node_b = &graph.nodes[next_b_index];

            if current_node_a.position.x != current_node_b.position.x {
                break;
            }

            match are_connected_vertically(graph, next_a_index, next_b_index) {
                ConnectionKind::Connected => {
                    max_x = current_node_a.position.x;
                    continue;
                }
                ConnectionKind::ConnectedThroughAnchor => {
                    max_x = current_node_a.position.x;
                    break;
                }
                ConnectionKind::Unconnected => break,
            }
        }

        let center_x = (min_x + max_x) / 2;
        a.x = center_x;
        b.x = center_x;

        NudgeOffset::Horizontal(center_x - node_a.position.x)
    } else {
        assert_eq!(node_a.position.y, node_b.position.y);

        let mut min_y = node_a.position.y;
        let mut max_y = node_a.position.y;

        let mut current_node_a = node_a;
        let mut current_node_b = node_b;

        loop {
            let next_a_index = current_node_a.neighbors[Direction::NegY];
            let next_b_index = current_node_b.neighbors[Direction::NegY];

            if (next_a_index == INVALID_NODE_INDEX) || (next_b_index == INVALID_NODE_INDEX) {
                break;
            }

            current_node_a = &graph.nodes[next_a_index];
            current_node_b = &graph.nodes[next_b_index];

            if current_node_a.position.y != current_node_b.position.y {
                break;
            }

            match are_connected_horizontally(graph, next_a_index, next_b_index) {
                ConnectionKind::Connected => {
                    min_y = current_node_a.position.y;
                    continue;
                }
                ConnectionKind::ConnectedThroughAnchor => {
                    min_y = current_node_a.position.y;
                    break;
                }
                ConnectionKind::Unconnected => break,
            }
        }

        current_node_a = node_a;
        current_node_b = node_b;

        loop {
            let next_a_index = current_node_a.neighbors[Direction::PosY];
            let next_b_index = current_node_b.neighbors[Direction::PosY];

            if (next_a_index == INVALID_NODE_INDEX) || (next_b_index == INVALID_NODE_INDEX) {
                break;
            }

            current_node_a = &graph.nodes[next_a_index];
            current_node_b = &graph.nodes[next_b_index];

            if current_node_a.position.y != current_node_b.position.y {
                break;
            }

            match are_connected_horizontally(graph, next_a_index, next_b_index) {
                ConnectionKind::Connected => {
                    max_y = current_node_a.position.y;
                    continue;
                }
                ConnectionKind::ConnectedThroughAnchor => {
                    max_y = current_node_a.position.y;
                    break;
                }
                ConnectionKind::Unconnected => break,
            }
        }

        let center_y = (min_y + max_y) / 2;
        a.y = center_y;
        b.y = center_y;

        NudgeOffset::Vertical(center_y - node_a.position.y)
    }
}

fn push_vertices(
    graph: &GraphData,
    vertices: &mut Array<Vertex>,
    path: &Path,
    ends: &mut HashMap<Point, Point>,
) -> Result<u16, ()> {
    let mut path_len = 0usize;

    let mut prev_prev_dir = None;
    let mut prev: Option<(usize, PathNode, &Node)> = None;
    for (path_node_index, mut path_node) in path.iter_pruned() {
        let node = &graph.nodes[graph
            .find_node(path_node.position)
            .expect("invalid wire vertex")];

        if let Some((prev_path_node_index, mut prev_path_node, prev_node)) = prev {
            let nudge_offset = if (prev_path_node.kind == PathNodeKind::Normal)
                && (path_node.kind == PathNodeKind::Normal)
                && (prev_prev_dir != Some(path_node.bend_direction.map(Direction::opposite)))
            {
                Some(center_in_alley(
                    graph,
                    prev_node,
                    &mut prev_path_node.position,
                    node,
                    &mut path_node.position,
                ))
            } else {
                None
            };

            for &PathNode { position, .. } in &path.nodes()[prev_path_node_index..=path_node_index]
            {
                let nudged = ends.entry(position).or_insert(position);

                match nudge_offset {
                    None => (),
                    Some(NudgeOffset::Horizontal(offset)) => nudged.x += offset,
                    Some(NudgeOffset::Vertical(offset)) => nudged.y += offset,
                }
            }

            vertices.push(prev_path_node.position.into())?;
            path_len += 1;
        }

        prev_prev_dir = prev.map(|(_, prev_path_node, _)| prev_path_node.bend_direction);
        prev = Some((path_node_index, path_node, node));
    }

    if let Some((_, prev_path_node, _)) = prev {
        vertices.push(ends[&prev_path_node.position].into())?;
        path_len += 1;
    }

    Ok(path_len.try_into().expect("path too long"))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoutingError {
    NotEnoughEndpoints,
    VertexBufferOverflow,
    WireViewBufferOverflow,
    InvalidPoint,
}

fn extend_ends(points: impl IntoIterator<Item = Point>, ends: &mut HashMap<Point, Point>) {
    use std::collections::hash_map::Entry;

    for point in points {
        match ends.entry(point) {
            Entry::Occupied(_) => (),
            Entry::Vacant(entry) => {
                entry.insert(point);
            }
        }
    }
}

fn route_root_wire<I>(
    graph: &GraphData,
    path_finder: &mut PathFinder,
    waypoints: I,
    root_start: Point,
    root_end: Point,
    vertices: &mut Array<Vertex>,
    wire_views: &mut Array<WireView>,
    ends: &mut HashMap<Point, Point>,
) -> Result<u32, RoutingError>
where
    I: IntoIterator<Item = Point>,
{
    let path_tail = match path_finder.find_path(graph, root_start, None, waypoints, true) {
        PathFindResult::Found(path) => {
            let path_len = push_vertices(graph, vertices, path, ends)
                .map_err(|_| RoutingError::VertexBufferOverflow)?;
            assert!(path_len >= 2);

            wire_views
                .push(WireView {
                    vertex_count: path_len,
                })
                .map_err(|_| RoutingError::WireViewBufferOverflow)?;

            let (last, head) = path.nodes().split_last().unwrap();
            let prev_last = head.last().unwrap();

            Some((last.position, prev_last.bend_direction))
        }
        PathFindResult::NotFound => None,
        PathFindResult::InvalidStartPoint | PathFindResult::InvalidEndPoint => {
            return Err(RoutingError::InvalidPoint);
        }
    };

    let path_len = match path_finder.find_path(
        graph,
        path_tail.map(|(last, _)| last).unwrap_or(root_start),
        path_tail.and_then(|(_, dir)| dir),
        [root_end],
        false,
    ) {
        PathFindResult::Found(path) => push_vertices(graph, vertices, path, ends)
            .map_err(|_| RoutingError::VertexBufferOverflow)?,
        PathFindResult::NotFound => {
            extend_ends([root_start, root_end], ends);
            vertices
                .push(root_start.into())
                .map_err(|_| RoutingError::VertexBufferOverflow)?;
            vertices
                .push(root_end.into())
                .map_err(|_| RoutingError::VertexBufferOverflow)?;
            2
        }
        PathFindResult::InvalidStartPoint | PathFindResult::InvalidEndPoint => {
            return Err(RoutingError::InvalidPoint);
        }
    };

    wire_views
        .push(WireView {
            vertex_count: path_len,
        })
        .map_err(|_| RoutingError::WireViewBufferOverflow)?;

    Ok((path_tail.is_some() as u32) + 1)
}

fn route_branch_wires<I>(
    graph: &GraphData,
    path_finder: &mut PathFinder,
    endpoints: I,
    root_start: Point,
    root_end: Point,
    vertices: &mut Array<Vertex>,
    wire_views: &mut Array<WireView>,
    ends: &mut HashMap<Point, Point>,
) -> Result<u32, RoutingError>
where
    I: IntoIterator<Item = Point>,
{
    let mut wire_count = 0;

    for endpoint in endpoints {
        if (endpoint != root_start) && (endpoint != root_end) {
            let path_len =
                match path_finder.find_path(graph, endpoint, None, ends.keys().copied(), false) {
                    PathFindResult::Found(path) => push_vertices(graph, vertices, path, ends)
                        .map_err(|_| RoutingError::VertexBufferOverflow)?,
                    PathFindResult::NotFound => {
                        extend_ends([endpoint, root_start], ends);
                        vertices
                            .push(endpoint.into())
                            .map_err(|_| RoutingError::VertexBufferOverflow)?;
                        vertices
                            .push(root_start.into())
                            .map_err(|_| RoutingError::VertexBufferOverflow)?;
                        2
                    }
                    PathFindResult::InvalidStartPoint | PathFindResult::InvalidEndPoint => {
                        return Err(RoutingError::InvalidPoint);
                    }
                };

            wire_views
                .push(WireView {
                    vertex_count: path_len,
                })
                .map_err(|_| RoutingError::WireViewBufferOverflow)?;

            wire_count += 1;
        }
    }

    Ok(wire_count)
}

pub(crate) fn connect_net<EndpointList, WaypointList>(
    graph: &Graph,
    endpoints: EndpointList,
    waypoints: WaypointList,
    vertex_base_offset: usize,
    wire_base_offset: usize,
    vertices: &mut Array<Vertex>,
    wire_views: &mut Array<WireView>,
    net_view: &mut MaybeUninit<NetView>,
    ends: &mut HashMap<Point, Point>,
) -> Result<(), RoutingError>
where
    EndpointList: Clone + IntoIterator<Item = Point>,
    EndpointList::IntoIter: Clone,
    WaypointList: IntoIterator<Item = Point>,
{
    let path_finder = &mut *graph.path_finder.get_or_default().borrow_mut();
    let (root_start, root_end) =
        pick_root_path(endpoints.clone()).map_err(|_| RoutingError::NotEnoughEndpoints)?;

    ends.clear();

    let wire_offset = (wire_base_offset + wire_views.len)
        .try_into()
        .expect("too many wires");
    let vertex_offset = (vertex_base_offset + vertices.len)
        .try_into()
        .expect("too many vertices");

    let root_wire_count = route_root_wire(
        &graph.data,
        path_finder,
        waypoints,
        root_start,
        root_end,
        vertices,
        wire_views,
        ends,
    )?;

    let branch_wire_count = route_branch_wires(
        &graph.data,
        path_finder,
        endpoints,
        root_start,
        root_end,
        vertices,
        wire_views,
        ends,
    )?;

    net_view.write(NetView {
        wire_offset,
        wire_count: root_wire_count + branch_wire_count,
        vertex_offset,
    });

    Ok(())
}
