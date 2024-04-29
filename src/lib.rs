#![deny(unsafe_op_in_unsafe_fn)]

#[cfg(test)]
mod test;

use rayon::prelude::*;
use std::cell::RefCell;
use std::cmp::Reverse;
use std::num::NonZeroU16;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    PosX,
    NegX,
    PosY,
    NegY,
}

impl Direction {
    const ALL: [Self; 4] = [Self::PosX, Self::NegX, Self::PosY, Self::NegY];

    #[inline]
    const fn opposite(self) -> Self {
        match self {
            Self::PosX => Self::NegX,
            Self::NegX => Self::PosX,
            Self::PosY => Self::NegY,
            Self::NegY => Self::PosY,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

    #[inline]
    fn offset(self, dir: Direction, offset: u16) -> Self {
        match dir {
            Direction::PosX => Self {
                x: self.x + (offset as i32),
                y: self.y,
            },
            Direction::NegX => Self {
                x: self.x - (offset as i32),
                y: self.y,
            },
            Direction::PosY => Self {
                x: self.x,
                y: self.y + (offset as i32),
            },
            Direction::NegY => Self {
                x: self.x,
                y: self.y - (offset as i32),
            },
        }
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

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
struct NeighborList([Option<NonZeroU16>; 4]);

impl NeighborList {
    #[inline]
    const fn new() -> Self {
        Self([None; 4])
    }
}

impl Index<Direction> for NeighborList {
    type Output = Option<NonZeroU16>;

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

#[derive(Default, Debug)]
struct Graph {
    x_coords: Vec<i32>,
    y_coords: Vec<i32>,
    nodes: HashMap<Point, NeighborList>,
}

impl Graph {
    fn build(&mut self, anchor_points: &[Point], bounding_boxes: &[BoundingBox]) {
        let have_horizontal_sightline = |y: i32, mut x1: i32, mut x2: i32| -> bool {
            if x1 > x2 {
                std::mem::swap(&mut x1, &mut x2);
            }

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

        let have_vertical_sightline = |x: i32, mut y1: i32, mut y2: i32| -> bool {
            if y1 > y2 {
                std::mem::swap(&mut y1, &mut y2);
            }

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

        self.nodes.clear();
        for anchor_point in anchor_points.iter().copied() {
            self.nodes.insert(anchor_point, NeighborList::new());
        }

        for anchor_point_a in anchor_points.iter().copied() {
            for anchor_point_b in anchor_points.iter().copied() {
                if have_horizontal_sightline(anchor_point_a.y, anchor_point_a.x, anchor_point_b.x)
                    && have_vertical_sightline(anchor_point_b.x, anchor_point_b.y, anchor_point_a.y)
                {
                    self.nodes.insert(
                        Point {
                            x: anchor_point_b.x,
                            y: anchor_point_a.y,
                        },
                        NeighborList::new(),
                    );
                }

                if have_horizontal_sightline(anchor_point_b.y, anchor_point_b.x, anchor_point_a.x)
                    && have_vertical_sightline(anchor_point_a.x, anchor_point_a.y, anchor_point_b.y)
                {
                    self.nodes.insert(
                        Point {
                            x: anchor_point_a.x,
                            y: anchor_point_b.y,
                        },
                        NeighborList::new(),
                    );
                }
            }
        }

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

        for y in self.y_coords.iter().copied() {
            let mut prev_x: Option<i32> = None;
            for x in self.x_coords.iter().copied() {
                let point = Point { x, y };
                if !self.nodes.contains_key(&point) {
                    continue;
                }

                if let Some(prev_x) = prev_x {
                    let prev_point = Point { x: prev_x, y };
                    let x_diff =
                        NonZeroU16::new(u16::try_from(x - prev_x).expect("node distance too far"))
                            .expect("duplicate coordinates");

                    if have_horizontal_sightline(y, prev_x, x) {
                        let prev_node = self.nodes.get_mut(&prev_point).expect("invalid node");
                        prev_node[Direction::PosX] = Some(x_diff);

                        let node = self.nodes.get_mut(&point).expect("invalid node");
                        node[Direction::NegX] = Some(x_diff);
                    }
                }

                prev_x = Some(x);
            }
        }

        for x in self.x_coords.iter().copied() {
            let mut prev_y: Option<i32> = None;
            for y in self.y_coords.iter().copied() {
                let point = Point { x, y };
                if !self.nodes.contains_key(&point) {
                    continue;
                }

                if let Some(prev_y) = prev_y {
                    let prev_point = Point { x, y: prev_y };
                    let y_diff =
                        NonZeroU16::new(u16::try_from(y - prev_y).expect("node distance too far"))
                            .expect("duplicate coordinates");

                    if have_vertical_sightline(x, prev_y, y) {
                        let prev_node = self.nodes.get_mut(&prev_point).expect("invalid node");
                        prev_node[Direction::PosY] = Some(y_diff);

                        let node = self.nodes.get_mut(&point).expect("invalid node");
                        node[Direction::NegY] = Some(y_diff);
                    }
                }

                prev_y = Some(y);
            }
        }
    }
}

#[derive(Default)]
struct PathFinder {
    g_score: HashMap<Point, u32>,
    predecessor: HashMap<Point, Direction>,
    open_queue: PriorityQueue<Point, Reverse<u32>>,
}

impl PathFinder {
    fn build_path(&self, graph: &Graph, path: &mut Vec<Point>, start: Point) {
        path.push(start);

        let mut dir: Option<Direction> = None;
        let mut current = start;
        loop {
            let Some(&pred_dir) = self.predecessor.get(&current) else {
                break;
            };

            let pred_offset = graph.nodes[&current][pred_dir].expect("invalid predecessor");
            let pred = current.offset(pred_dir, pred_offset.get());

            if Some(pred_dir) == dir {
                *path.last_mut().unwrap() = pred;
            } else {
                path.push(pred);
                dir = Some(pred_dir);
            }

            current = pred;
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

        self.g_score.insert(end, 0);
        self.open_queue.push(end, Reverse(0));

        while let Some((current, _)) = self.open_queue.pop() {
            if current == start {
                self.build_path(graph, path, start);
                return true;
            }

            let straight_dir = self
                .predecessor
                .get(&current)
                .copied()
                .map(Direction::opposite);

            for dir in Direction::ALL {
                let Some(neighbor_offset) = graph.nodes[&current][dir] else {
                    continue;
                };

                let neighbor = current.offset(dir, neighbor_offset.get());

                let new_g_score = self.g_score[&current]
                    + (neighbor_offset.get() as u32)
                        * if Some(dir) == straight_dir { 1 } else { 2 };

                let update = match self.g_score.get(&neighbor) {
                    Some(&g_score) => new_g_score < g_score,
                    None => true,
                };

                if update {
                    self.g_score.insert(neighbor, new_g_score);
                    self.predecessor.insert(neighbor, dir.opposite());

                    let new_f_score = new_g_score + neighbor.manhatten_distance_to(start);
                    self.open_queue.push(neighbor, Reverse(new_f_score));
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

    let ptr = Box::into_raw(Box::new(Graph::default()));
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
    buffer: *mut Point,
    buffer_size: usize,
    node_count: *mut usize,
) -> RoutingResult {
    if graph.is_null() || buffer.is_null() || node_count.is_null() {
        return RoutingResult::NullPointerError;
    }

    let graph = unsafe { &*graph };
    if graph.nodes.len() > buffer_size {
        return RoutingResult::BufferOverflowError;
    }

    unsafe {
        node_count.write(graph.nodes.len());
    }

    for (i, node) in graph.nodes.keys().copied().enumerate() {
        unsafe {
            buffer.add(i).write(node);
        }
    }

    RoutingResult::Success
}

#[repr(C)]
struct Neighbors {
    pos_x: Point,
    neg_x: Point,
    pos_y: Point,
    neg_y: Point,
}

#[no_mangle]
#[must_use]
unsafe extern "C" fn RT_graph_get_node_neighbors(
    graph: *const Graph,
    node: Point,
    neighbors: *mut Neighbors,
) -> RoutingResult {
    if graph.is_null() || neighbors.is_null() {
        return RoutingResult::NullPointerError;
    }

    let graph = unsafe { &*graph };
    if let Some(list) = graph.nodes.get(&node).copied() {
        unsafe {
            neighbors.write(Neighbors {
                pos_x: node.offset(
                    Direction::PosX,
                    list[Direction::PosX].map(NonZeroU16::get).unwrap_or(0),
                ),
                neg_x: node.offset(
                    Direction::NegX,
                    list[Direction::NegX].map(NonZeroU16::get).unwrap_or(0),
                ),
                pos_y: node.offset(
                    Direction::PosY,
                    list[Direction::PosY].map(NonZeroU16::get).unwrap_or(0),
                ),
                neg_y: node.offset(
                    Direction::NegY,
                    list[Direction::NegY].map(NonZeroU16::get).unwrap_or(0),
                ),
            });
        }

        RoutingResult::Success
    } else {
        RoutingResult::InvalidOperationError
    }
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
        })
    });

    match result {
        Ok(_) => RoutingResult::Success,
        Err(err) => err,
    }
}
