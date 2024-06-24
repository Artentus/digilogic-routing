#![allow(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use crate::graph::{NodeIndex, INVALID_NODE_INDEX};
use crate::routing::{Array, CenteringCandidate, JunctionMap};
use crate::*;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::ffi::{c_char, c_void, CStr};
use std::fs::File;
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
    IoError = 7,
}

impl From<RoutingError> for Result {
    fn from(err: RoutingError) -> Self {
        match err {
            RoutingError::NotEnoughEndpoints => Result::InvalidArgumentError,
            RoutingError::VertexBufferOverflow => Result::VertexBufferOverflowError,
            RoutingError::WireViewBufferOverflow => Result::WireViewBufferOverflowError,
            RoutingError::InvalidPoint => Result::InvalidOperationError,
        }
    }
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

impl<'a, T> From<&'a [T]> for Slice<T> {
    fn from(value: &'a [T]) -> Self {
        Slice {
            ptr: value.as_ptr(),
            len: value.len(),
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

impl<'a, T> From<&'a mut [T]> for MutSlice<T> {
    fn from(value: &'a mut [T]) -> Self {
        MutSlice {
            ptr: value.as_mut_ptr(),
            len: value.len(),
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

/// Serializes a graph.
///
/// **Parameters**  
/// `graph`: The graph to serialize.  
/// `file_path`: The file to serialize the graph into.
///
/// **Returns**  
/// `RT_RESULT_SUCCESS`: The operation completed successfully.  
/// `RT_RESULT_NULL_POINTER_ERROR`: `graph` or `file_path` was `NULL`.  
/// `RT_RESULT_INVALID_OPERATION_ERROR`: The serialization failed.  
/// `RT_RESULT_INVALID_ARGUMENT_ERROR`: `file_path` did not contain legal UTF-8.  
/// `RT_RESULT_IO_ERROR`: An IO error occurred while writing to the file.
#[no_mangle]
#[must_use]
pub unsafe extern "C" fn RT_graph_serialize(
    graph: *const Graph,
    file_path: *const c_char,
) -> Result {
    if graph.is_null() || file_path.is_null() {
        return Result::NullPointerError;
    }

    let graph = unsafe { &*graph };
    let file_path = unsafe { CStr::from_ptr(file_path) };
    let Ok(file_path) = file_path.to_str() else {
        return Result::InvalidArgumentError;
    };

    let Ok(mut file) = File::create(file_path) else {
        return Result::IoError;
    };

    match rmp_serde::encode::write(&mut file, graph) {
        Ok(_) => Result::Success,
        Err(rmp_serde::encode::Error::InvalidValueWrite(_)) => Result::IoError,
        Err(_) => Result::InvalidOperationError,
    }
}

/// Deserializes a graph.
///
/// **Parameters**  
/// `[out] graph`: The deserialized graph.  
/// `file_path`: The file to deserialize the graph from.
///
/// **Returns**  
/// `RT_RESULT_SUCCESS`: The operation completed successfully.  
/// `RT_RESULT_NULL_POINTER_ERROR`: `graph` or `file_path` was `NULL`.  
/// `RT_RESULT_INVALID_OPERATION_ERROR`: The deserialization failed.  
/// `RT_RESULT_INVALID_ARGUMENT_ERROR`: `file_path` did not contain legal UTF-8.  
/// `RT_RESULT_IO_ERROR`: An IO error occurred while reading from the file.
#[no_mangle]
#[must_use]
pub unsafe extern "C" fn RT_graph_deserialize(
    graph: *mut *mut Graph,
    file_path: *const c_char,
) -> Result {
    if graph.is_null() || file_path.is_null() {
        return Result::NullPointerError;
    }

    let file_path = unsafe { CStr::from_ptr(file_path) };
    let Ok(file_path) = file_path.to_str() else {
        return Result::InvalidArgumentError;
    };

    let Ok(mut file) = File::open(file_path) else {
        return Result::IoError;
    };

    match rmp_serde::decode::from_read(&mut file) {
        Ok(decoded_graph) => {
            let ptr = Box::into_raw(Box::new(decoded_graph));
            unsafe {
                graph.write(ptr);
            }

            Result::Success
        }
        Err(rmp_serde::decode::Error::InvalidDataRead(_))
        | Err(rmp_serde::decode::Error::InvalidMarkerRead(_)) => Result::IoError,
        Err(_) => Result::InvalidOperationError,
    }
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

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[repr(C)]
pub struct Endpoint {
    /// The position of the endpoint.
    pub position: Point,
    /// The offset into the waypoint list at which the waypoints of this endpoint start.
    pub waypoint_offset: u32,
    /// The number of waypoints associated with the endpoint.
    pub waypoint_count: u32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[repr(C)]
pub struct Net {
    /// The offset into the endpoint list at which the endpoints of this net start.
    pub endpoint_offset: u32,
    /// The number of endpoints in the net.
    pub endpoint_count: u32,
}

#[derive(Serialize, Deserialize)]
struct GraphConnectNetsQuery {
    graph: GraphData,
    nets: Vec<Net>,
    endpoints: Vec<Endpoint>,
    waypoints: Vec<Point>,
    perform_centering: bool,
}

/// Serializes a query to connect nets in a graph.
///
/// **Parameters**  
/// `graph`: The graph to serialize.  
/// `nets`: The list of nets to serialize.  
/// `endpoints`: The list of endpoints to serialize.  
/// `waypoints`: The list of waypoints to serialize.  
/// `file_path`: The file to serialize the graph into.
///
/// **Returns**  
/// `RT_RESULT_SUCCESS`: The operation completed successfully.  
/// `RT_RESULT_NULL_POINTER_ERROR`: `graph`, `nets.ptr`, `endpoints.ptr`, `waypoints.ptr` or `file_path` was `NULL`.  
/// `RT_RESULT_INVALID_OPERATION_ERROR`: The serialization failed.  
/// `RT_RESULT_INVALID_ARGUMENT_ERROR`: `file_path` did not contain legal UTF-8.  
/// `RT_RESULT_IO_ERROR`: An IO error occurred while writing to the file.
#[no_mangle]
#[must_use]
pub unsafe extern "C" fn RT_graph_serialize_connect_nets_query(
    graph: *const Graph,
    nets: Slice<Net>,
    endpoints: Slice<Endpoint>,
    waypoints: Slice<Point>,
    perform_centering: bool,
    file_path: *const c_char,
) -> Result {
    if graph.is_null()
        || nets.is_null()
        || endpoints.is_null()
        || waypoints.is_null()
        || file_path.is_null()
    {
        return Result::NullPointerError;
    }

    let graph = unsafe { &*graph };
    let nets = unsafe { nets.as_ref() };
    let endpoints = unsafe { endpoints.as_ref() };
    let waypoints = unsafe { waypoints.as_ref() };

    let query = GraphConnectNetsQuery {
        graph: graph.data.clone(),
        nets: nets.to_vec(),
        endpoints: endpoints.to_vec(),
        waypoints: waypoints.to_vec(),
        perform_centering,
    };

    let file_path = unsafe { CStr::from_ptr(file_path) };
    let Ok(file_path) = file_path.to_str() else {
        return Result::InvalidArgumentError;
    };

    let Ok(mut file) = File::create(file_path) else {
        return Result::IoError;
    };

    match rmp_serde::encode::write(&mut file, &query) {
        Ok(_) => Result::Success,
        Err(rmp_serde::encode::Error::InvalidValueWrite(_)) => Result::IoError,
        Err(_) => Result::InvalidOperationError,
    }
}

/// Connects nets in a graph.
///
/// **Parameters**  
/// `graph`: The graph to connect the nets in.  
/// `nets`: A list of nets to connect.  
/// `endpoints`: A list of endpoints.  
/// `waypoints`: A list of waypoints.  
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
    waypoints: Slice<Point>,
    vertices: MutSlice<Vertex>,
    wire_views: MutSlice<WireView>,
    mut net_views: MutSlice<NetView>,
    perform_centering: bool,
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
        vertices: Array<'static, Vertex>,
        wire_views: Array<'static, WireView>,
        ends: Vec<Point>,
        centering_candidates: Vec<CenteringCandidate>,
        junctions: JunctionMap,
    }

    struct ThreadlocalData {
        mutable: RefCell<MutableThreadlocalData>,
        vertex_base_offset: usize,
        wire_base_offset: usize,
    }

    let next_thread_index: AtomicU16 = AtomicU16::new(0);
    let threadlocal_data = ThreadLocal::new();

    let result = nets
        .par_iter()
        .zip(net_views.par_iter_mut())
        .try_for_each(|(net, net_view)| {
            let threadlocal_data = threadlocal_data.get_or(|| {
                let thread_index = next_thread_index.fetch_add(1, Ordering::AcqRel);
                assert!(thread_index < num_cpus);
                let thread_index = thread_index as usize;

                let mut vertices = vertices;
                let vertices_start = thread_index * vertices_per_thread;
                let vertices_end = vertices_start + vertices_per_thread;
                let mut vertices = unsafe { vertices.subslice_mut(vertices_start..vertices_end) };
                let vertices = unsafe { vertices.as_uninit_mut().into() };

                let mut wire_views = wire_views;
                let wire_views_start = thread_index * wire_views_per_thread;
                let wire_views_end = wire_views_start + wire_views_per_thread;
                let mut wire_views =
                    unsafe { wire_views.subslice_mut(wire_views_start..wire_views_end) };
                let wire_views = unsafe { wire_views.as_uninit_mut().into() };

                ThreadlocalData {
                    mutable: RefCell::new(MutableThreadlocalData {
                        vertices,
                        wire_views,
                        ends: Vec::new(),
                        centering_candidates: Vec::new(),
                        junctions: JunctionMap::default(),
                    }),
                    vertex_base_offset: vertices_start,
                    wire_base_offset: wire_views_start,
                }
            });

            let ThreadlocalData {
                vertex_base_offset,
                wire_base_offset,
                ..
            } = *threadlocal_data;

            let MutableThreadlocalData {
                vertices,
                wire_views,
                ends,
                centering_candidates,
                junctions,
            } = &mut *threadlocal_data.mutable.borrow_mut();

            let endpoint_start = net.endpoint_offset as usize;
            let endpoint_end = endpoint_start + (net.endpoint_count as usize);
            let endpoints = &endpoints[endpoint_start..endpoint_end];

            let endpoints = endpoints.iter().map(|endpoint| {
                let waypoint_start = endpoint.waypoint_offset as usize;
                let waypoint_end = waypoint_start + (endpoint.waypoint_count as usize);
                let waypoints = &waypoints[waypoint_start..waypoint_end];

                routing::Endpoint {
                    position: endpoint.position,
                    waypoints: Cow::Borrowed(waypoints),
                }
            });

            routing::connect_net(
                graph,
                endpoints,
                vertex_base_offset,
                wire_base_offset,
                vertices,
                wire_views,
                net_view,
                ends,
                centering_candidates,
                junctions,
                perform_centering,
                &mut NoReplay,
            )
        });

    match result {
        Ok(_) => Result::Success,
        Err(err) => err.into(),
    }
}

#[repr(C)]
pub struct ReplayCallbacks {
    pub context: *mut c_void,

    pub begin_path_finding: extern "C" fn(*mut c_void, NodeIndex, Slice<NodeIndex>, bool),
    pub path_finding_set_g_score: extern "C" fn(*mut c_void, NodeIndex, u32),
    pub path_finding_push_open_queue: extern "C" fn(*mut c_void, NodeIndex, u32),
    pub path_finding_set_predecessor: extern "C" fn(*mut c_void, NodeIndex, NodeIndex),
    pub path_finding_pop_open_queue: extern "C" fn(*mut c_void, NodeIndex),
    pub path_finding_clear_state: extern "C" fn(*mut c_void),
    pub path_finding_insert_path_node: extern "C" fn(*mut c_void, usize, NodeIndex),
    pub path_finding_remove_path_node: extern "C" fn(*mut c_void, usize),
    pub end_path_finding: extern "C" fn(*mut c_void, bool),

    pub routing_begin_root_wire: extern "C" fn(*mut c_void, Point, Point),
    pub routing_begin_branch_wire: extern "C" fn(*mut c_void, Point),
    pub routing_push_vertex: extern "C" fn(*mut c_void, Vertex),
    pub routing_end_wire_segment: extern "C" fn(*mut c_void, bool),
    pub routing_end_wire: extern "C" fn(*mut c_void),
}

impl ReplayCapture for ReplayCallbacks {
    #[inline]
    fn begin_path_finding(
        &mut self,
        start: NodeIndex,
        ends: impl Iterator<Item = NodeIndex>,
        visit_all: bool,
    ) {
        let ends: Vec<_> = ends.collect();
        (self.begin_path_finding)(self.context, start, ends.as_slice().into(), visit_all);
    }

    #[inline]
    fn path_finding_set_g_score(&mut self, node: NodeIndex, g_score: u32) {
        (self.path_finding_set_g_score)(self.context, node, g_score);
    }

    #[inline]
    fn path_finding_push_open_queue(&mut self, node: NodeIndex, f_score: u32) {
        (self.path_finding_push_open_queue)(self.context, node, f_score);
    }

    #[inline]
    fn path_finding_set_predecessor(&mut self, node: NodeIndex, predecessor: NodeIndex) {
        (self.path_finding_set_predecessor)(self.context, node, predecessor);
    }

    #[inline]
    fn path_finding_pop_open_queue(&mut self, node: NodeIndex) {
        (self.path_finding_pop_open_queue)(self.context, node);
    }

    #[inline]
    fn path_finding_clear_state(&mut self) {
        (self.path_finding_clear_state)(self.context);
    }

    #[inline]
    fn path_finding_insert_path_node(&mut self, index: usize, node: NodeIndex) {
        (self.path_finding_insert_path_node)(self.context, index, node);
    }

    #[inline]
    fn path_finding_remove_path_node(&mut self, index: usize) {
        (self.path_finding_remove_path_node)(self.context, index);
    }

    #[inline]
    fn end_path_finding(&mut self, found: bool) {
        (self.end_path_finding)(self.context, found);
    }

    #[inline]
    fn routing_begin_root_wire(&mut self, start: Point, end: Point) {
        (self.routing_begin_root_wire)(self.context, start, end);
    }

    #[inline]
    fn routing_begin_branch_wire(&mut self, start: Point) {
        (self.routing_begin_branch_wire)(self.context, start);
    }

    #[inline]
    fn routing_push_vertex(&mut self, vertex: Vertex) {
        (self.routing_push_vertex)(self.context, vertex);
    }

    #[inline]
    fn routing_end_wire_segment(&mut self, ends_in_junction: bool) {
        (self.routing_end_wire_segment)(self.context, ends_in_junction);
    }

    #[inline]
    fn routing_end_wire(&mut self) {
        (self.routing_end_wire)(self.context);
    }
}

/// Connects nets in a graph.
///
/// **Parameters**  
/// `graph`: The graph to connect the nets in.  
/// `nets`: A list of nets to connect.  
/// `endpoints`: A list of endpoints.  
/// `waypoints`: A list of waypoints.  
/// `vertices`: A list to write the found vertices into.  
/// `wire_views`: A list to write the found wires into.  
/// `net_views`: A list to write the found nets into.  
/// `replay`: Callbacks for constructing a replay.
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
pub unsafe extern "C" fn RT_graph_connect_nets_replay(
    graph: *const Graph,
    nets: Slice<Net>,
    endpoints: Slice<Endpoint>,
    waypoints: Slice<Point>,
    mut vertices: MutSlice<Vertex>,
    mut wire_views: MutSlice<WireView>,
    mut net_views: MutSlice<NetView>,
    perform_centering: bool,
    mut replay: ReplayCallbacks,
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

    let mut vertices = unsafe { vertices.as_uninit_mut().into() };
    let mut wire_views = unsafe { wire_views.as_uninit_mut().into() };
    let mut ends = Vec::new();
    let mut centering_candidates = Vec::new();
    let mut junctions = JunctionMap::default();

    for (net, net_view) in nets.iter().zip(net_views.iter_mut()) {
        let endpoint_start = net.endpoint_offset as usize;
        let endpoint_end = endpoint_start + (net.endpoint_count as usize);
        let endpoints = &endpoints[endpoint_start..endpoint_end];

        let endpoints = endpoints.iter().map(|endpoint| {
            let waypoint_start = endpoint.waypoint_offset as usize;
            let waypoint_end = waypoint_start + (endpoint.waypoint_count as usize);
            let waypoints = &waypoints[waypoint_start..waypoint_end];

            routing::Endpoint {
                position: endpoint.position,
                waypoints: Cow::Borrowed(waypoints),
            }
        });

        let result = routing::connect_net(
            graph,
            endpoints,
            0,
            0,
            &mut vertices,
            &mut wire_views,
            net_view,
            &mut ends,
            &mut centering_candidates,
            &mut junctions,
            perform_centering,
            &mut replay,
        );

        match result {
            Ok(_) => (),
            Err(err) => return err.into(),
        }
    }

    Result::Success
}
