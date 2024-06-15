use crate::graph::{NodeIndex, INVALID_NODE_INDEX};
use crate::*;
use std::borrow::Borrow;
use std::mem::MaybeUninit;
use std::ops::{Deref, DerefMut};

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

impl<'a, T> Deref for Array<'a, T> {
    type Target = [T];

    #[inline]
    #[allow(unsafe_code)]
    fn deref(&self) -> &Self::Target {
        let slice = &self.data[..self.len];
        unsafe { &*(slice as *const [MaybeUninit<T>] as *const [T]) }
    }
}

impl<'a, T> DerefMut for Array<'a, T> {
    #[inline]
    #[allow(unsafe_code)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        let slice = &mut self.data[..self.len];
        unsafe { &mut *(slice as *mut [MaybeUninit<T>] as *mut [T]) }
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
#[repr(transparent)]
pub struct WireView(u16);

impl WireView {
    #[inline]
    pub const fn new(vertex_count: usize, ends_in_junction: bool) -> Option<Self> {
        if vertex_count > 0x7FFF {
            return None;
        }

        Some(Self(
            (vertex_count as u16) | ((ends_in_junction as u16) << 15),
        ))
    }

    /// The number of vertices in this wire.
    #[inline]
    pub const fn vertex_count(self) -> usize {
        (self.0 & 0x7FFF) as usize
    }

