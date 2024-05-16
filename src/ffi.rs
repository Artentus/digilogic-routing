#![allow(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use crate::graph::{NodeIndex, INVALID_NODE_INDEX};
use crate::*;
use rayon::prelude::*;
use std::mem::MaybeUninit;
use std::ops::Range;
use std::sync::atomic::{AtomicU16, Ordering};
use thread_local::ThreadLocal;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum Result {
    Success = 0,
    NullPointerError = 1,
    InvalidOperationError = 2,
    VertexBufferOverflowError = 3,
    WireViewBufferOverflowError = 4,
    UninitializedError = 5,
    InvalidArgumentError = 6,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Slice<T> {
    pub ptr: *const T,
    pub len: usize,
}

unsafe impl<T> Send for Slice<T> {}
unsafe impl<T> Sync for Slice<T> {}

#[allow(dead_code)]
impl<T> Slice<T> {
    #[inline]
    fn is_null(&self) -> bool {
        self.ptr.is_null() && (self.len != 0)
    }

    #[inline]
    unsafe fn subslice(&self, range: Range<usize>) -> Slice<T> {
        assert!(range.start <= range.end);
        assert!(range.end <= self.len);

        let len = range.end - range.start;
        let ptr = if len == 0 {
            std::ptr::null()
        } else {
            unsafe { self.ptr.add(range.start) }
        };

        Slice { ptr, len }
    }

    #[inline]
    unsafe fn as_ref<'a>(&self) -> &'a [T] {
        if self.len == 0 {
            &[]
        } else {
            unsafe { std::slice::from_raw_parts(self.ptr, self.len) }
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct MutSlice<T> {
    pub ptr: *mut T,
    pub len: usize,
}

unsafe impl<T> Send for MutSlice<T> {}
unsafe impl<T> Sync for MutSlice<T> {}

#[allow(dead_code)]
impl<T> MutSlice<T> {
    #[inline]
    fn is_null(self) -> bool {
        self.ptr.is_null()
    }

    #[inline]
    unsafe fn subslice(&self, range: Range<usize>) -> Slice<T> {
        assert!(range.start <= range.end);
        assert!(range.end <= self.len);

        let len = range.end - range.start;
        let ptr = if len == 0 {
            std::ptr::null()
        } else {
            unsafe { self.ptr.add(range.start) }
        };

        Slice { ptr, len }
    }

    #[inline]
    unsafe fn subslice_mut(&mut self, range: Range<usize>) -> MutSlice<T> {
        assert!(range.start <= range.end);
        assert!(range.end <= self.len);

        let len = range.end - range.start;
        let ptr = if len == 0 {
            std::ptr::null_mut()
        } else {
            unsafe { self.ptr.add(range.start) }
        };

        MutSlice { ptr, len }
    }

    #[inline]
    unsafe fn as_ref<'a>(&self) -> &'a [T] {
        if self.len == 0 {
            &[]
        } else {
            unsafe { std::slice::from_raw_parts(self.ptr, self.len) }
        }
    }

    #[inline]
    unsafe fn as_uninit_mut<'a>(&mut self) -> &'a mut [MaybeUninit<T>] {
        if self.len == 0 {
            &mut []
        } else {
            unsafe { std::slice::from_raw_parts_mut(self.ptr as _, self.len) }
        }
    }
}

static NUM_CPUS: AtomicU16 = AtomicU16::new(0);

/// Initializes the thread pool.
///
/// **Returns**  
/// `RT_RESULT_SUCCESS`: The operation completed successfully.  
/// `RT_RESULT_INVALID_OPERATION_ERROR`: The function was called more than once.
#[no_mangle]
#[must_use]
pub unsafe extern "C" fn RT_init_thread_pool() -> Result {
    if NUM_CPUS.load(Ordering::Acquire) == 0 {
        let num_cpus: u16 = num_cpus::get().try_into().unwrap_or(u16::MAX);
        assert_ne!(num_cpus, 0);

        rayon::ThreadPoolBuilder::new()
            .num_threads(num_cpus as usize)
            .build_global()
            .expect("unable to initialize thread pool");

        NUM_CPUS.store(num_cpus, Ordering::Release);
        Result::Success
    } else {
        Result::InvalidOperationError
    }
}

/// Gets the number of threads in the pool.
///
/// **Parameters**  
/// `[out] thread_count`: The number of threads in the pool.
///
/// **Returns**  
/// `RT_RESULT_SUCCESS`: The operation completed successfully.  
/// `RT_RESULT_NULL_POINTER_ERROR`: `thread_count` was `NULL`.  
/// `RT_RESULT_UNINITIALIZED_ERROR`: The thread pool was not initialized yet.
#[no_mangle]
#[must_use]
pub unsafe extern "C" fn RT_get_thread_count(thread_count: *mut u16) -> Result {
    if thread_count.is_null() {
        return Result::NullPointerError;
    }

    let num_cpus = NUM_CPUS.load(Ordering::Acquire);
    if num_cpus > 0 {
        unsafe {
            thread_count.write(num_cpus);
        }

        Result::Success
    } else {
        Result::UninitializedError
    }
}

/// Creates a new graph.
///
/// **Parameters**  
/// `[out] graph`: The created graph.
///
/// **Returns**  
/// `RT_RESULT_SUCCESS`: The operation completed successfully.  
/// `RT_RESULT_NULL_POINTER_ERROR`: `graph` was `NULL`.
#[no_mangle]
#[must_use]
pub unsafe extern "C" fn RT_graph_new(graph: *mut *mut Graph) -> Result {
    if graph.is_null() {
        return Result::NullPointerError;
    }

    let ptr = Box::into_raw(Box::new(Graph::default()));
    unsafe {
        graph.write(ptr);
    }

    Result::Success
}

/// Builds a graph.
///
/// **Parameters**  
/// `graph`: The graph to build.  
/// `anchors`: A list of anchor points to build the graph from.  
/// `anchor_count`: The number of elements in `anchors`.  
/// `bounding_boxes`: A list of bounding boxes to build the graph from.  
/// `bounding_box_count`: The number of elements in `bounding_boxes`.  
/// `minimal`: Whether to spend more processing time to ensure the graph is minimal.
///
/// **Returns**  
/// `RT_RESULT_SUCCESS`: The operation completed successfully.  
/// `RT_RESULT_NULL_POINTER_ERROR`: `graph`, `anchor_points` or `bounding_boxes` was `NULL`.
#[no_mangle]
#[must_use]
pub unsafe extern "C" fn RT_graph_build(
    graph: *mut Graph,
    anchors: Slice<Anchor>,
    bounding_boxes: Slice<BoundingBox>,
    minimal: bool,
) -> Result {
    if graph.is_null() || anchors.is_null() || bounding_boxes.is_null() {
        return Result::NullPointerError;
    }

    let graph = unsafe { &mut *graph };
    let anchors = unsafe { anchors.as_ref() };
    let bounding_boxes = unsafe { bounding_boxes.as_ref() };
    graph.build(anchors, bounding_boxes, minimal);

    Result::Success
}

/// Gets the nodes in a graph.
///
/// **Parameters**  
/// `graph`: The graph to get the nodes of.  
/// `[out] nodes`: The list of nodes in the graph.
///
/// **Returns**  
/// `RT_RESULT_SUCCESS`: The operation completed successfully.  
/// `RT_RESULT_NULL_POINTER_ERROR`: `graph` or `nodes` was `NULL`.
#[no_mangle]
#[must_use]
pub unsafe extern "C" fn RT_graph_get_nodes(
    graph: *const Graph,
    nodes: *mut Slice<Node>,
) -> Result {
    if graph.is_null() || nodes.is_null() {
        return Result::NullPointerError;
    }

    let graph = unsafe { &*graph };
    unsafe {
        nodes.write(Slice {
            ptr: graph.nodes().as_ptr(),
            len: graph.nodes().len(),
        });
    }

    Result::Success
}

/// Finds the node at a specific position in the graph.
///
/// **Parameters**  
/// `graph`: The graph to find the node in.  
/// `position`: The position of the node to find.  
/// `[out] node_index`: The index of the node at the given position, or `RT_INVALID_NODE_INDEX` if none.
///
/// **Returns**  
/// `RT_RESULT_SUCCESS`: The operation completed successfully.  
/// `RT_RESULT_NULL_POINTER_ERROR`: `graph`, or `node_index` was `NULL`.
#[no_mangle]
#[must_use]
pub unsafe extern "C" fn RT_graph_find_node(
    graph: *const Graph,
    position: Point,
    node_index: *mut NodeIndex,
) -> Result {
    if graph.is_null() || node_index.is_null() {
        return Result::NullPointerError;
    }

    let graph = unsafe { &*graph };
    unsafe {
        node_index.write(graph.data.find_node(position).unwrap_or(INVALID_NODE_INDEX));
    }

    Result::Success
}

/// Frees a graph.
///
/// **Parameters**  
/// `graph`: The graph to free.
///
/// **Returns**  
/// `RT_RESULT_SUCCESS`: The operation completed successfully.  
/// `RT_RESULT_NULL_POINTER_ERROR`: `graph` was `NULL`.
#[no_mangle]
#[must_use]
pub unsafe extern "C" fn RT_graph_free(graph: *mut Graph) -> Result {
    if graph.is_null() {
        return Result::NullPointerError;
    }

    let graph = unsafe { Box::from_raw(graph) };
    std::mem::drop(graph);

    Result::Success
}

pub type EndpointIndex = u32;
pub type WaypointIndex = u32;

pub const INVALID_ENDPOINT_INDEX: EndpointIndex = u32::MAX;
pub const INVALID_WAYPOINT_INDEX: WaypointIndex = u32::MAX;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Endpoint {
    /// The position of the endpoint.
    pub position: Point,
    /// The next endpoint in the net, or `RT_INVALID_ENDPOINT_INDEX` if none.
    pub next: EndpointIndex,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Waypoint {
    /// The position of the waypoint.
    pub position: Point,
    /// The next waypoint in the net, or `RT_INVALID_WAYPOINT_INDEX` if none.
    pub next: WaypointIndex,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Net {
    /// The first endpoint of the net.
    pub first_endpoint: EndpointIndex,
    /// The first waypoint of the net, or `RT_INVALID_WAYPOINT_INDEX` if none.
    pub first_waypoint: WaypointIndex,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Vertex {
    /// The X coordinate of the vertex.
    pub x: f32,
    /// The Y coordinate of the vertex.
    pub y: f32,
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

struct Array<T> {
    ptr: *mut T,
    cap: usize,
    len: usize,
}

unsafe impl<T> Send for Array<T> {}
unsafe impl<T> Sync for Array<T> {}

impl<T> Array<T> {
    #[inline]
    fn from_mut_slice(slice: &mut MutSlice<T>, len: usize) -> Self {
        Self {
            ptr: slice.ptr,
            cap: slice.len,
            len,
        }
    }
}

#[derive(Clone, Copy)]
struct EndpointList<'a> {
    endpoints: &'a [Endpoint],
    first: EndpointIndex,
}

impl<'a> EndpointList<'a> {
    #[inline]
    fn new(endpoints: &'a [Endpoint], first: EndpointIndex) -> Self {
        Self { endpoints, first }
    }
}

#[derive(Clone)]
struct EndpointIter<'a> {
    endpoints: &'a [Endpoint],
    current: EndpointIndex,
}

impl<'a> IntoIterator for EndpointList<'a> {
    type Item = Point;
    type IntoIter = EndpointIter<'a>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        EndpointIter {
            endpoints: self.endpoints,
            current: self.first,
        }
    }
}

impl Iterator for EndpointIter<'_> {
    type Item = Point;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.current == INVALID_ENDPOINT_INDEX {
            return None;
        }

        let endpoint = self.endpoints[self.current as usize];
        self.current = endpoint.next;
        Some(endpoint.position)
    }
}

#[derive(Clone, Copy)]
struct WaypointList<'a> {
    waypoints: &'a [Waypoint],
    first: WaypointIndex,
}

impl<'a> WaypointList<'a> {
    #[inline]
    fn new(waypoints: &'a [Waypoint], first: WaypointIndex) -> Self {
        Self { waypoints, first }
    }
}

#[derive(Clone)]
struct WaypointIter<'a> {
    waypoints: &'a [Waypoint],
    current: WaypointIndex,
}

