#![deny(unsafe_op_in_unsafe_fn)]

#[cfg(test)]
mod test;

use rayon::prelude::*;
use std::cell::RefCell;
use std::cmp::Reverse;
use std::ops::{Index, IndexMut};

type HashMap<K, V> = ahash::AHashMap<K, V>;
type PriorityQueue<I, P> = priority_queue::PriorityQueue<I, P, ahash::RandomState>;

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

pub const INVALID_INDEX: u32 = u32::MAX;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    #[inline]
    fn manhatten_distance_to(self, other: Self) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
struct BoundingBox {
    center: Point,
    half_width: u16,
    half_height: u16,
}

impl BoundingBox {
    #[inline]
    fn min_x(self) -> i32 {
        self.center.x - (self.half_width as i32)
    }

    #[inline]
    fn max_x(self) -> i32 {
        self.center.x + (self.half_width as i32)
    }

    #[inline]
    fn min_y(self) -> i32 {
        self.center.y - (self.half_height as i32)
    }

    #[inline]
    fn max_y(self) -> i32 {
        self.center.y + (self.half_height as i32)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    PosX,
    NegX,
    PosY,
    NegY,
}

impl Direction {
    const ALL: [Self; 4] = [Self::PosX, Self::NegX, Self::PosY, Self::NegY];
}

#[derive(Clone)]
#[repr(transparent)]
struct NeighborList([u32; 4]);

impl NeighborList {
    #[inline]
    const fn new() -> Self {
        Self([INVALID_INDEX; 4])
    }

    fn find(&self, node: u32) -> Option<Direction> {
        for dir in Direction::ALL {
            if self[dir] == node {
                return Some(dir);
            }
        }

        None
    }
}

impl Index<Direction> for NeighborList {
    type Output = u32;

    #[inline]
    fn index(&self, index: Direction) -> &Self::Output {
        match index {
            Direction::PosX => &self.0[0],
            Direction::NegX => &self.0[1],
            Direction::PosY => &self.0[2],
            Direction::NegY => &self.0[3],
        }
    }
}

impl IndexMut<Direction> for NeighborList {
    #[inline]
    fn index_mut(&mut self, index: Direction) -> &mut Self::Output {
        match index {
            Direction::PosX => &mut self.0[0],
            Direction::NegX => &mut self.0[1],
            Direction::PosY => &mut self.0[2],
            Direction::NegY => &mut self.0[3],
        }
    }
}

#[derive(Clone)]
#[repr(C)]
struct Node {
    point: Point,
    neighbors: NeighborList,
}

#[repr(transparent)]
struct NodeList(Vec<Node>);

impl NodeList {
    #[inline]
    fn new() -> Self {
        Self(Vec::new())
    }

    #[inline]
    fn resize(&mut self, new_len: u32) {
        const INIT: Node = Node {
            point: Point { x: 0, y: 0 },
            neighbors: NeighborList::new(),
        };

        self.0.clear();
        self.0.resize(new_len as usize, INIT);
    }

    #[inline]
    fn as_mut_ptr(&mut self) -> *mut Node {
        self.0.as_mut_ptr()
    }
}

impl Index<u32> for NodeList {
    type Output = Node;

    #[inline]
    fn index(&self, index: u32) -> &Self::Output {
        &self.0[index as usize]
    }
}

struct Graph {
    x_coords: Vec<i32>,
    y_coords: Vec<i32>,
    nodes: NodeList,
}

impl Graph {
    fn new() -> Self {
        Self {
            x_coords: Vec::new(),
            y_coords: Vec::new(),
            nodes: NodeList::new(),
        }
    }

