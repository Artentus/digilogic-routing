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
enum Direction {
    PosX,
    NegX,
    PosY,
    NegY,
}

impl Direction {
    const ALL: [Self; 4] = [Self::PosX, Self::NegX, Self::PosY, Self::NegY];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    #[inline]
    fn manhatten_distance_to(self, other: Self) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct BoundingBox {
    pub center: Point,
    pub half_width: u16,
    pub half_height: u16,
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
#[repr(C)]
struct Node {
    point: Point,
    neighbors: NeighborList,
}

#[derive(Default, Debug)]
#[repr(transparent)]
struct NodeList(Vec<Node>);

impl NodeList {
    #[inline]
    fn clear(&mut self) {
        self.0.clear();
    }

    #[inline]
    fn push(&mut self, point: Point) -> u32 {
        let index: u32 = self.0.len().try_into().expect("too many nodes");
        self.0.push(Node {
            point,
            neighbors: NeighborList::new(),
        });
        index
    }
}

impl Index<u32> for NodeList {
    type Output = Node;

    #[inline]
    fn index(&self, index: u32) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl IndexMut<u32> for NodeList {
    #[inline]
    fn index_mut(&mut self, index: u32) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}

fn have_horizontal_sightline(bounding_boxes: &[BoundingBox], y: i32, x1: i32, x2: i32) -> bool {
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
}

fn have_vertical_sightline(bounding_boxes: &[BoundingBox], x: i32, y1: i32, y2: i32) -> bool {
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
}

fn find_neg_horizontal_cutoff(
    bounding_boxes: &[BoundingBox],
    x_coords: &[i32],
    y: i32,
    x2: i32,
    offset: usize,
) -> usize {
    if x_coords.len() == 0 {
        return offset;
    }

    let center = x_coords.len() / 2;
    let x1 = x_coords[center];

    if have_horizontal_sightline(bounding_boxes, y, x1, x2) {
        find_neg_horizontal_cutoff(bounding_boxes, &x_coords[..center], y, x2, offset)
    } else {
        find_neg_horizontal_cutoff(
            bounding_boxes,
            &x_coords[(center + 1)..],
            y,
            x2,
            offset + center + 1,
        )
    }
}

fn find_pos_horizontal_cutoff(
    bounding_boxes: &[BoundingBox],
    x_coords: &[i32],
    y: i32,
    x1: i32,
    offset: usize,
) -> usize {
    if x_coords.len() == 0 {
        return offset;
    }

    let center = x_coords.len() / 2;
    let x2 = x_coords[center];

    if have_horizontal_sightline(bounding_boxes, y, x1, x2) {
        find_pos_horizontal_cutoff(
            bounding_boxes,
            &x_coords[(center + 1)..],
            y,
            x1,
            offset + center + 1,
        )
    } else {
        find_pos_horizontal_cutoff(bounding_boxes, &x_coords[..center], y, x1, offset)
    }
}

fn find_neg_vertical_cutoff(
    bounding_boxes: &[BoundingBox],
    y_coords: &[i32],
    x: i32,
    y2: i32,
    offset: usize,
) -> usize {
    if y_coords.len() == 0 {
        return offset;
    }

    let center = y_coords.len() / 2;
    let y1 = y_coords[center];

    if have_vertical_sightline(bounding_boxes, x, y1, y2) {
        find_neg_vertical_cutoff(bounding_boxes, &y_coords[..center], x, y2, offset)
    } else {
        find_neg_vertical_cutoff(
            bounding_boxes,
            &y_coords[(center + 1)..],
            x,
            y2,
            offset + center + 1,
        )
    }
}

fn find_pos_vertical_cutoff(
    bounding_boxes: &[BoundingBox],
    y_coords: &[i32],
    x: i32,
    y1: i32,
    offset: usize,
) -> usize {
    if y_coords.len() == 0 {
        return offset;
    }

    let center = y_coords.len() / 2;
    let y2 = y_coords[center];

    if have_vertical_sightline(bounding_boxes, x, y1, y2) {
        find_pos_vertical_cutoff(
            bounding_boxes,
            &y_coords[(center + 1)..],
            x,
            y1,
            offset + center + 1,
        )
    } else {
        find_pos_vertical_cutoff(bounding_boxes, &y_coords[..center], x, y1, offset)
    }
}

#[derive(Default)]
pub struct Graph {
    x_coords: Vec<i32>,
    y_coords: Vec<i32>,
    node_map: HashMap<Point, u32>,
    nodes: NodeList,
}

impl Graph {
    pub fn build(&mut self, anchor_points: &[Point], bounding_boxes: &[BoundingBox]) {
        use std::collections::hash_map::Entry;

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

        self.node_map.clear();
        self.nodes.clear();

        macro_rules! node_index {
            ($point:expr) => {
                match self.node_map.entry($point) {
                    Entry::Occupied(entry) => (*entry.get(), true),
                    Entry::Vacant(entry) => {
                        let index = self.nodes.push($point);
                        entry.insert(index);
                        (index, false)
                    }
                }
            };
        }

        for anchor_point in anchor_points.iter().copied() {
            let (anchor_index, _) = node_index!(anchor_point);

            let x_index = self
                .x_coords
                .binary_search(&anchor_point.x)
                .expect("invalid anchor point");
            let y_index = self
                .y_coords
                .binary_search(&anchor_point.y)
                .expect("invalid anchor point");

            let cutoff = find_neg_horizontal_cutoff(
                bounding_boxes,
                &self.x_coords[..x_index],
                anchor_point.y,
                anchor_point.x,
                0,
            );
            let mut prev_index = anchor_index;
            for x in self.x_coords[cutoff..x_index].iter().copied().rev() {
                let current_point = Point {
                    x,
                    y: anchor_point.y,
                };

                let (current_index, existed) = node_index!(current_point);

                self.nodes[prev_index].neighbors[Direction::NegX] = current_index;
                self.nodes[current_index].neighbors[Direction::PosX] = prev_index;

                if existed
                    && (self.nodes[current_index].neighbors[Direction::NegX] != INVALID_INDEX)
                {
                    break;
                }

                prev_index = current_index;
            }

            let cutoff = find_pos_horizontal_cutoff(
                bounding_boxes,
                &self.x_coords[(x_index + 1)..],
                anchor_point.y,
                anchor_point.x,
                x_index + 1,
            );
            let mut prev_index = anchor_index;
            for x in self.x_coords[(x_index + 1)..cutoff].iter().copied() {
                let current_point = Point {
                    x,
                    y: anchor_point.y,
                };

                let (current_index, existed) = node_index!(current_point);

                self.nodes[prev_index].neighbors[Direction::PosX] = current_index;
                self.nodes[current_index].neighbors[Direction::NegX] = prev_index;

                if existed
                    && (self.nodes[current_index].neighbors[Direction::PosX] != INVALID_INDEX)
                {
                    break;
                }

                prev_index = current_index;
            }

            let cutoff = find_neg_vertical_cutoff(
                bounding_boxes,
                &self.y_coords[..y_index],
                anchor_point.x,
                anchor_point.y,
                0,
            );
            let mut prev_index = anchor_index;
            for y in self.y_coords[cutoff..y_index].iter().copied().rev() {
                let current_point = Point {
                    x: anchor_point.x,
                    y,
                };

                let (current_index, existed) = node_index!(current_point);

                self.nodes[prev_index].neighbors[Direction::NegY] = current_index;
                self.nodes[current_index].neighbors[Direction::PosY] = prev_index;

                if existed
                    && (self.nodes[current_index].neighbors[Direction::NegY] != INVALID_INDEX)
                {
                    break;
                }

                prev_index = current_index;
            }

            let cutoff = find_pos_vertical_cutoff(
                bounding_boxes,
                &self.y_coords[(y_index + 1)..],
                anchor_point.x,
                anchor_point.y,
                y_index + 1,
            );
            let mut prev_index = anchor_index;
            for y in self.y_coords[(y_index + 1)..cutoff].iter().copied() {
                let current_point = Point {
                    x: anchor_point.x,
                    y,
                };

                let (current_index, existed) = node_index!(current_point);

                self.nodes[prev_index].neighbors[Direction::PosY] = current_index;
                self.nodes[current_index].neighbors[Direction::NegY] = prev_index;

                if existed
                    && (self.nodes[current_index].neighbors[Direction::PosY] != INVALID_INDEX)
                {
                    break;
                }

                prev_index = current_index;
            }
        }
    }

    #[inline]
    fn find_node(&self, point: Point) -> Option<u32> {
        self.node_map.get(&point).copied()
    }
}

#[derive(Default)]
pub struct PathFinder {
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

    pub fn find_path(
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

fn extend_vertex_buffer(
    vertex_buffer: &mut VertexBuffer,
    vertex_buffer_capacity: usize,
    path: &[Point],
    net_id: u32,
) -> Result<(), RoutingResult> {
    if vertex_buffer_capacity < (vertex_buffer.vertex_count + path.len()) {
        return Err(RoutingResult::BufferOverflowError);
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
                extend_vertex_buffer(
                    vertex_buffer,
                    vertex_buffer_capacity,
                    &path,
                    path_def.net_id,
                )
            } else {
                extend_vertex_buffer(
                    vertex_buffer,
                    vertex_buffer_capacity,
                    &[path_def.start, path_def.end],
                    path_def.net_id,
                )
            }
        })
    });

    match result {
        Ok(_) => RoutingResult::Success,
        Err(err) => err,
    }
}