impl<'a> IntoIterator for WaypointList<'a> {
    type Item = Point;
    type IntoIter = WaypointIter<'a>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        WaypointIter {
            waypoints: self.waypoints,
            current: self.first,
        }
    }
}

impl Iterator for WaypointIter<'_> {
    type Item = Point;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.current == INVALID_WAYPOINT_INDEX {
            return None;
        }

        let waypoint = self.waypoints[self.current as usize];
        self.current = waypoint.next;
        Some(waypoint.position)
    }
}

fn pick_root_path(endpoints: EndpointList) -> Option<(Point, Point)> {
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
        None
    } else {
        Some(max_pair)
    }
}

fn push_vertex(vertices: &mut Array<Vertex>, point: Point) -> std::result::Result<(), Result> {
    let Some(new_len) = vertices.len.checked_add(1) else {
        return Err(Result::VertexBufferOverflowError);
    };
    if vertices.cap < new_len {
        return Err(Result::VertexBufferOverflowError);
    }

    unsafe {
        vertices.ptr.add(vertices.len).write(Vertex {
            x: point.x as f32,
            y: point.y as f32,
        });
    }

    vertices.len = new_len;
    Ok(())
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

    let mut through_anchor = false;
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

    let mut through_anchor = false;
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

fn center_in_alley(graph: &GraphData, node_a: &Node, a: &mut Point, node_b: &Node, b: &mut Point) {
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

        let center_x = (min_x + max_x) / 2;
        a.x = center_x;
        b.x = center_x;
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

        let center_y = (min_y + max_y) / 2;
        a.y = center_y;
        b.y = center_y;
    }
}