    fn build(&mut self, anchor_points: &[Point], bounding_boxes: &[BoundingBox]) {
        let have_horizontal_sightline = |y: i32, x1: i32, x2: i32| -> bool {
            assert!(x1 < x2);

            for &bb in bounding_boxes {
                if (y < bb.min_y()) || (y > bb.max_y()) {
                    continue;
                }

                if (x2 <= bb.min_x()) || (x1 >= bb.max_x()) {
                    continue;
                }

                return false;
            }

            true
        };

        let have_vertical_sightline = |x: i32, y1: i32, y2: i32| -> bool {
            assert!(y1 < y2);

            for &bb in bounding_boxes {
                if (x < bb.min_x()) || (x > bb.max_x()) {
                    continue;
                }

                if (y2 <= bb.min_y()) || (y1 >= bb.max_y()) {
                    continue;
                }

                return false;
            }

            true
        };

        self.x_coords.clear();
        self.x_coords.reserve(anchor_points.len());
        self.x_coords
            .extend(anchor_points.iter().map(|&Point { x, .. }| x));
        self.x_coords.par_sort_unstable();
        self.x_coords.dedup();

        self.y_coords.clear();
        self.y_coords.reserve(anchor_points.len());
        self.y_coords
            .extend(anchor_points.iter().map(|&Point { y, .. }| y));
        self.y_coords.par_sort_unstable();
        self.y_coords.dedup();

        let node_count: u32 = (self.x_coords.len() * self.y_coords.len())
            .try_into()
            .expect("too many nodes");
        self.nodes.resize(node_count);

        let nodes = SyncPtr(self.nodes.as_mut_ptr());

        self.y_coords
            .par_iter()
            .copied()
            .enumerate()
            .for_each(|(yi, y)| {
                let nodes = nodes;
                macro_rules! nodes {
                    ($index:expr) => {
                        (unsafe { &mut *nodes.0.add($index) })
                    };
                }

                let mut prev: Option<usize> = None;
                for (xi, x) in self.x_coords.iter().copied().enumerate() {
                    let index = yi * self.x_coords.len() + xi;
                    nodes![index].point = Point { x, y };

                    if let Some(prev) = prev {
                        if have_horizontal_sightline(y, nodes![prev].point.x, x) {
                            nodes![prev].neighbors[Direction::PosX] = index as u32;
                            nodes![index].neighbors[Direction::NegX] = prev as u32;
                        }
                    }

                    prev = Some(index);
                }
            });

        self.x_coords
            .par_iter()
            .copied()
            .enumerate()
            .for_each(|(xi, x)| {
                let nodes = nodes;
                macro_rules! nodes {
                    ($index:expr) => {
                        (unsafe { &mut *nodes.0.add($index) })
                    };
                }

                let mut prev: Option<usize> = None;
                for (yi, y) in self.y_coords.iter().copied().enumerate() {
                    let index = yi * self.x_coords.len() + xi;

                    if let Some(prev) = prev {
                        if have_vertical_sightline(x, nodes![prev].point.y, y) {
                            nodes![prev].neighbors[Direction::PosY] = index as u32;
                            nodes![index].neighbors[Direction::NegY] = prev as u32;
                        }
                    }

                    prev = Some(index);
                }
            });
    }

    fn find_node(&self, point: Point) -> Option<u32> {
        self.nodes
            .0
            .binary_search_by(|node| {
                let x_ord = node.point.x.cmp(&point.x);
                let y_ord = node.point.y.cmp(&point.y);
                y_ord.then(x_ord)
            })
            .ok()
            .map(|index| index as u32)
    }
}

#[derive(Default)]
struct PathFinder {
    g_score: HashMap<u32, u32>,
    predecessor: HashMap<u32, u32>,
    open_queue: PriorityQueue<u32, Reverse<u32>>,
}

impl PathFinder {
    fn build_path(&self, graph: &Graph, path: &mut Vec<Point>, start_index: u32) {
        path.push(graph.nodes[start_index].point);

        let mut dir: Option<Direction> = None;
        let mut current_index = start_index;
        loop {
            let Some(&pred_index) = self.predecessor.get(&current_index) else {
                break;
            };

            let pred_dir = graph.nodes[current_index]
                .neighbors
                .find(pred_index)
                .expect("invalid predecessor");

            if Some(pred_dir) == dir {
                *path.last_mut().unwrap() = graph.nodes[pred_index].point;
            } else {
                path.push(graph.nodes[pred_index].point);
                dir = Some(pred_dir);
            }

            current_index = pred_index;
        }
    }

