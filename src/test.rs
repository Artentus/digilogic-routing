#![allow(unsafe_code)]

use crate::ffi::*;
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

fn test_impl(
    graph: &Graph,
    paths: &[PathDef],
    vertex_buffer_capacity: usize,
    expected: &[&[Vertex]],
) {
    let graph = graph as *const _;
    let path_count = paths.len();
    let paths = paths.as_ptr();
    let vertex_buffer_count = rayon::current_num_threads();

    let mut vertex_buffers = Vec::new();
    for _ in 0..vertex_buffer_count {
        let mut vertices = Vec::new();
        vertices.reserve_exact(vertex_buffer_capacity);
        let ptr = vertices.as_mut_ptr();
        std::mem::forget(vertices);

        vertex_buffers.push(VertexBuffer {
            vertices: ptr,
            vertex_count: 0,
        });
    }

    let result = unsafe {
        RT_graph_find_paths(
            graph,
            paths,
            path_count,
            vertex_buffers.as_mut_ptr(),
            vertex_buffer_capacity,
        )
    };

    assert_eq!(result, Result::Success);

    let mut expected_matches = vec![false; expected.len()];
    for vertex_buffer in vertex_buffers {
        let vertices = unsafe {
            Vec::from_raw_parts(
                vertex_buffer.vertices,
                vertex_buffer.vertex_count,
                vertex_buffer_capacity,
            )
        };

        println!("{vertices:#?}");

        for (i, expected) in expected.iter().enumerate() {
            if expected_matches[i] {
                continue;
            }

            if expected.iter().all(|vertex| vertices.contains(vertex)) {
                expected_matches[i] = true;
            }
        }
    }

    assert!(expected_matches.iter().all(|i| *i))
}

#[test]
fn straight() {
    let mut graph = Graph::default();
    graph.build(POINTS, &[]);

    test_impl(
        &graph,
        &[PathDef {
            net_id: 0,
            start: Point { x: 0, y: 2 },
            end: Point { x: 4, y: 2 },
        }],
        2,
        &[&[
            Vertex {
                net_id: 0,
                x: 0.0,
                y: 2.0,
            },
            Vertex {
                net_id: 0,
                x: 4.0,
                y: 2.0,
            },
        ]],
    );
}

#[test]
fn one_bend() {
    let mut graph = Graph::default();
    graph.build(POINTS, &[]);

    test_impl(
        &graph,
        &[PathDef {
            net_id: 0,
            start: Point { x: 0, y: 0 },
            end: Point { x: 4, y: 4 },
        }],
        3,
        &[&[
            Vertex {
                net_id: 0,
                x: 0.0,
                y: 0.0,
            },
            Vertex {
                net_id: 0,
                x: 0.0,
                y: 4.0,
            },
            Vertex {
                net_id: 0,
                x: 4.0,
                y: 4.0,
            },
        ]],
    );
}

#[test]
fn two_bends() {
    let mut graph = Graph::default();
    graph.build(
        POINTS,
        &[BoundingBox {
            center: Point { x: 2, y: 1 },
            half_width: 1,
            half_height: 2,
        }],
    );

    test_impl(
        &graph,
        &[PathDef {
            net_id: 0,
            start: Point { x: 0, y: 0 },
            end: Point { x: 4, y: 0 },
        }],
        4,
        &[&[
            Vertex {
                net_id: 0,
                x: 0.0,
                y: 0.0,
            },
            Vertex {
                net_id: 0,
                x: 0.0,
                y: 4.0,
            },
            Vertex {
                net_id: 0,
                x: 4.0,
                y: 4.0,
            },
            Vertex {
                net_id: 0,
                x: 4.0,
                y: 0.0,
            },
        ]],
    );
}