fn push_vertices(
    graph: &GraphData,
    vertices: &mut Array<Vertex>,
    path: impl IntoIterator<Item = PathNode>,
) -> std::result::Result<u16, Result> {
    let mut path_len = 0usize;

    let mut prev_prev_dir = None;
    let mut prev: Option<(PathNode, &Node)> = None;
    for mut path_node in path {
        let node = &graph.nodes[graph
            .find_node(path_node.position)
            .expect("invalid wire vertex")];

        if let Some((mut prev_path_node, prev_node)) = prev {
            if (prev_path_node.kind == PathNodeKind::Normal)
                && (path_node.kind == PathNodeKind::Normal)
                && (prev_prev_dir != Some(path_node.bend_direction.map(Direction::opposite)))
            {
                center_in_alley(
                    graph,
                    prev_node,
                    &mut prev_path_node.position,
                    node,
                    &mut path_node.position,
                );
            }

            push_vertex(vertices, prev_path_node.position)?;
            path_len += 1;
        }

        prev_prev_dir = prev.map(|(prev_path_node, _)| prev_path_node.bend_direction);
        prev = Some((path_node, node));
    }

    if let Some((prev_path_node, _)) = prev {
        push_vertex(vertices, prev_path_node.position)?;
        path_len += 1;
    }

    Ok(path_len.try_into().expect("path too long"))
}