    fn find_path(
        &mut self,
        graph: &Graph,
        path: &mut Vec<Point>,
        start: Point,
        end: Point,
    ) -> bool {
        self.g_score.clear();
        self.predecessor.clear();
        self.open_queue.clear();

        let start_index = graph.find_node(start).expect("invalid start node");
        let end_index = graph.find_node(end).expect("invalid end node");

        self.g_score.insert(end_index, 0);
        self.open_queue.push(end_index, Reverse(0));

        while let Some((current_index, _)) = self.open_queue.pop() {
            if current_index == start_index {
                self.build_path(graph, path, start_index);
                return true;
            }

            let straight_dir = self
                .predecessor
                .get(&current_index)
                .copied()
                .map(|pred_index| {
                    graph.nodes[pred_index]
                        .neighbors
                        .find(current_index)
                        .expect("invalid predecessor")
                });

            for dir in Direction::ALL {
                let neighbor_index = graph.nodes[current_index].neighbors[dir];
                if neighbor_index == INVALID_INDEX {
                    continue;
                }

                let new_g_score = self.g_score[&current_index]
                    + graph.nodes[current_index]
                        .point
                        .manhatten_distance_to(graph.nodes[neighbor_index].point)
                        * if Some(dir) == straight_dir { 1 } else { 2 };

                let update = match self.g_score.get(&neighbor_index) {
                    Some(&g_score) => new_g_score < g_score,
                    None => true,
                };

                if update {
                    self.g_score.insert(neighbor_index, new_g_score);
                    self.predecessor.insert(neighbor_index, current_index);

                    let new_f_score = new_g_score
                        + graph.nodes[neighbor_index]
                            .point
                            .manhatten_distance_to(start);
                    self.open_queue.push(neighbor_index, Reverse(new_f_score));
                }
            }
        }

        false
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
enum RoutingResult {
    Success = 0,
    NullPointerError = 1,
    InvalidOperationError = 2,
    BufferOverflowError = 3,
}

#[no_mangle]
#[must_use]
unsafe extern "C" fn RT_init_thread_pool(thread_count: *mut usize) -> RoutingResult {
    if thread_count.is_null() {
        return RoutingResult::NullPointerError;
    }

    let num_cpus = num_cpus::get();
    unsafe {
        thread_count.write(num_cpus);
    }

    match rayon::ThreadPoolBuilder::new()
        .num_threads(num_cpus)
        .build_global()
    {
        Ok(_) => RoutingResult::Success,
        Err(_) => RoutingResult::InvalidOperationError,
    }
}

#[no_mangle]
#[must_use]
unsafe extern "C" fn RT_graph_new(graph: *mut *mut Graph) -> RoutingResult {
    if graph.is_null() {
        return RoutingResult::NullPointerError;
    }

    let ptr = Box::into_raw(Box::new(Graph::new()));
    unsafe {
        graph.write(ptr);
    }

    RoutingResult::Success
}

#[no_mangle]
#[must_use]
unsafe extern "C" fn RT_graph_build(
    graph: *mut Graph,
    anchor_points: *const Point,
    anchor_point_count: usize,
    bounding_boxes: *const BoundingBox,
    bounding_box_count: usize,
) -> RoutingResult {
    if graph.is_null() || anchor_points.is_null() || bounding_boxes.is_null() {
        return RoutingResult::NullPointerError;
    }

    let graph = unsafe { &mut *graph };
    let anchor_points = unsafe { std::slice::from_raw_parts(anchor_points, anchor_point_count) };
    let bounding_boxes = unsafe { std::slice::from_raw_parts(bounding_boxes, bounding_box_count) };
    graph.build(anchor_points, bounding_boxes);

    RoutingResult::Success
}

#[no_mangle]
#[must_use]
unsafe extern "C" fn RT_graph_get_nodes(
    graph: *const Graph,
    nodes: *mut *const Node,
    node_count: *mut usize,
) -> RoutingResult {
    if graph.is_null() || nodes.is_null() || node_count.is_null() {
        return RoutingResult::NullPointerError;
    }

    let graph = unsafe { &*graph };
    unsafe {
        nodes.write(graph.nodes.0.as_ptr());
        node_count.write(graph.nodes.0.len());
    }

    RoutingResult::Success
}

#[no_mangle]
#[must_use]
unsafe extern "C" fn RT_graph_free(graph: *mut Graph) -> RoutingResult {
    if graph.is_null() {
        return RoutingResult::NullPointerError;
    }

    let graph = unsafe { Box::from_raw(graph) };
    std::mem::drop(graph);

    RoutingResult::Success
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
struct PathDef {
    net_id: u32,
    start: Point,
    end: Point,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
struct Vertex {
    net_id: u32,
    x: f32,
    y: f32,
}

#[derive(Clone, Copy)]
#[repr(C)]
struct VertexBuffer {
    vertices: *mut Vertex,
    vertex_count: usize,
}

#[no_mangle]
#[must_use]
unsafe extern "C" fn RT_graph_find_paths(
    graph: *const Graph,
    paths: *const PathDef,
    path_count: usize,
    vertex_buffers: *mut VertexBuffer,
    vertex_buffer_capacity: usize,
) -> RoutingResult {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use thread_local::ThreadLocal;

    if graph.is_null() || paths.is_null() || vertex_buffers.is_null() {
        return RoutingResult::NullPointerError;
    }

    {
        let vertex_buffers =
            unsafe { std::slice::from_raw_parts_mut(vertex_buffers, rayon::current_num_threads()) };

        for vertex_buffer in vertex_buffers {
            if vertex_buffer.vertices.is_null() {
                return RoutingResult::NullPointerError;
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
        buffer_index: usize,
    }

    let buffer_index: AtomicUsize = AtomicUsize::new(0);
    let thread_local_data = ThreadLocal::new();

    let result = paths.par_iter().copied().try_for_each(|path_def| {
        let thread_local_data = thread_local_data.get_or(|| {
            let buffer_index = buffer_index.fetch_add(1, Ordering::SeqCst);
            assert!(buffer_index < rayon::current_num_threads());

            RefCell::new(ThreadLocalData {
                path_finder: PathFinder::default(),
                path: Vec::new(),
                buffer_index,
            })
        });

        let &mut ThreadLocalData {
            ref mut path_finder,
            ref mut path,
            buffer_index,
        } = &mut *thread_local_data.borrow_mut();

        let vertex_buffers = vertex_buffers;
        let vertex_buffer = unsafe { vertex_buffers.0.add(buffer_index) };
        let vertex_buffer = unsafe { &mut *vertex_buffer };

        path.clear();

        if path_finder.find_path(graph, path, path_def.start, path_def.end) {
            if vertex_buffer_capacity < (vertex_buffer.vertex_count + path.len()) {
                return Err(RoutingResult::BufferOverflowError);
            }

            for (i, point) in path.iter().copied().enumerate() {
                unsafe {
                    vertex_buffer
                        .vertices
                        .add(vertex_buffer.vertex_count + i)
                        .write(Vertex {
                            net_id: path_def.net_id,
                            x: point.x as f32,
                            y: point.y as f32,
                        });
                }
            }

            vertex_buffer.vertex_count += path.len();
        } else {
            if vertex_buffer_capacity < (vertex_buffer.vertex_count + 2) {
                return Err(RoutingResult::BufferOverflowError);
            }

            unsafe {
                vertex_buffer
                    .vertices
                    .add(vertex_buffer.vertex_count + 0)
                    .write(Vertex {
                        net_id: path_def.net_id,
                        x: path_def.start.x as f32,
                        y: path_def.start.y as f32,
                    });

                vertex_buffer
                    .vertices
                    .add(vertex_buffer.vertex_count + 1)
                    .write(Vertex {
                        net_id: path_def.net_id,
                        x: path_def.end.x as f32,
                        y: path_def.end.y as f32,
                    });
            }

            vertex_buffer.vertex_count += 2;
        }

        Ok(())
    });

    match result {
        Ok(_) => RoutingResult::Success,
        Err(err) => err,
    }
}
