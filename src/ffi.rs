#![allow(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

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
    anchors: *const Anchor,
    anchor_count: usize,
    bounding_boxes: *const BoundingBox,
    bounding_box_count: usize,
    minimal: bool,
) -> Result {
    if graph.is_null() || anchors.is_null() || bounding_boxes.is_null() {
        return Result::NullPointerError;
    }

    let graph = unsafe { &mut *graph };
    let anchors = unsafe { std::slice::from_raw_parts(anchors, anchor_count) };
    let bounding_boxes = unsafe { std::slice::from_raw_parts(bounding_boxes, bounding_box_count) };
    graph.build(anchors, bounding_boxes, minimal);

    Result::Success
}

/// Gets the nodes in a graph.
///
/// **Parameters**  
/// `graph`: The graph to get the nodes of.  
/// `[out] nodes`: The list of nodes in the graph.  
/// `[out] node_count`: The number of elements in `nodes`.
///
/// **Returns**  
/// `RT_RESULT_SUCCESS`: The operation completed successfully.  
/// `RT_RESULT_NULL_POINTER_ERROR`: `graph`, `nodes` or `node_count` was `NULL`.
#[no_mangle]
#[must_use]
pub unsafe extern "C" fn RT_graph_get_nodes(
    graph: *const Graph,
    nodes: *mut *const Node,
    node_count: *mut usize,
) -> Result {
    if graph.is_null() || nodes.is_null() || node_count.is_null() {
        return Result::NullPointerError;
    }

    let graph = unsafe { &*graph };
    unsafe {
        nodes.write(graph.nodes().as_ptr());
        node_count.write(graph.nodes().len());
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

fn pick_root_path(endpoints: &[Endpoint], first: EndpointIndex) -> (EndpointIndex, EndpointIndex) {
    let mut max_dist = 0;
    let mut max_pair = (INVALID_ENDPOINT_INDEX, INVALID_ENDPOINT_INDEX);

    let mut a_index = first;
    while a_index != INVALID_ENDPOINT_INDEX {
        let a = endpoints[a_index as usize];

        let mut b_index = a.next;
        while b_index != INVALID_ENDPOINT_INDEX {
            let b = endpoints[b_index as usize];

            let dist = a.position.manhatten_distance_to(b.position);
            if dist > max_dist {
                max_dist = dist;
                max_pair = (a_index, b_index);
            }

            b_index = b.next;
        }

        a_index = a.next;
    }

    max_pair
}

fn push_vertices(
    vertices: &mut Array<Vertex>,
    path: impl Iterator<Item = Point>,
) -> std::result::Result<u16, Result> {
    let mut path_len = 0usize;

    for point in path {
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
    net: &Net,
    endpoints: &[Endpoint],
    waypoints: &[Waypoint],
    root_start: EndpointIndex,
    root_end: EndpointIndex,
    vertices: &mut Array<Vertex>,
    wire_views: &mut Array<WireView>,
    ends: &mut Vec<Point>,
) -> std::result::Result<u32, Result> {
    let mut wire_count = 0;

    let mut prev_waypoint = endpoints[root_start as usize].position;
    let mut waypoint_index = net.first_waypoint;
    while waypoint_index != INVALID_WAYPOINT_INDEX {
        let waypoint = waypoints[waypoint_index as usize];

        match path_finder.find_path(graph, prev_waypoint, &[waypoint.position]) {
            PathFindResult::Found(path) => {
                ends.extend(path.iter());
                let path_len = push_vertices(vertices, path.iter_pruned())?;
                push_wire_view(wire_views, path_len)?;
            }
            PathFindResult::NotFound => {
                let path = [waypoint.position, prev_waypoint];
                ends.extend_from_slice(&path);
                push_vertices(vertices, path.iter().copied())?;
                push_wire_view(wire_views, 2)?;
            }
            PathFindResult::InvalidStartPoint | PathFindResult::InvalidEndPoint => {
                return Err(Result::InvalidOperationError);
            }
        }

        prev_waypoint = waypoint.position;
        waypoint_index = waypoint.next;
        wire_count += 1;
    }

    let waypoint = endpoints[root_end as usize].position;
    match path_finder.find_path(graph, prev_waypoint, &[waypoint]) {
        PathFindResult::Found(path) => {
            ends.extend(path.iter());
            let path_len = push_vertices(vertices, path.iter_pruned())?;
            push_wire_view(wire_views, path_len)?;
        }
        PathFindResult::NotFound => {
            let path = [waypoint, prev_waypoint];
            ends.extend_from_slice(&path);
            push_vertices(vertices, path.iter().copied())?;
            push_wire_view(wire_views, 2)?;
        }
        PathFindResult::InvalidStartPoint | PathFindResult::InvalidEndPoint => {
            return Err(Result::InvalidOperationError);
        }
    }

    Ok(wire_count + 1)
}

fn route_branch_wires(
    graph: &GraphData,
    path_finder: &mut PathFinder,
    net: &Net,
    endpoints: &[Endpoint],
    root_start: EndpointIndex,
    root_end: EndpointIndex,
    vertices: &mut Array<Vertex>,
    wire_views: &mut Array<WireView>,
    ends: &mut Vec<Point>,
) -> std::result::Result<u32, Result> {
    let mut wire_count = 0;

    let mut endpoint_index = net.first_endpoint;
    while endpoint_index != INVALID_ENDPOINT_INDEX {
        let endpoint = endpoints[endpoint_index as usize];

        if (endpoint_index != root_start) && (endpoint_index != root_end) {
            match path_finder.find_path(graph, endpoint.position, ends) {
                PathFindResult::Found(path) => {
                    ends.extend(path.iter());
                    let path_len = push_vertices(vertices, path.iter_pruned())?;
                    push_wire_view(wire_views, path_len)?;
                }
                PathFindResult::NotFound => {
                    let path = [endpoint.position, endpoints[root_start as usize].position];
                    ends.extend_from_slice(&path);
                    push_vertices(vertices, path.iter().copied())?;
                    push_wire_view(wire_views, 2)?;
                }
                PathFindResult::InvalidStartPoint | PathFindResult::InvalidEndPoint => {
                    return Err(Result::InvalidOperationError);
                }
            }

            wire_count += 1;
        }

        endpoint_index = endpoint.next;
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

            let (root_start, root_end) = pick_root_path(endpoints, net.first_endpoint);
            if (root_start == INVALID_ENDPOINT_INDEX) || (root_end == INVALID_ENDPOINT_INDEX) {
                return Err(Result::InvalidArgumentError);
            }

            let root_wire_count = route_root_wire(
                &graph.data,
                path_finder,
                net,
                endpoints,
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
                net,
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