fn push_wire_view(
    wire_views: &mut Array<WireView>,
    path_len: u16,
) -> std::result::Result<(), Result> {
    let Some(new_len) = wire_views.len.checked_add(1) else {
        return Err(Result::WireViewBufferOverflowError);
    };
    if wire_views.cap < new_len {
        return Err(Result::WireViewBufferOverflowError);
    }

    unsafe {
        wire_views.ptr.add(wire_views.len).write(WireView {
            vertex_count: path_len,
        });
    }

    wire_views.len = new_len;
    Ok(())
}

fn route_root_wire(
    graph: &GraphData,
    path_finder: &mut PathFinder,
    waypoints: WaypointList,
    root_start: Point,
    root_end: Point,
    vertices: &mut Array<Vertex>,
    wire_views: &mut Array<WireView>,
    ends: &mut Vec<Point>,
) -> std::result::Result<u32, Result> {
    match path_finder.find_path(
        graph,
        root_start,
        [root_end].into_iter().chain(waypoints),
        true,
    ) {
        PathFindResult::Found(path) => {
            ends.extend(path.iter().map(|path_node| path_node.position));
            let path_len = push_vertices(graph, vertices, path.iter_pruned())?;
            push_wire_view(wire_views, path_len)?;
        }
        PathFindResult::NotFound => {
            let path = [
                PathNode {
                    position: root_start,
                    kind: PathNodeKind::Start,
                    bend_direction: None,
                },
                PathNode {
                    position: root_end,
                    kind: PathNodeKind::End,
                    bend_direction: None,
                },
            ];
            ends.extend(path.iter().map(|path_node| path_node.position));
            push_vertices(graph, vertices, path)?;
            push_wire_view(wire_views, 2)?;
        }
        PathFindResult::InvalidStartPoint | PathFindResult::InvalidEndPoint => {
            return Err(Result::InvalidOperationError);
        }
    }

    Ok(1)
}

