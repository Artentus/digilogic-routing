use crate::*;

#[test]
fn straight_line() {
    let grid = Grid::new();
    let mut path = Vec::new();
    grid.find_path(
        &mut HashMap::default(),
        &mut HashMap::default(),
        &mut PriorityQueue::default(),
        &mut path,
        PathEndpoints {
            start: Point { x: 10, y: 10 },
            end: Point { x: 10, y: 20 },
        },
    );
    assert_eq!(path, [Point { x: 10, y: 10 }, Point { x: 10, y: 20 }])
}

#[test]
fn one_corner() {
    let grid = Grid::new();
    let mut path = Vec::new();
    grid.find_path(
        &mut HashMap::default(),
        &mut HashMap::default(),
        &mut PriorityQueue::default(),
        &mut path,
        PathEndpoints {
            start: Point { x: 10, y: 10 },
            end: Point { x: 20, y: 20 },
        },
    );
    assert_eq!(
        path,
        [
            Point { x: 10, y: 10 },
            Point { x: 10, y: 20 },
            Point { x: 20, y: 20 },
        ]
    )
}