    /// Wether this wire ends in a junction.
    #[inline]
    pub const fn ends_in_junction(self) -> bool {
        (self.0 >> 15) > 0
    }
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

fn pick_root_path<EndpointList, WaypointList, WaypointListRef>(
    mut endpoints: EndpointList,
) -> Result<(Endpoint<WaypointList>, Endpoint<WaypointList>), ()>
where
    WaypointListRef: ?Sized,
    for<'a> &'a WaypointListRef: IntoIterator<Item: Borrow<Point>>,
    WaypointList: Clone + Borrow<WaypointListRef>,
    EndpointList: Clone + Iterator<Item: Borrow<Endpoint<WaypointList>>>,
{
    let mut max_dist = 0;
    let mut max_pair: Option<(Endpoint<WaypointList>, Endpoint<WaypointList>)> = None;

    while let Some(a) = endpoints.next() {
        let a = a.borrow();

        for b in endpoints.clone() {
            let b = b.borrow();

            let dist = a.position.manhatten_distance_to(b.position);
            if dist >= max_dist {
                max_dist = dist;
                max_pair = Some((a.clone(), b.clone()));
            }
        }
    }

    max_pair.ok_or(())
}

pub(crate) struct CenteringCandidate {
    node_a: NodeIndex,
    node_b: NodeIndex,
    vertex_index: u32,
}

fn push_vertices(
    path: &Path,
    graph: &GraphData,
    vertices: &mut Array<Vertex>,
    ends: &mut Vec<Point>,
    centering_candidates: &mut Vec<CenteringCandidate>,
) -> Result<usize, ()> {
    ends.reserve(path.nodes().len());
    for node in path.nodes() {
        ends.push(node.position);
    }

    let mut path_len = 0usize;
    let mut prev_prev_dir = None;
    let mut prev_node: Option<PathNode> = None;
    for (_, node) in path.iter_pruned() {
        if let Some(prev_node) = prev_node {
            if (prev_node.kind == PathNodeKind::Normal)
                && (node.kind == PathNodeKind::Normal)
                && (prev_prev_dir != Some(node.bend_direction.map(Direction::opposite)))
            {
                centering_candidates.push(CenteringCandidate {
                    node_a: graph
                        .find_node(prev_node.position)
                        .expect("invalid path node"),
                    node_b: graph.find_node(node.position).expect("invalid path node"),
                    vertex_index: vertices.len as u32,
                });
            }

            vertices.push(prev_node.position.into())?;
            path_len += 1;
        }

        prev_prev_dir = prev_node.map(|prev_node| prev_node.bend_direction);
        prev_node = Some(node);
    }

    if let Some(prev_node) = prev_node {
        vertices.push(prev_node.position.into())?;
        path_len += 1;
    }

    Ok(path_len)
}

fn find_fallback_junction(endpoint: Point, ends: &[Point]) -> Point {
    let mut min_dist = endpoint.manhatten_distance_to(ends[0]);
    let mut min_end = ends[0];

    for &end in &ends[1..] {
        let dist = endpoint.manhatten_distance_to(end);
        if dist < min_dist {
            min_dist = dist;
            min_end = end;
        }
    }

    min_end
}

fn push_fallback_vertices(
    start: Point,
    end: Point,
    start_dirs: Directions,
    vertices: &mut Array<Vertex>,
) -> Result<(usize, Direction), ()> {
    let mut path_len = 0usize;

    vertices.push(start.into())?;
    path_len += 1;

    let (middle, dir) = if start_dirs.intersects(Directions::X) {
        let middle = Point {
            x: end.x,
            y: start.y,
        };

        let dir = if end.y < start.y {
            Direction::NegY
        } else {
            Direction::PosY
        };

        (middle, dir)
    } else {
        let middle = Point {
            x: end.x,
            y: start.y,
        };

        let dir = if end.x < start.x {
            Direction::NegX
        } else {
            Direction::PosX
        };

        (middle, dir)
    };

    if (middle != start) && (middle != end) {
        vertices.push(middle.into())?;
        path_len += 1;
    }

    vertices.push(end.into())?;
    path_len += 1;

    Ok((path_len, dir))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoutingError {
    NotEnoughEndpoints,
    VertexBufferOverflow,
    WireViewBufferOverflow,
    InvalidPoint,
}

fn route_root_wire<WaypointList, WaypointListRef>(
    graph: &GraphData,
    path_finder: &mut PathFinder,
    root_start: Endpoint<WaypointList>,
    root_end: Endpoint<WaypointList>,
    vertices: &mut Array<Vertex>,
    wire_views: &mut Array<WireView>,
    ends: &mut Vec<Point>,
    centering_candidates: &mut Vec<CenteringCandidate>,
) -> Result<u32, RoutingError>
where
    WaypointListRef: ?Sized,
    for<'a> &'a WaypointListRef: IntoIterator<Item: Borrow<Point>>,
    WaypointList: Borrow<WaypointListRef>,
{
    let mut wire_count = 0;

    let waypoints = root_start
        .waypoints
        .borrow()
        .into_iter()
        .chain(root_end.waypoints.borrow());

    let (last_waypoint, last_waypoint_dir) =
        match path_finder.find_path(graph, root_start.position, None, waypoints, true) {
            PathFindResult::Found(path) => {
                let path_len = push_vertices(path, graph, vertices, ends, centering_candidates)
                    .map_err(|_| RoutingError::VertexBufferOverflow)?;

                if path_len < 2 {
                    (root_start.position, None)
                } else {
                    wire_views
                        .push(WireView::new(path_len, false).expect("path too long"))
                        .map_err(|_| RoutingError::WireViewBufferOverflow)?;

                    let (last, head) = path.nodes().split_last().unwrap();
                    let prev_last = head.last().unwrap();

                    wire_count += 1;
                    (last.position, prev_last.bend_direction)
                }
            }
            PathFindResult::NotFound => {
                ends.push(root_start.position);
                (root_start.position, None)
            }
            PathFindResult::InvalidStartPoint | PathFindResult::InvalidEndPoint => {
                return Err(RoutingError::InvalidPoint);
            }
        };

    let path_len = match path_finder.find_path(
        graph,
        last_waypoint,
        last_waypoint_dir,
        [root_end.position],
        false,
    ) {
        PathFindResult::Found(path) => {
            push_vertices(path, graph, vertices, ends, centering_candidates)
                .map_err(|_| RoutingError::VertexBufferOverflow)?
        }
        PathFindResult::NotFound => {
            let root_end_node = &graph.nodes[graph.find_node(root_end.position).unwrap()];
            let (path_len, _) = push_fallback_vertices(
                root_end.position,
                last_waypoint,
                root_end_node.legal_directions,
                vertices,
            )
            .map_err(|_| RoutingError::VertexBufferOverflow)?;

            assert!(path_len >= 2);
            ends.push(root_end.position);

            path_len
        }
        PathFindResult::InvalidStartPoint | PathFindResult::InvalidEndPoint => {
            return Err(RoutingError::InvalidPoint);
        }
    };

    wire_views
        .push(WireView::new(path_len, false).expect("path too long"))
        .map_err(|_| RoutingError::WireViewBufferOverflow)?;

    Ok(wire_count + 1)
}

#[derive(Debug, Clone)]
pub(crate) enum JunctionKind {
    // A junction already has 2 connections from the root, so at most 2 other connections can be made.
    Single {
        vertex_index: usize,
        inbound_dir: Direction,
    },
    Double {
        vertex_index: [usize; 2],
        inbound_dir: [Direction; 2],
    },
    /// The junction is in a state the centering algorithm cannot deal with, so ignore it.
    Degenerate,
}

impl JunctionKind {
    fn iter(&self) -> impl Iterator<Item = (usize, Direction)> {
        enum Iter {
            Single(Option<(usize, Direction)>),
            Double([Option<(usize, Direction)>; 2]),
            Degenerate,
        }

        impl Iterator for Iter {
            type Item = (usize, Direction);

            fn next(&mut self) -> Option<Self::Item> {
                match self {
                    Iter::Single(opt) => opt.take(),
                    Iter::Double([opt1, opt2]) => opt1.take().or_else(|| opt2.take()),
                    Iter::Degenerate => None,
                }
            }
        }

        match self {
            &JunctionKind::Single {
                vertex_index,
                inbound_dir,
            } => Iter::Single(Some((vertex_index, inbound_dir))),
            JunctionKind::Double {
                vertex_index,
                inbound_dir,
            } => Iter::Double([
                Some((vertex_index[0], inbound_dir[0])),
                Some((vertex_index[1], inbound_dir[1])),
            ]),
            JunctionKind::Degenerate => Iter::Degenerate,
        }
    }
}

pub(crate) type JunctionMap = HashMap<Point, JunctionKind>;

fn insert_junction(
    junctions: &mut JunctionMap,
    position: Point,
    vertex_index: usize,
    inbound_dir: Direction,
) {
    use std::collections::hash_map::Entry;

    match junctions.entry(position) {
        Entry::Vacant(entry) => {
            entry.insert(JunctionKind::Single {
                vertex_index,
                inbound_dir,
            });
        }
        Entry::Occupied(mut entry) => {
            let kind = entry.get_mut();

            match *kind {
                JunctionKind::Single {
                    vertex_index: prev_vertex_index,
                    inbound_dir: prev_inbound_dir,
                } => {
                    *kind = JunctionKind::Double {
                        vertex_index: [prev_vertex_index, vertex_index],
                        inbound_dir: [prev_inbound_dir, inbound_dir],
                    };
                }
                JunctionKind::Double { .. } => {
                    // With normal routing this is impossible, because it requires routing
                    // on top of an existing wire that should have been connected to instead.
                    // However if a wire cannot be routed it ignores geometry so there is
                    // a small chance for it to happen.
                    *kind = JunctionKind::Degenerate;
                }
                JunctionKind::Degenerate => (),
            }
        }
    }
}

fn route_branch_wires<EndpointList, WaypointList, WaypointListRef>(
    graph: &GraphData,
    path_finder: &mut PathFinder,
    endpoints: EndpointList,
    root_start: Endpoint<WaypointList>,
    root_end: Endpoint<WaypointList>,
    vertices: &mut Array<Vertex>,
    wire_views: &mut Array<WireView>,
    ends: &mut Vec<Point>,
    centering_candidates: &mut Vec<CenteringCandidate>,
    junctions: &mut JunctionMap,
) -> Result<u32, RoutingError>
where
    WaypointListRef: ?Sized,
    for<'a> &'a WaypointListRef: IntoIterator<Item: Borrow<Point>>,
    WaypointList: Borrow<WaypointListRef>,
    EndpointList: Clone + Iterator<Item: Borrow<Endpoint<WaypointList>>>,
{
    let mut wire_count = 0;

    for endpoint in endpoints {
        let endpoint = endpoint.borrow();

        if (endpoint.position != root_start.position) && (endpoint.position != root_end.position) {
            let (last_waypoint, last_waypoint_dir) = match path_finder.find_path(
                graph,
                endpoint.position,
                None,
                endpoint.waypoints.borrow(),
                true,
            ) {
                PathFindResult::Found(path) => {
                    let path_len = push_vertices(path, graph, vertices, ends, centering_candidates)
                        .map_err(|_| RoutingError::VertexBufferOverflow)?;

                    if path_len < 2 {
                        (endpoint.position, None)
                    } else {
                        wire_views
                            .push(WireView::new(path_len, false).expect("path too long"))
                            .map_err(|_| RoutingError::WireViewBufferOverflow)?;

                        let (last, head) = path.nodes().split_last().unwrap();
                        let prev_last = head.last().unwrap();

                        wire_count += 1;
                        (last.position, prev_last.bend_direction)
                    }
                }
                PathFindResult::NotFound => (endpoint.position, None),
                PathFindResult::InvalidStartPoint | PathFindResult::InvalidEndPoint => {
                    return Err(RoutingError::InvalidPoint);
                }
            };

            let path_len = match path_finder.find_path(
                graph,
                last_waypoint,
                last_waypoint_dir,
                ends.iter().copied(),
                false,
            ) {
                PathFindResult::Found(path) => {
                    let path_len = push_vertices(path, graph, vertices, ends, centering_candidates)
                        .map_err(|_| RoutingError::VertexBufferOverflow)?;

                    if path_len < 2 {
                        continue;
                    }

                    let (last, head) = path.nodes().split_last().unwrap();
                    let prev_last = head.last().unwrap();
                    insert_junction(
                        junctions,
                        last.position,
                        vertices.len - 1,
                        prev_last.bend_direction.unwrap(),
                    );

                    path_len
                }
                PathFindResult::NotFound => {
                    let junction_pos = find_fallback_junction(endpoint.position, ends);
                    let endpoint_node = &graph.nodes[graph.find_node(endpoint.position).unwrap()];
                    let (path_len, junction_dir) = push_fallback_vertices(
                        endpoint.position,
                        junction_pos,
                        endpoint_node.legal_directions,
                        vertices,
                    )
                    .map_err(|_| RoutingError::VertexBufferOverflow)?;

                    assert!(path_len >= 2);
                    insert_junction(junctions, junction_pos, vertices.len - 1, junction_dir);
                    ends.push(endpoint.position);

                    path_len
                }
                PathFindResult::InvalidStartPoint | PathFindResult::InvalidEndPoint => {
                    return Err(RoutingError::InvalidPoint);
                }
            };

            wire_views
                .push(WireView::new(path_len, true).expect("path too long"))
                .map_err(|_| RoutingError::WireViewBufferOverflow)?;

            wire_count += 1;
        }
    }

    Ok(wire_count)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ConnectionKind {
    Connected {
        through_anchor: bool,
        through_junction: bool,
    },
    Unconnected,
}

fn are_connected_vertically(
    graph: &GraphData,
    mut a: NodeIndex,
    b: NodeIndex,
    junctions: &JunctionMap,
) -> ConnectionKind {
    let node_a = &graph.nodes[a];
    let node_b = &graph.nodes[b];

    let dir = if node_a.position.y < node_b.position.y {
        Direction::PosY
    } else {
        assert!(node_a.position.y > node_b.position.y);
        Direction::NegY
    };

    let mut through_anchor = node_a.is_anchor || node_b.is_anchor;
    let mut through_junction =
        junctions.contains_key(&node_a.position) || junctions.contains_key(&node_b.position);

    a = node_a.neighbors[dir];
    while a != INVALID_NODE_INDEX {
        if a == b {
            return ConnectionKind::Connected {
                through_anchor,
                through_junction,
            };
        }

        let node = &graph.nodes[a];
        if node.is_anchor {
            through_anchor = true;
        }
        match junctions.get(&node.position) {
            Some(JunctionKind::Degenerate) => {
                through_anchor = true;
                through_junction = true;
            }
            Some(_) => through_junction = true,
            _ => (),
        }

        a = node.neighbors[dir];
    }

    ConnectionKind::Unconnected
}

fn are_connected_horizontally(
    graph: &GraphData,
    mut a: NodeIndex,
    b: NodeIndex,
    junctions: &JunctionMap,
) -> ConnectionKind {
    let node_a = &graph.nodes[a];
    let node_b = &graph.nodes[b];

    let dir = if node_a.position.x < node_b.position.x {
        Direction::PosX
    } else {
        assert!(node_a.position.x > node_b.position.x);
        Direction::NegX
    };

    let mut through_anchor = node_a.is_anchor || node_b.is_anchor;
    let mut through_junction =
        junctions.contains_key(&node_a.position) || junctions.contains_key(&node_b.position);

    a = node_a.neighbors[dir];
    while a != INVALID_NODE_INDEX {
        if a == b {
            return ConnectionKind::Connected {
                through_anchor,
                through_junction,
            };
        }

        let node = &graph.nodes[a];
        if node.is_anchor {
            through_anchor = true;
        }
        match junctions.get(&node.position) {
            Some(JunctionKind::Degenerate) => {
                through_anchor = true;
                through_junction = true;
            }
            Some(_) => through_junction = true,
            _ => (),
        }

        a = node.neighbors[dir];
    }

    ConnectionKind::Unconnected
}

#[derive(Debug, Clone, Copy)]
enum NudgeOffset {
    None,
    Horizontal(f32),
    Vertical(f32),
}

fn center_in_alley(
    graph: &GraphData,
    node_a_index: NodeIndex,
    node_b_index: NodeIndex,
    vertex_index: usize,
    vertices: &mut Array<Vertex>,
    junctions: &JunctionMap,
) -> NudgeOffset {
    let node_a = &graph.nodes[node_a_index];
    let node_b = &graph.nodes[node_b_index];

    if node_a.position.x == node_b.position.x {
        match are_connected_vertically(graph, node_a_index, node_b_index, junctions) {
            ConnectionKind::Connected {
                through_junction, ..
            } => {
                if through_junction {
                    return NudgeOffset::None;
                }
            }
            ConnectionKind::Unconnected => panic!("wire segment not connected in graph"),
        }

        let mut min_x = node_a.position.x;
        let mut max_x = node_a.position.x;
        let mut x_min_cap = None;
        let mut x_max_cap = None;

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

            match are_connected_vertically(graph, next_a_index, next_b_index, junctions) {
                ConnectionKind::Connected {
                    through_anchor,
                    through_junction,
                } => {
                    min_x = current_node_a.position.x;

                    if through_junction && x_min_cap.is_none() {
                        x_min_cap = Some(current_node_a.position.x);
                    }

                    if through_anchor {
                        break;
                    } else {
                        continue;
                    }
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

            match are_connected_vertically(graph, next_a_index, next_b_index, junctions) {
                ConnectionKind::Connected {
                    through_anchor,
                    through_junction,
                } => {
                    max_x = current_node_a.position.x;

                    if through_junction && x_max_cap.is_none() {
                        x_max_cap = Some(current_node_a.position.x);
                    }

                    if through_anchor {
                        break;
                    } else {
                        continue;
                    }
                }
                ConnectionKind::Unconnected => break,
            }
        }

        let vertex_start = vertex_index;
        let vertex_end = vertex_start + 1;
        let [vertex_a, vertex_b] = &mut vertices[vertex_start..=vertex_end] else {
            panic!("invalid vertex offset");
        };

        let center_x = ((min_x + max_x) as f32 / 2.0).clamp(
            x_min_cap.unwrap_or(min_x) as f32,
            x_max_cap.unwrap_or(max_x) as f32,
        );
        vertex_a.x = center_x;
        vertex_b.x = center_x;

        NudgeOffset::Horizontal(center_x - (node_a.position.x as f32))
    } else {
        assert_eq!(node_a.position.y, node_b.position.y);

        match are_connected_horizontally(graph, node_a_index, node_b_index, junctions) {
            ConnectionKind::Connected {
                through_junction, ..
            } => {
                if through_junction {
                    return NudgeOffset::None;
                }
            }
            ConnectionKind::Unconnected => panic!("wire segment not connected in graph"),
        }

        let mut min_y = node_a.position.y;
        let mut max_y = node_a.position.y;
        let mut y_min_cap = None;
        let mut y_max_cap = None;

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

            match are_connected_horizontally(graph, next_a_index, next_b_index, junctions) {
                ConnectionKind::Connected {
                    through_anchor,
                    through_junction,
                } => {
                    min_y = current_node_a.position.y;

                    if through_junction && y_min_cap.is_none() {
                        y_min_cap = Some(current_node_a.position.y);
                    }

                    if through_anchor {
                        break;
                    } else {
                        continue;
                    }
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

            match are_connected_horizontally(graph, next_a_index, next_b_index, junctions) {
                ConnectionKind::Connected {
                    through_anchor,
                    through_junction,
                } => {
                    max_y = current_node_a.position.y;

                    if through_junction && y_max_cap.is_none() {
                        y_max_cap = Some(current_node_a.position.y);
                    }

                    if through_anchor {
                        break;
                    } else {
                        continue;
                    }
                }
                ConnectionKind::Unconnected => break,
            }
        }

        let vertex_start = vertex_index;
        let vertex_end = vertex_start + 1;
        let [vertex_a, vertex_b] = &mut vertices[vertex_start..=vertex_end] else {
            panic!("invalid vertex offset");
        };

        let center_y = ((min_y + max_y) as f32 / 2.0).clamp(
            y_min_cap.unwrap_or(min_y) as f32,
            y_max_cap.unwrap_or(max_y) as f32,
        );
        vertex_a.y = center_y;
        vertex_b.y = center_y;

        NudgeOffset::Vertical(center_y - (node_a.position.y as f32))
    }
}

fn center_wires(
    centering_candidates: &[CenteringCandidate],
    graph: &GraphData,
    vertices: &mut Array<Vertex>,
    junctions: &JunctionMap,
) {
    for centering_candidate in centering_candidates {
        let offset = center_in_alley(
            graph,
            centering_candidate.node_a,
            centering_candidate.node_b,
            centering_candidate.vertex_index as usize,
            vertices,
            junctions,
        );

        if matches!(offset, NudgeOffset::None) {
            continue;
        }

        let node_a = &graph.nodes[centering_candidate.node_a];
        let node_b = &graph.nodes[centering_candidate.node_b];

        for (&junction_point, junction_kind) in junctions {
            for (junction_vertex, junction_dir) in junction_kind.iter() {
                let junction_vertex = junction_vertex as usize;

                match offset {
                    NudgeOffset::None => unreachable!(),
                    NudgeOffset::Horizontal(offset) => {
                        assert_eq!(node_a.position.x, node_b.position.x);

                        let min_y = node_a.position.y.min(node_b.position.y);
                        let max_y = node_a.position.y.max(node_b.position.y);

                        if (junction_point.x == node_a.position.x)
                            && matches!(junction_dir, Direction::NegX | Direction::PosX)
                        {
                            if (junction_point.y >= min_y) && (junction_point.y <= max_y) {
                                vertices[junction_vertex].x += offset;
                            }
                        }

                        if junction_point.x == (node_a.position.x + (offset as i32)) {
                            match junction_dir {
                                Direction::PosY => {
                                    if junction_point.y == max_y {
                                        vertices[junction_vertex].y = min_y as f32;
                                    }
                                }
                                Direction::NegY => {
                                    if junction_point.y == min_y {
                                        vertices[junction_vertex].y = max_y as f32;
                                    }
                                }
                                _ => (),
                            }
                        }
                    }
                    NudgeOffset::Vertical(offset) => {
                        assert_eq!(node_a.position.y, node_b.position.y);

                        let min_x = node_a.position.x.min(node_b.position.x);
                        let max_x = node_a.position.x.max(node_b.position.x);

                        if (junction_point.y == node_a.position.y)
                            && matches!(junction_dir, Direction::NegY | Direction::PosY)
                        {
                            if (junction_point.x >= min_x) && (junction_point.x <= max_x) {
                                vertices[junction_vertex].y += offset;
                            }
                        }

                        if junction_point.y == (node_a.position.y + (offset as i32)) {
                            match junction_dir {
                                Direction::PosX => {
                                    if junction_point.x == max_x {
                                        vertices[junction_vertex].x = min_x as f32;
                                    }
                                }
                                Direction::NegX => {
                                    if junction_point.x == min_x {
                                        vertices[junction_vertex].x = max_x as f32;
                                    }
                                }
                                _ => (),
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Endpoint<WaypointList> {
    pub position: Point,
    pub waypoints: WaypointList,
}

pub(crate) fn connect_net<EndpointList, WaypointList, WaypointListRef>(
    graph: &Graph,
    endpoints: EndpointList,
    vertex_base_offset: usize,
    wire_base_offset: usize,
    vertices: &mut Array<Vertex>,
    wire_views: &mut Array<WireView>,
    net_view: &mut MaybeUninit<NetView>,
    ends: &mut Vec<Point>,
    centering_candidates: &mut Vec<CenteringCandidate>,
    junctions: &mut JunctionMap,
    perform_centering: bool,
) -> Result<(), RoutingError>
where
    WaypointListRef: ?Sized,
    for<'a> &'a WaypointListRef: IntoIterator<Item: Borrow<Point>>,
    WaypointList: Clone + Borrow<WaypointListRef>,
    EndpointList: Clone + Iterator<Item: Borrow<Endpoint<WaypointList>>>,
{
    let path_finder = &mut *graph.path_finder.get_or_default().borrow_mut();
    let (root_start, root_end) =
        pick_root_path(endpoints.clone()).map_err(|_| RoutingError::NotEnoughEndpoints)?;

    ends.clear();
    centering_candidates.clear();
    junctions.clear();

    let wire_offset = (wire_base_offset + wire_views.len)
        .try_into()
        .expect("too many wires");
    let vertex_offset = (vertex_base_offset + vertices.len)
        .try_into()
        .expect("too many vertices");

    let root_wire_count = route_root_wire(
        &graph.data,
        path_finder,
        root_start.clone(),
        root_end.clone(),
        vertices,
        wire_views,
        ends,
        centering_candidates,
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
        centering_candidates,
        junctions,
    )?;

    if perform_centering {
        center_wires(&centering_candidates, &graph.data, vertices, junctions);
    }

    net_view.write(NetView {
        wire_offset,
        wire_count: root_wire_count + branch_wire_count,
        vertex_offset,
    });

    Ok(())
}