fn route_branch_wires(
    graph: &GraphData,
    path_finder: &mut PathFinder,
    endpoints: EndpointList,
    root_start: Point,
    root_end: Point,
    vertices: &mut Array<Vertex>,
    wire_views: &mut Array<WireView>,
    ends: &mut Vec<Point>,
) -> std::result::Result<u32, Result> {
    let mut wire_count = 0;

    for endpoint in endpoints {
        if (endpoint != root_start) && (endpoint != root_end) {
            match path_finder.find_path(graph, endpoint, ends.iter().copied(), false) {
                PathFindResult::Found(path) => {
                    ends.extend(path.iter().map(|path_node| path_node.position));
                    let path_len = push_vertices(graph, vertices, path.iter_pruned())?;
                    push_wire_view(wire_views, path_len)?;
                }
                PathFindResult::NotFound => {
                    let path = [
                        PathNode {
                            position: endpoint,
                            kind: PathNodeKind::Start,
                            bend_direction: None,
                        },
                        PathNode {
                            position: root_start,
                            kind: PathNodeKind::End,
                            bend_direction: None,
                        },
                    ];
                    ends.extend(path.iter().map(|path_node| path_node.position));
                    push_vertices(graph, vertices, path)?;
                    push_wire_view(wire_views, 2)?;
                }
                PathFindResult::InvalidStartPoint | PathFindResult::InvalidEndPoint => {
                    return Err(Result::InvalidOperationError);
                }
            }

            wire_count += 1;
        }
    }

    Ok(wire_count)
}

