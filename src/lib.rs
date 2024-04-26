#![deny(unsafe_op_in_unsafe_fn)]

#[cfg(test)]
mod test;

type HashMap<K, V> = ahash::AHashMap<K, V>;
type PriorityQueue<I, P> = priority_queue::PriorityQueue<I, P, ahash::RandomState>;

#[derive(Debug, Clone, Copy)]
struct CellCoords {
    x: u8,
    y: u8,
}

type ChunkStorage = usize;
const CHUNK_SIZE: usize = ChunkStorage::BITS as usize;

#[derive(Debug, PartialEq, Eq)]
#[repr(transparent)]
struct Chunk {
    rows: [ChunkStorage; CHUNK_SIZE],
}

impl Chunk {
    const EMPTY: Self = Self {
        rows: [0; CHUNK_SIZE],
    };

    fn get(&self, coords: CellCoords) -> bool {
        let x = coords.x as usize;
        let y = coords.y as usize;

        assert!(x < CHUNK_SIZE);
        assert!(y < CHUNK_SIZE);

        let bit = 1 >> x;
        (self.rows[y] & bit) != 0
    }

    fn fill(&mut self, start: CellCoords, end: CellCoords) {
        let x_min = start.x as usize;
        let x_max = end.x as usize;
        let y_min = start.y as usize;
        let y_max = end.y as usize;

        assert!(x_min < x_max);
        assert!(y_min < y_max);
        assert!(x_max <= CHUNK_SIZE);
        assert!(y_max <= CHUNK_SIZE);

        let mask = (ChunkStorage::MAX << x_min) & (ChunkStorage::MAX >> (CHUNK_SIZE - x_max));
        for row in &mut self.rows[y_min..y_max] {
            *row |= mask;
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ChunkCoords {
    x: i32,
    y: i32,
}

impl Ord for ChunkCoords {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.y.cmp(&other.y).then(self.x.cmp(&other.x))
    }
}

impl PartialOrd for ChunkCoords {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn transform(self) -> (ChunkCoords, CellCoords) {
        (
            ChunkCoords {
                x: self.x.div_euclid(CHUNK_SIZE as i32),
                y: self.y.div_euclid(CHUNK_SIZE as i32),
            },
            CellCoords {
                x: self.x.rem_euclid(CHUNK_SIZE as i32) as u8,
                y: self.y.rem_euclid(CHUNK_SIZE as i32) as u8,
            },
        )
    }

    fn neighbors(self) -> [Self; 4] {
        [
            Self {
                x: self.x + 1,
                y: self.y,
            },
            Self {
                x: self.x - 1,
                y: self.y,
            },
            Self {
                x: self.x,
                y: self.y + 1,
            },
            Self {
                x: self.x,
                y: self.y - 1,
            },
        ]
    }

    fn manhatten_distance_to(self, other: Self) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct Rect {
    min: Point,
    max: Point,
}

struct Grid {
    chunk_coords: Vec<ChunkCoords>,
    chunks: Vec<Chunk>,
}

impl Grid {
    fn new() -> Self {
        Self {
            chunk_coords: Vec::new(),
            chunks: Vec::new(),
        }
    }

    fn get_chunk(&self, coords: ChunkCoords) -> &Chunk {
        assert_eq!(self.chunk_coords.len(), self.chunks.len());

        match self.chunk_coords.binary_search(&coords) {
            Ok(index) => &self.chunks[index],
            Err(_) => &Chunk::EMPTY,
        }
    }

    fn get_chunk_mut(&mut self, coords: ChunkCoords) -> &mut Chunk {
        assert_eq!(self.chunk_coords.len(), self.chunks.len());

        match self.chunk_coords.binary_search(&coords) {
            Ok(index) => &mut self.chunks[index],
            Err(index) => {
                self.chunk_coords.insert(index, coords);
                self.chunks.insert(index, Chunk::EMPTY);
                &mut self.chunks[index]
            }
        }
    }

    fn get(&self, point: Point) -> bool {
        let (chunk_coords, cell_coords) = point.transform();
        self.get_chunk(chunk_coords).get(cell_coords)
    }

    fn fill(&mut self, rect: Rect) {
        if (rect.min.x >= rect.max.x) || (rect.min.y >= rect.max.y) {
            return;
        }

        let (min_chunk_coords, min_cell_coords) = rect.min.transform();
        let (max_chunk_coords, max_cell_coords) = rect.max.transform();

        for chunk_y in min_chunk_coords.y..=max_chunk_coords.y {
            for chunk_x in min_chunk_coords.x..=max_chunk_coords.x {
                let chunk_coords = ChunkCoords {
                    x: chunk_x,
                    y: chunk_y,
                };

                let min_cell_coords = CellCoords {
                    x: if chunk_x == min_chunk_coords.x {
                        min_cell_coords.x
                    } else {
                        0
                    },
                    y: if chunk_y == min_chunk_coords.y {
                        min_cell_coords.y
                    } else {
                        0
                    },
                };

                let max_cell_coords = CellCoords {
                    x: if chunk_x == max_chunk_coords.x {
                        max_cell_coords.x
                    } else {
                        CHUNK_SIZE as u8
                    },
                    y: if chunk_y == max_chunk_coords.y {
                        max_cell_coords.y
                    } else {
                        CHUNK_SIZE as u8
                    },
                };

                if (max_cell_coords.x > 0) && (max_cell_coords.y > 0) {
                    let chunk = self.get_chunk_mut(chunk_coords);
                    chunk.fill(min_cell_coords, max_cell_coords);
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
struct PathEndpoints {
    start: Point,
    end: Point,
}

fn build_path(predecessor: &HashMap<Point, Point>, path: &mut Vec<Point>, start: Point) {
    path.push(start);

    let mut succ: Option<Point> = None;
    let mut current = start;
    while let Some(&pred) = predecessor.get(&current) {
        let replace = match succ {
            Some(succ) => {
                ((succ.x == current.x) && (succ.x == pred.x))
                    || ((succ.y == current.y) && (succ.y == pred.y))
            }
            None => false,
        };

        if replace {
            *path.last_mut().unwrap() = pred;
        } else {
            path.push(pred);
            succ = Some(current);
        }

        current = pred;
    }
}

impl Grid {
    fn find_path(
        &self,
        predecessor: &mut HashMap<Point, Point>,
        g_score: &mut HashMap<Point, u32>,
        open_queue: &mut PriorityQueue<Point, std::cmp::Reverse<u32>>,
        path: &mut Vec<Point>,
        endpoints: PathEndpoints,
    ) -> bool {
        g_score.insert(endpoints.end, 0);
        open_queue.push(endpoints.end, std::cmp::Reverse(0));

        while let Some((current, _)) = open_queue.pop() {
            if current == endpoints.start {
                build_path(predecessor, path, endpoints.start);
                return true;
            }

            for neighbor in current.neighbors() {
                if self.get(neighbor) {
                    continue;
                }

                let new_g_score = g_score[&current]
                    + if let Some(pred) = predecessor.get(&current) {
                        if ((pred.x - current.x) == (current.x - neighbor.x))
                            && ((pred.y - current.y) == (current.y - neighbor.y))
                        {
                            1
                        } else {
                            2
                        }
                    } else {
                        2
                    };

                let update = match g_score.get(&neighbor) {
                    Some(&old_g_score) => new_g_score < old_g_score,
                    None => true,
                };

                if update {
                    let new_f_score = new_g_score + neighbor.manhatten_distance_to(endpoints.start);

                    predecessor.insert(neighbor, current);
                    g_score.insert(neighbor, new_g_score);
                    open_queue.push(neighbor, std::cmp::Reverse(new_f_score));
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
unsafe extern "C" fn grid_new(grid: *mut *mut Grid) -> RoutingResult {
    if grid.is_null() {
        return RoutingResult::NullPointerError;
    }

    let ptr = Box::into_raw(Box::new(Grid::new()));
    unsafe {
        grid.write(ptr);
    }

    RoutingResult::Success
}

#[no_mangle]
#[must_use]
unsafe extern "C" fn grid_free(grid: *mut Grid) -> RoutingResult {
    if grid.is_null() {
        return RoutingResult::NullPointerError;
    }

    let grid = unsafe { Box::from_raw(grid) };
    std::mem::drop(grid);

    RoutingResult::Success
}

#[no_mangle]
#[must_use]
unsafe extern "C" fn grid_fill(grid: *mut Grid, rect: Rect) -> RoutingResult {
    if grid.is_null() {
        return RoutingResult::NullPointerError;
    }

    unsafe {
        (*grid).fill(rect);
    }

    RoutingResult::Success
}

#[derive(Clone, Copy)]
#[repr(C)]
struct Vertex {
    id: u32,
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
unsafe extern "C" fn grid_find_paths(
    grid: *const Grid,
    paths: *const PathEndpoints,
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

    if grid.is_null() || paths.is_null() || vertex_buffers.is_null() {
        return RoutingResult::NullPointerError;
    }

    let grid = unsafe { &*grid };
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

    let result = paths.par_iter().copied().enumerate().try_for_each_init(
        || {
            let buffer_index = buffer_index.fetch_add(1, Ordering::Relaxed);
            assert!(buffer_index < rayon::current_num_threads());

            let vertex_buffers = vertex_buffers;
            let vertex_buffer = unsafe { vertex_buffers.0.add(buffer_index) };
            let vertex_buffer = SyncPtr(vertex_buffer);

            (
                HashMap::default(),
                HashMap::default(),
                PriorityQueue::default(),
                Vec::default(),
                vertex_buffer,
            )
        },
        |(predecessor, g_score, open_queue, path, vertex_buffer), (id, endpoints)| {
            let vertex_buffer = unsafe { &mut *vertex_buffer.0 };

            predecessor.clear();
            g_score.clear();
            open_queue.clear();
            path.clear();

            let id = id as u32;
            if grid.find_path(predecessor, g_score, open_queue, path, endpoints) {
                if vertex_buffer_capacity < (vertex_buffer.len + path.len()) {
                    return Err(RoutingResult::BufferOverflowError);
                }

                for (i, point) in path.iter().copied().enumerate() {
                    unsafe {
                        vertex_buffer
                            .vertices
                            .add(vertex_buffer.len + i)
                            .write(Vertex {
                                id,
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
                            id,
                            x: endpoints.start.x as f32,
                            y: endpoints.start.y as f32,
                        });

                    vertex_buffer
                        .vertices
                        .add(vertex_buffer.len + 1)
                        .write(Vertex {
                            id,
                            x: endpoints.end.x as f32,
                            y: endpoints.end.y as f32,
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
