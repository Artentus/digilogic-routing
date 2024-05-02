#![allow(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use crate::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use thread_local::ThreadLocal;

#[repr(transparent)]
struct SyncPtr<T: ?Sized>(*mut T);

impl<T: ?Sized> Clone for SyncPtr<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl<T: ?Sized> Copy for SyncPtr<T> {}
unsafe impl<T: ?Sized> Send for SyncPtr<T> {}
unsafe impl<T: ?Sized> Sync for SyncPtr<T> {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum Result {
    Success = 0,
    NullPointerError = 1,
    InvalidOperationError = 2,
    BufferOverflowError = 3,
}

/// Initializes the thread pool.
///
/// **Parameters**  
/// `[out] thread_count`: The number of threads in the pool.
///
/// **Returns**  
/// `RT_RESULT_SUCCESS`: The operation completed successfully.  
/// `RT_RESULT_NULL_POINTER_ERROR`: `thread_count` was `NULL`.  
/// `RT_RESULT_INVALID_OPERATION_ERROR`: The function was called more than once.
#[no_mangle]
#[must_use]
pub unsafe extern "C" fn RT_init_thread_pool(thread_count: *mut usize) -> Result {
    if thread_count.is_null() {
        return Result::NullPointerError;
    }

    let num_cpus = num_cpus::get();
    unsafe {
        thread_count.write(num_cpus);
    }

    match rayon::ThreadPoolBuilder::new()
        .num_threads(num_cpus)
        .build_global()
    {
        Ok(_) => Result::Success,
        Err(_) => Result::InvalidOperationError,
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
/// `anchor_points`: A list of anchor points to build the graph from.  
/// `anchor_point_count`: The number of elements in `anchor_points`.  
/// `bounding_boxes`: A list of bounding boxes to build the graph from.  
/// `bounding_box_count`: The number of elements in `bounding_boxes`.
///
/// **Returns**  
/// `RT_RESULT_SUCCESS`: The operation completed successfully.  
/// `RT_RESULT_NULL_POINTER_ERROR`: `graph`, `anchor_points` or `bounding_boxes` was `NULL`.
#[no_mangle]
#[must_use]
pub unsafe extern "C" fn RT_graph_build(
    graph: *mut Graph,
    anchor_points: *const Point,
    anchor_point_count: usize,
    bounding_boxes: *const BoundingBox,
    bounding_box_count: usize,
) -> Result {
    if graph.is_null() || anchor_points.is_null() || bounding_boxes.is_null() {
        return Result::NullPointerError;
    }

    let graph = unsafe { &mut *graph };
    let anchor_points = unsafe { std::slice::from_raw_parts(anchor_points, anchor_point_count) };
    let bounding_boxes = unsafe { std::slice::from_raw_parts(bounding_boxes, bounding_box_count) };
    graph.build(anchor_points, bounding_boxes);

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
        nodes.write(graph.nodes.0.as_ptr());
        node_count.write(graph.nodes.0.len());
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

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PathDef {
    /// The ID of the net this path belongs to.  
    /// Populates the corresponding field in the resulting vertices.
    pub net_id: u32,
    /// The start point of the path.
    pub start: Point,
    /// The end point of the path.
    pub end: Point,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Vertex {
    /// The ID of the net this vertex belongs to.  
    /// Populated by the corresponding field in the path.
    pub net_id: u32,
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VertexBuffer {
    /// A list of vertices.
    pub vertices: *mut Vertex,
    /// The number of elements in `vertices`.
    pub vertex_count: usize,
}

fn extend_vertex_buffer(
    vertex_buffer: &mut VertexBuffer,
    vertex_buffer_capacity: usize,
    path: &[Point],
    net_id: u32,
) -> std::result::Result<(), Result> {
    if vertex_buffer_capacity < (vertex_buffer.vertex_count + path.len()) {
        return Err(Result::BufferOverflowError);
    }

    for (i, point) in path.iter().copied().enumerate() {
        unsafe {
            vertex_buffer
                .vertices
                .add(vertex_buffer.vertex_count + i)
                .write(Vertex {
                    net_id,
                    x: point.x as f32,
                    y: point.y as f32,
                });
        }
    }

    vertex_buffer.vertex_count += path.len();
    Ok(())
}

/// Finds shortest paths through a graph.
///
/// **Parameters**  
/// `graph`: The graph to find the paths through.  
/// `paths`: A list of paths to find.  
/// `path_count`: The number of elements in `paths`.  
/// `vertex_buffers`: A list of buffers to write the found paths into. There must be exactly as many buffers as threads in the pool.  
/// `vertex_buffer_capacity`: The maximum number of vertices each buffer in `vertex_buffers` can hold.
///
/// **Returns**  
/// `RT_RESULT_SUCCESS`: The operation completed successfully.  
/// `RT_RESULT_NULL_POINTER_ERROR`: `graph`, `paths`, `vertex_buffers` or `VertexBuffer::vertices` was `NULL`.  
/// `RT_RESULT_INVALID_OPERATION_ERROR`: One of the paths had an invalid start or end point.  
/// `RT_RESULT_BUFFER_OVERFLOW_ERROR`: The capacity of the vertex buffers was too small to hold all vertices.
#[no_mangle]
#[must_use]
pub unsafe extern "C" fn RT_graph_find_paths(
    graph: *const Graph,
    paths: *const PathDef,
    path_count: usize,
    vertex_buffers: *mut VertexBuffer,
    vertex_buffer_capacity: usize,
) -> Result {
    if graph.is_null() || paths.is_null() || vertex_buffers.is_null() {
        return Result::NullPointerError;
    }

    {
        let vertex_buffers =
            unsafe { std::slice::from_raw_parts_mut(vertex_buffers, rayon::current_num_threads()) };

        for vertex_buffer in vertex_buffers {
            if vertex_buffer.vertices.is_null() {
                return Result::NullPointerError;
            }

            vertex_buffer.vertex_count = 0;
        }
    }

    let graph = unsafe { &*graph };
    let paths = unsafe { std::slice::from_raw_parts(paths, path_count) };
    let vertex_buffers = SyncPtr(vertex_buffers);

    struct ThreadLocalData {
        path_finder: PathFinder,
        path: Vec<Point>,
    }

    thread_local! {
        static THREAD_LOCAL_DATA: RefCell<ThreadLocalData> = RefCell::new(ThreadLocalData {
            path_finder: PathFinder::default(),
            path: Vec::new(),
        });
    }

    let next_buffer_index: AtomicUsize = AtomicUsize::new(0);
    let buffer_index = ThreadLocal::new();

    let result = paths.par_iter().copied().try_for_each(|path_def| {
        THREAD_LOCAL_DATA.with_borrow_mut(|ThreadLocalData { path_finder, path }| {
            let buffer_index = *buffer_index.get_or(|| {
                let buffer_index = next_buffer_index.fetch_add(1, Ordering::SeqCst);
                assert!(buffer_index < rayon::current_num_threads());
                buffer_index
            });

            let vertex_buffers = vertex_buffers;
            let vertex_buffer = unsafe { vertex_buffers.0.add(buffer_index) };
            let vertex_buffer = unsafe { &mut *vertex_buffer };

            path.clear();

            match path_finder.find_path_impl(graph, path, path_def.start, path_def.end) {
                PathFindResult::Found(_) => extend_vertex_buffer(
                    vertex_buffer,
                    vertex_buffer_capacity,
                    &path,
                    path_def.net_id,
                ),
                PathFindResult::NotFound => extend_vertex_buffer(
                    vertex_buffer,
                    vertex_buffer_capacity,
                    &[path_def.start, path_def.end],
                    path_def.net_id,
                ),
                PathFindResult::InvalidStartPoint | PathFindResult::InvalidEndPoint => {
                    Err(Result::InvalidOperationError)
                }
            }
        })
    });

    match result {
        Ok(_) => Result::Success,
        Err(err) => err,
    }
}
