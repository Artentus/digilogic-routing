use crate::*;

const POINTS: &[Point] = &[
    Point { x: 0, y: 0 },
    Point { x: 1, y: 0 },
    Point { x: 2, y: 0 },
    Point { x: 3, y: 0 },
    Point { x: 4, y: 0 },
    Point { x: 0, y: 0 },
    Point { x: 0, y: 1 },
    Point { x: 0, y: 2 },
    Point { x: 0, y: 3 },
    Point { x: 0, y: 4 },
];

#[test]
fn straight() {
    let mut graph = Graph::build(POINTS, |_, _| true);
    let mut path = Vec::new();

    assert!(graph.find_path(
        &mut PriorityQueue::default(),
        &mut path,
        Point { x: 0, y: 2 },
        Point { x: 4, y: 2 },
    ));

    assert_eq!(path, [Point { x: 0, y: 2 }, Point { x: 4, y: 2 }]);
}

#[test]
fn one_bend() {
    let mut graph = Graph::build(POINTS, |_, _| true);
    let mut path = Vec::new();

    assert!(graph.find_path(
        &mut PriorityQueue::default(),
        &mut path,
        Point { x: 0, y: 0 },
        Point { x: 4, y: 4 },
    ));

    assert_eq!(
        path,
        [
            Point { x: 0, y: 0 },
            Point { x: 0, y: 4 },
            Point { x: 4, y: 4 },
        ],
    );
}

#[test]
fn two_bends() {
    const EXCLUDE_EDGES: &[(Point, Point)] = &[
        (Point { x: 3, y: 0 }, Point { x: 4, y: 0 }),
        (Point { x: 3, y: 1 }, Point { x: 4, y: 1 }),
        (Point { x: 3, y: 2 }, Point { x: 4, y: 2 }),
        (Point { x: 3, y: 3 }, Point { x: 4, y: 3 }),
    ];

    let mut graph = Graph::build(POINTS, |a, b| {
        !EXCLUDE_EDGES.contains(&(a, b)) && !EXCLUDE_EDGES.contains(&(b, a))
    });
    let mut path = Vec::new();

    assert!(graph.find_path(
        &mut PriorityQueue::default(),
        &mut path,
        Point { x: 0, y: 0 },
        Point { x: 4, y: 0 },
    ));

    assert_eq!(
        path,
        [
            Point { x: 0, y: 0 },
            Point { x: 0, y: 4 },
            Point { x: 4, y: 4 },
            Point { x: 4, y: 0 },
        ],
    );
}
