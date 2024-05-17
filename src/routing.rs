use crate::graph::{NodeIndex, INVALID_NODE_INDEX};
use crate::*;
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

pub(crate) struct CenteringCandidate {
    node_a: NodeIndex,
    node_b: NodeIndex,
    vertex_index: u32,
}

fn push_vertices(
    graph: &GraphData,
    vertices: &mut Array<Vertex>,
    path: &Path,
    ends: &mut Vec<Point>,
    centering_candidates: &mut Vec<CenteringCandidate>,
) -> Result<u16, ()> {
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

    Ok(path_len.try_into().expect("path too long"))
}

fn push_fallback_vertices(
    points: impl IntoIterator<Item = Point>,
    vertices: &mut Array<Vertex>,
    ends: &mut Vec<Point>,
) -> Result<u16, ()> {
    let mut path_len = 0usize;

    for point in points {
        vertices.push(point.into())?;
        ends.push(point);
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

fn route_root_wire<I>(
    graph: &GraphData,
    path_finder: &mut PathFinder,
    waypoints: I,
    root_start: Point,
    root_end: Point,
    vertices: &mut Array<Vertex>,
    wire_views: &mut Array<WireView>,
    ends: &mut Vec<Point>,
    centering_candidates: &mut Vec<CenteringCandidate>,
) -> Result<u32, RoutingError>
where
    I: IntoIterator<Item = Point>,
{
    let path_tail = match path_finder.find_path(graph, root_start, None, waypoints, true) {
        PathFindResult::Found(path) => {
            let path_len = push_vertices(graph, vertices, path, ends, centering_candidates)
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

    let start = path_tail.map(|(last, _)| last).unwrap_or(root_start);
    let path_len = match path_finder.find_path(
        graph,
        start,
        path_tail.and_then(|(_, dir)| dir),
        [root_end],
        false,
    ) {
        PathFindResult::Found(path) => {
            push_vertices(graph, vertices, path, ends, centering_candidates)
                .map_err(|_| RoutingError::VertexBufferOverflow)?
        }
        PathFindResult::NotFound => push_fallback_vertices([start, root_end], vertices, ends)
            .map_err(|_| RoutingError::VertexBufferOverflow)?,
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
    ends: &mut Vec<Point>,
    centering_candidates: &mut Vec<CenteringCandidate>,
    junctions: &mut HashMap<Point, (usize, Direction)>,
) -> Result<u32, RoutingError>
where
    I: IntoIterator<Item = Point>,
{
    let mut wire_count = 0;

    for endpoint in endpoints {
        if (endpoint != root_start) && (endpoint != root_end) {
            let path_len =
                match path_finder.find_path(graph, endpoint, None, ends.iter().copied(), false) {
                    PathFindResult::Found(path) => {
                        let path_len =
                            push_vertices(graph, vertices, path, ends, centering_candidates)
                                .map_err(|_| RoutingError::VertexBufferOverflow)?;

                        assert!(path_len >= 2);
                        let (last, head) = path.nodes().split_last().unwrap();
                        let prev_last = head.last().unwrap();
                        junctions.insert(
                            last.position,
                            (vertices.len - 1, prev_last.bend_direction.unwrap()),
                        );

                        path_len
                    }
                    PathFindResult::NotFound => {
                        push_fallback_vertices([endpoint, root_start], vertices, ends)
                            .map_err(|_| RoutingError::VertexBufferOverflow)?
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
    junctions: &HashMap<Point, (usize, Direction)>,
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
        if junctions.contains_key(&node.position) {
            through_junction = true;
        }

        a = node.neighbors[dir];
    }

    ConnectionKind::Unconnected
}

fn are_connected_horizontally(
    graph: &GraphData,
    mut a: NodeIndex,
    b: NodeIndex,
    junctions: &HashMap<Point, (usize, Direction)>,
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
        if junctions.contains_key(&node.position) {
            through_junction = true;
        }

        a = node.neighbors[dir];
    }

    ConnectionKind::Unconnected
}

#[derive(Debug, Clone, Copy)]
enum NudgeOffset {
    Horizontal(f32),
    Vertical(f32),
}

fn center_in_alley(
    graph: &GraphData,
    node_a: &Node,
    node_b: &Node,
    vertex_index: usize,
    vertices: &mut Array<Vertex>,
    junctions: &HashMap<Point, (usize, Direction)>,
) -> NudgeOffset {
    if node_a.position.x == node_b.position.x {
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
    junctions: &HashMap<Point, (usize, Direction)>,
) {
    for centering_candidate in centering_candidates {
        let node_a = &graph.nodes[centering_candidate.node_a];
        let node_b = &graph.nodes[centering_candidate.node_b];

        let offset = center_in_alley(
            graph,
            node_a,
            node_b,
            centering_candidate.vertex_index as usize,
            vertices,
            junctions,
        );

        for (&junction_point, &(junction_vertex, junction_dir)) in junctions {
            match offset {
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

pub(crate) fn connect_net<EndpointList, WaypointList>(
    graph: &Graph,
    endpoints: EndpointList,
    waypoints: WaypointList,
    vertex_base_offset: usize,
    wire_base_offset: usize,
    vertices: &mut Array<Vertex>,
    wire_views: &mut Array<WireView>,
    net_view: &mut MaybeUninit<NetView>,
    ends: &mut Vec<Point>,
    centering_candidates: &mut Vec<CenteringCandidate>,
    junctions: &mut HashMap<Point, (usize, Direction)>,
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
        waypoints,
        root_start,
        root_end,
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

    center_wires(&centering_candidates, &graph.data, vertices, junctions);

    net_view.write(NetView {
        wire_offset,
        wire_count: root_wire_count + branch_wire_count,
        vertex_offset,
    });

    Ok(())
}
