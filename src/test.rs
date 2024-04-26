mod grid;
mod pathfinding;

use crate::*;

const fn rect(min_x: i32, min_y: i32, max_x: i32, max_y: i32) -> Rect {
    Rect {
        min: Point { x: min_x, y: min_y },
        max: Point { x: max_x, y: max_y },
    }
}

const fn chunk(repr: &[&[u8; CHUNK_SIZE]; CHUNK_SIZE]) -> Chunk {
    let mut chunk = Chunk::EMPTY;

    let mut y = 0;
    while y < CHUNK_SIZE {
        let mut x = 0;
        while x < CHUNK_SIZE {
            match repr[y][x] {
                b'#' => chunk.rows[y] |= 1 << x,
                b'.' => (),
                _ => panic!("illegal char"),
            }

            x += 1;
        }

        y += 1;
    }

    chunk
}
