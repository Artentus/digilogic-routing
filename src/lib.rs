#![deny(unsafe_op_in_unsafe_fn)]

#[cfg(test)]
mod test;

use std::ops::{Index, IndexMut};

type PriorityQueue<I, P> = priority_queue::PriorityQueue<I, P, ahash::RandomState>;

const INVALID_INDEX: u32 = u32::MAX;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn manhatten_distance_to(self, other: Self) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
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

    fn opposite(self) -> Self {
        match self {
            Direction::PosX => Direction::NegX,
            Direction::NegX => Direction::PosX,
            Direction::PosY => Direction::NegY,
            Direction::NegY => Direction::PosY,
        }
    }
}

#[derive(Clone)]
#[repr(transparent)]
struct NeighborList([u32; 4]);

impl NeighborList {
    fn new() -> Self {
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
struct Node {
    point: Point,
    neighbors: NeighborList,
    g_score: u32,
    predecessor: u32,
}

impl Node {
    fn new(point: Point) -> Self {
        Self {
            point,
            neighbors: NeighborList::new(),
            g_score: u32::MAX,
            predecessor: INVALID_INDEX,
        }
    }

    fn reset(&mut self) {
        self.g_score = u32::MAX;
        self.predecessor = INVALID_INDEX;
    }
}

#[derive(Clone)]
#[repr(transparent)]
struct NodeList(Vec<Node>);

impl Index<u32> for NodeList {
    type Output = Node;

    fn index(&self, index: u32) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl IndexMut<u32> for NodeList {
    fn index_mut(&mut self, index: u32) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}

#[derive(Clone)]
struct Graph {
    nodes: NodeList,
}

impl Graph {
    fn build(anchor_points: &[Point], have_sightline: impl Fn(Point, Point) -> bool) -> Self {
        let mut x_coords: Vec<_> = anchor_points.iter().map(|&Point { x, .. }| x).collect();
        x_coords.sort_unstable();
        x_coords.dedup();

        let mut y_coords: Vec<_> = anchor_points.iter().map(|&Point { y, .. }| y).collect();
        y_coords.sort_unstable();
        y_coords.dedup();

        let node_count = x_coords.len() * y_coords.len();
        assert!(node_count <= (INVALID_INDEX as usize));
        let mut nodes = Vec::with_capacity(node_count);

        for &y in y_coords.iter() {
            let mut prev: Option<usize> = None;
            for &x in x_coords.iter() {
                let index = nodes.len();
                let point = Point { x, y };
                nodes.push(Node::new(point));

                if let Some(prev) = prev {
                    if have_sightline(point, nodes[prev].point) {
                        nodes[prev].neighbors[Direction::PosX] = index as u32;
                        nodes[index].neighbors[Direction::NegX] = prev as u32;
                    }
                }

                prev = Some(index);
            }
        }

        for (xi, &x) in x_coords.iter().enumerate() {
            let mut prev: Option<usize> = None;
            for (yi, &y) in y_coords.iter().enumerate() {
                let index = yi * x_coords.len() + xi;
                let point = Point { x, y };

                if let Some(prev) = prev {
                    if have_sightline(point, nodes[prev].point) {
                        nodes[prev].neighbors[Direction::PosY] = index as u32;
                        nodes[index].neighbors[Direction::NegY] = prev as u32;
                    }
                }

                prev = Some(index);
            }
        }

        Self {
            nodes: NodeList(nodes),
        }
    }

    fn reset(&mut self) {
        for node in self.nodes.0.iter_mut() {
            node.reset();
        }
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

    fn build_path(&self, path: &mut Vec<Point>, start_index: u32) {
        path.push(self.nodes[start_index].point);

        let mut dir: Option<Direction> = None;
        let mut current_index = start_index;
        loop {
            let pred_index = self.nodes[current_index].predecessor;
            if pred_index == INVALID_INDEX {
                break;
            }

            let pred_dir = self.nodes[current_index]
                .neighbors
                .find(pred_index)
                .expect("invalid predecessor");

            if Some(pred_dir) == dir {
                *path.last_mut().unwrap() = self.nodes[pred_index].point;
            } else {
                path.push(self.nodes[pred_index].point);
                dir = Some(pred_dir);
            }

            current_index = pred_index;
        }
    }

    fn find_path(
        &mut self,
        open_queue: &mut PriorityQueue<u32, std::cmp::Reverse<u32>>,
        path: &mut Vec<Point>,
        start: Point,
        end: Point,
    ) -> bool {
        self.reset();

        let start_index = self.find_node(start).expect("invalid start node");
        let end_index = self.find_node(end).expect("invalid end node");

        macro_rules! start {
            () => {
                self.nodes[start_index]
            };
        }

        macro_rules! end {
            () => {
                self.nodes[end_index]
            };
        }

        end!().g_score = 0;
        open_queue.push(end_index, std::cmp::Reverse(0));

        while let Some((current_index, _)) = open_queue.pop() {
            if current_index == start_index {
                self.build_path(path, start_index);
                return true;
            }

            macro_rules! current {
                () => {
                    self.nodes[current_index]
                };
            }

            let pred_index = current!().predecessor;
            let pred_dir = current!().neighbors.find(pred_index);
            let straight_dir = pred_dir.map(Direction::opposite);

            for dir in Direction::ALL {
                let neighbor_index = current!().neighbors[dir];
                if neighbor_index == INVALID_INDEX {
                    continue;
                }

                macro_rules! neighbor {
                    () => {
                        self.nodes[neighbor_index]
                    };
                }

                let new_g_score =
                    current!().g_score + if Some(dir) == straight_dir { 1 } else { 2 };

                if new_g_score < neighbor!().g_score {
                    neighbor!().g_score = new_g_score;
                    neighbor!().predecessor = current_index;

                    let new_f_score =
                        new_g_score + neighbor!().point.manhatten_distance_to(start!().point);
                    open_queue.push(neighbor_index, std::cmp::Reverse(new_f_score));
                }
            }
        }

        false
    }
}

#[repr(u32)]
enum RoutingResult {
    Success = 0,
    NullPointerError = 1,
    InvalidOperationError = 2,
    BufferOverflowError = 3,
}

#[no_mangle]
#[must_use]
unsafe extern "C" fn init_thread_pool(thread_count: *mut usize) -> RoutingResult {
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
unsafe extern "C" fn graph_build(
    anchor_points: *const Point,
    anchor_point_count: usize,
    have_sightline: Option<unsafe extern "C" fn(Point, Point) -> bool>,
    graph: *mut *mut Graph,
) -> RoutingResult {
    if anchor_points.is_null() || graph.is_null() {
        return RoutingResult::NullPointerError;
    }

    let Some(have_sightline) = have_sightline else {
        return RoutingResult::NullPointerError;
    };

    let anchor_points = unsafe { std::slice::from_raw_parts(anchor_points, anchor_point_count) };
    let have_sightline = |a, b| unsafe { have_sightline(a, b) };
    let ptr = Box::into_raw(Box::new(Graph::build(anchor_points, have_sightline)));
    unsafe {
        graph.write(ptr);
    }

    RoutingResult::Success
}

#[no_mangle]
#[must_use]
unsafe extern "C" fn graph_free(graph: *mut Graph) -> RoutingResult {
    if graph.is_null() {
        return RoutingResult::NullPointerError;
    }

    let graph = unsafe { Box::from_raw(graph) };
    std::mem::drop(graph);

    RoutingResult::Success
}

#[derive(Clone, Copy)]
#[repr(C)]
struct PathDef {
    net_id: u32,
    start: Point,
    end: Point,
}

#[derive(Clone, Copy)]
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
    len: usize,
}

#[no_mangle]
#[must_use]
unsafe extern "C" fn graph_find_paths(
    graph: *const Graph,
    paths: *const PathDef,
    path_count: usize,
    vertex_buffers: *mut VertexBuffer,
    vertex_buffer_capacity: usize,
) -> RoutingResult {
    #[derive(Clone, Copy)]
    #[repr(transparent)]
    struct SyncPtr<T: ?Sized>(*mut T);

    unsafe impl<T: ?Sized> Send for SyncPtr<T> {}
    unsafe impl<T: ?Sized> Sync for SyncPtr<T> {}

    use rayon::prelude::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    if graph.is_null() || paths.is_null() || vertex_buffers.is_null() {
        return RoutingResult::NullPointerError;
    }

    let graph = unsafe { &*graph };
    let paths = unsafe { std::slice::from_raw_parts(paths, path_count) };

    {
        let vertex_buffers =
            unsafe { std::slice::from_raw_parts_mut(vertex_buffers, rayon::current_num_threads()) };

        for vertex_buffer in vertex_buffers {
            if vertex_buffer.vertices.is_null() {
                return RoutingResult::NullPointerError;
            }

            vertex_buffer.len = 0;
        }
    }

    let vertex_buffers = SyncPtr(vertex_buffers);
    let buffer_index = AtomicUsize::new(0);

    let result = paths.par_iter().copied().try_for_each_init(
        || {
            let buffer_index = buffer_index.fetch_add(1, Ordering::Relaxed);
            assert!(buffer_index < rayon::current_num_threads());

            let vertex_buffers = vertex_buffers;
            let vertex_buffer = unsafe { vertex_buffers.0.add(buffer_index) };
            let vertex_buffer = SyncPtr(vertex_buffer);

            (
                graph.clone(),
                PriorityQueue::default(),
                Vec::default(),
                vertex_buffer,
            )
        },
        |(graph, open_queue, path, vertex_buffer), path_def| {
            let vertex_buffer = unsafe { &mut *vertex_buffer.0 };

            open_queue.clear();
            path.clear();

            if graph.find_path(open_queue, path, path_def.start, path_def.end) {
                if vertex_buffer_capacity < (vertex_buffer.len + path.len()) {
                    return Err(RoutingResult::BufferOverflowError);
                }

                for (i, point) in path.iter().copied().enumerate() {
                    unsafe {
                        vertex_buffer
                            .vertices
                            .add(vertex_buffer.len + i)
                            .write(Vertex {
                                net_id: path_def.net_id,
                                x: point.x as f32,
                                y: point.y as f32,
                            });
                    }
                }
            } else {
                if vertex_buffer_capacity < (vertex_buffer.len + 2) {
                    return Err(RoutingResult::BufferOverflowError);
                }

                unsafe {
                    vertex_buffer
                        .vertices
                        .add(vertex_buffer.len + 0)
                        .write(Vertex {
                            net_id: path_def.net_id,
                            x: path_def.start.x as f32,
                            y: path_def.start.y as f32,
                        });

                    vertex_buffer
                        .vertices
                        .add(vertex_buffer.len + 1)
                        .write(Vertex {
                            net_id: path_def.net_id,
                            x: path_def.end.x as f32,
                            y: path_def.end.y as f32,
                        });
                }
            }

            Ok(())
        },
    );

    match result {
        Ok(_) => RoutingResult::Success,
        Err(err) => err,
    }
}