/// Connects nets in a graph.
///
/// **Parameters**  
/// `graph`: The graph to connect the nets in.  
/// `nets`: A list of nets to connect.  
/// `endpoints`: A list of net endpoints.  
/// `waypoints`: A list of net waypoints.  
/// `vertices`: A list to write the found vertices into.  
/// `wire_views`: A list to write the found wires into.  
/// `net_views`: A list to write the found nets into.
///
/// **Returns**  
/// `RT_RESULT_SUCCESS`: The operation completed successfully.  
/// `RT_RESULT_NULL_POINTER_ERROR`: `graph`, `nets.ptr`, `endpoints.ptr`, `waypoints.ptr`, `vertices.ptr`, `wire_views.ptr` or `net_views.ptr` was `NULL`.  
/// `RT_RESULT_INVALID_OPERATION_ERROR`: One of the paths had an invalid start or end point.  
/// `RT_RESULT_VERTEX_BUFFER_OVERFLOW_ERROR`: The capacity of `vertices` was too small to hold all vertices.  
/// `RT_RESULT_WIRE_VIEW_BUFFER_OVERFLOW_ERROR`: The capacity of `wire_views` was too small to hold all wire views.  
/// `RT_RESULT_UNINITIALIZED_ERROR`: The thread pool has not been initialized yet.  
/// `RT_RESULT_INVALID_ARGUMENT_ERROR`: `nets.len` was not equal to `net_views.len` or a net contained fewer than 2 endpoints.
#[no_mangle]
#[must_use]
pub unsafe extern "C" fn RT_graph_connect_nets(
    graph: *const Graph,
    nets: Slice<Net>,
    endpoints: Slice<Endpoint>,
    waypoints: Slice<Waypoint>,
    vertices: MutSlice<Vertex>,
    wire_views: MutSlice<WireView>,
    mut net_views: MutSlice<NetView>,
) -> Result {
    let num_cpus = NUM_CPUS.load(Ordering::Acquire);
    if num_cpus == 0 {
        return Result::UninitializedError;
    }
    assert_eq!(num_cpus as usize, rayon::current_num_threads());

    if graph.is_null()
        || nets.is_null()
        || endpoints.is_null()
        || waypoints.is_null()
        || vertices.is_null()
        || wire_views.is_null()
        || net_views.is_null()
    {
        return Result::NullPointerError;
    }

    if nets.len != net_views.len {
        return Result::InvalidArgumentError;
    }

    let graph = unsafe { &*graph };
    let nets = unsafe { nets.as_ref() };
    let endpoints = unsafe { endpoints.as_ref() };
    let waypoints = unsafe { waypoints.as_ref() };
    let net_views = unsafe { net_views.as_uninit_mut() };

    let vertices_per_thread = vertices.len / (num_cpus as usize);
    let wire_views_per_thread = wire_views.len / (num_cpus as usize);

    struct MutableThreadlocalData {
        vertices: Array<Vertex>,
        wire_views: Array<WireView>,
        ends: Vec<Point>,
    }

    struct ThreadlocalData {
        mutable: RefCell<MutableThreadlocalData>,
        vertex_offset: usize,
        wire_offset: usize,
    }

    let next_thread_index: AtomicU16 = AtomicU16::new(0);
    let threadlocal_data = ThreadLocal::new();

    let result = nets
        .par_iter()
        .zip(net_views.par_iter_mut())
        .try_for_each(|(net, net_view)| {
            let path_finder = &mut *graph.path_finder.get_or_default().borrow_mut();

            let threadlocal_data = threadlocal_data.get_or(|| {
                let thread_index = next_thread_index.fetch_add(1, Ordering::AcqRel);
                assert!(thread_index < num_cpus);
                let thread_index = thread_index as usize;

                let mut vertices = vertices;
                let vertices_start = thread_index * vertices_per_thread;
                let vertices_end = vertices_start + vertices_per_thread;
                let mut vertices = unsafe { vertices.subslice_mut(vertices_start..vertices_end) };
                let vertices = Array::from_mut_slice(&mut vertices, 0);

                let mut wire_views = wire_views;
                let wire_views_start = thread_index * wire_views_per_thread;
                let wire_views_end = wire_views_start + wire_views_per_thread;
                let mut wire_views =
                    unsafe { wire_views.subslice_mut(wire_views_start..wire_views_end) };
                let wire_views = Array::from_mut_slice(&mut wire_views, 0);

                ThreadlocalData {
                    mutable: RefCell::new(MutableThreadlocalData {
                        vertices,
                        wire_views,
                        ends: Vec::new(),
                    }),
                    vertex_offset: vertices_start,
                    wire_offset: wire_views_start,
                }
            });

            let ThreadlocalData {
                vertex_offset,
                wire_offset,
                ..
            } = *threadlocal_data;

            let MutableThreadlocalData {
                vertices,
                wire_views,
                ends,
            } = &mut *threadlocal_data.mutable.borrow_mut();

            ends.clear();
            let vertex_offset = vertex_offset + vertices.len;
            let wire_offset = wire_offset + wire_views.len;

            let endpoints = EndpointList::new(endpoints, net.first_endpoint);
            let waypoints = WaypointList::new(waypoints, net.first_waypoint);

            let Some((root_start, root_end)) = pick_root_path(endpoints) else {
                return Err(Result::InvalidArgumentError);
            };

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
                wire_offset: wire_offset.try_into().expect("too many wires"),
                wire_count: root_wire_count + branch_wire_count,
                vertex_offset: vertex_offset.try_into().expect("too many vertices"),
            });

            Ok(())
        });

    match result {
        Ok(_) => Result::Success,
        Err(err) => err,
    }
}
