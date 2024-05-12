#![allow(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use crate::*;
use std::mem::MaybeUninit;
use std::sync::atomic::{AtomicU16, Ordering};
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
    UninitializedError = 4,
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
    /// The start point of the path.
    pub start: Point,
    /// The end point of the path.
    pub end: Point,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Vertex {
    /// The X coordinate of the vertex.
    pub x: f32,
    /// The Y coordinate of the vertex.
    pub y: f32,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PathRange {
    /// The vertex offset this range starts at.
    pub vertex_offset: u32,
    /// The length of this range.
    pub vertex_count: u16,
    /// The index of the vertex buffer this range is in.
    pub vertex_buffer_index: u16,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VertexBuffer {
    /// A list of vertices.
    pub vertices: *mut Vertex,
    /// The number of elements in `vertices`.
    pub vertex_count: u32,
}

#[inline]
unsafe fn slice_from_raw_parts_mut_uninit<'a, T>(
    ptr: *mut T,
    len: usize,
) -> &'a mut [MaybeUninit<T>] {
    unsafe { std::slice::from_raw_parts_mut(ptr as *mut MaybeUninit<T>, len) }
}

fn extend_vertex_buffer(
    vertex_buffer: &mut VertexBuffer,
    vertex_buffer_capacity: u32,
    vertex_buffer_index: u16,
    path_range: &mut MaybeUninit<PathRange>,
    path: &[Point],
) -> std::result::Result<(), Result> {
    let path_len: u16 = path.len().try_into().expect("path too long");
    let Some(new_vertex_count) = vertex_buffer.vertex_count.checked_add(path_len as u32) else {
        return Err(Result::BufferOverflowError);
    };
    if vertex_buffer_capacity < new_vertex_count {
        return Err(Result::BufferOverflowError);
    }

    path_range.write(PathRange {
        vertex_offset: vertex_buffer.vertex_count,
        vertex_count: path_len,
        vertex_buffer_index,
    });

    for (i, point) in path.iter().copied().enumerate() {
        unsafe {
            vertex_buffer
                .vertices
                .add((vertex_buffer.vertex_count as usize) + i)
                .write(Vertex {
                    x: point.x as f32,
                    y: point.y as f32,
                });
        }
    }

    vertex_buffer.vertex_count = new_vertex_count;
    Ok(())
}

/// Finds shortest paths through a graph.
///
/// **Parameters**  
/// `graph`: The graph to find the paths through.  
/// `paths`: A list of paths to find.  
/// `path_ranges`: A list to write the range of vertices that belongs to each path into.  
/// `path_count`: The number of elements in `paths` and `path_ranges`.  
/// `vertex_buffers`: A list of buffers to write the found paths into. There must be exactly as many buffers as threads in the pool.  
/// `vertex_buffer_capacity`: The maximum number of vertices each buffer in `vertex_buffers` can hold.
///
/// **Returns**  
/// `RT_RESULT_SUCCESS`: The operation completed successfully.  
/// `RT_RESULT_NULL_POINTER_ERROR`: `graph`, `paths`, `path_ranges`, `vertex_buffers` or `VertexBuffer::vertices` was `NULL`.  
/// `RT_RESULT_INVALID_OPERATION_ERROR`: One of the paths had an invalid start or end point.  
/// `RT_RESULT_BUFFER_OVERFLOW_ERROR`: The capacity of the vertex buffers was too small to hold all vertices.  
/// `RT_RESULT_UNINITIALIZED_ERROR`: The thread pool was not initialized yet.
#[no_mangle]
#[must_use]
pub unsafe extern "C" fn RT_graph_find_paths(
    graph: *const Graph,
    paths: *const PathDef,
    path_ranges: *mut PathRange,
    path_count: u32,
    vertex_buffers: *mut VertexBuffer,
    vertex_buffer_capacity: u32,
) -> Result {
    let num_cpus = NUM_CPUS.load(Ordering::Acquire);
    if num_cpus == 0 {
        return Result::UninitializedError;
    }
    assert_eq!(num_cpus as usize, rayon::current_num_threads());

    if graph.is_null() || paths.is_null() || path_ranges.is_null() || vertex_buffers.is_null() {
        return Result::NullPointerError;
    }

    {
        let vertex_buffers =
            unsafe { std::slice::from_raw_parts_mut(vertex_buffers, num_cpus as usize) };

        for vertex_buffer in vertex_buffers {
            if vertex_buffer.vertices.is_null() {
                return Result::NullPointerError;
            }

            vertex_buffer.vertex_count = 0;
        }
    }

    let graph = unsafe { &*graph };
    let paths = unsafe { std::slice::from_raw_parts(paths, path_count as usize) };
    let path_ranges = unsafe { slice_from_raw_parts_mut_uninit(path_ranges, path_count as usize) };
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

    let next_buffer_index: AtomicU16 = AtomicU16::new(0);
    let buffer_index = ThreadLocal::new();

    let result = paths
        .par_iter()
        .copied()
        .zip(path_ranges.par_iter_mut())
        .try_for_each(|(path_def, path_range)| {
            THREAD_LOCAL_DATA.with_borrow_mut(|ThreadLocalData { path_finder, path }| {
                let vertex_buffer_index = *buffer_index.get_or(|| {
                    let buffer_index = next_buffer_index.fetch_add(1, Ordering::AcqRel);
                    assert!(buffer_index < num_cpus);
                    buffer_index
                });

                let vertex_buffers = vertex_buffers;
                let vertex_buffer = unsafe { vertex_buffers.0.add(vertex_buffer_index as usize) };
                let vertex_buffer = unsafe { &mut *vertex_buffer };

                path.clear();

                match path_finder.find_path_impl(graph, path, path_def.start, &[path_def.end]) {
                    PathFindResult::Found(_) => extend_vertex_buffer(
                        vertex_buffer,
                        vertex_buffer_capacity,
                        vertex_buffer_index,
                        path_range,
                        &path,
                    ),
                    PathFindResult::NotFound => extend_vertex_buffer(
                        vertex_buffer,
                        vertex_buffer_capacity,
                        vertex_buffer_index,
                        path_range,
                        &[path_def.start, path_def.end],
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
