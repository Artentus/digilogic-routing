#![allow(unsafe_code)]

use crate::ffi::*;
use crate::*;

const ANCHORS: &[Anchor] = &[
    Anchor::new(0, 0),
    Anchor::new(1, 0),
    Anchor::new(2, 0),
    Anchor::new(3, 0),
    Anchor::new(4, 0),
    Anchor::new(0, 0),
    Anchor::new(0, 1),
    Anchor::new(0, 2),
    Anchor::new(0, 3),
    Anchor::new(0, 4),
];

fn test_impl(
    graph: &Graph,
    paths: &[PathDef],
    vertex_buffer_capacity: u32,
    expected: &[&[Vertex]],
) {
    use std::sync::Once;

    static INIT: Once = Once::new();
    INIT.call_once(|| {
        let result = unsafe { RT_init_thread_pool() };
        assert_eq!(result, Result::Success);
    });

    let mut path_ranges = vec![
        PathRange {
            vertex_offset: 0,
            vertex_count: 0,
            vertex_buffer_index: 0
        };
        paths.len()
    ];

    let vertex_buffer_count = {
        let mut thread_count = 0u16;
        let result = unsafe { RT_get_thread_count((&mut thread_count) as *mut _) };
        assert_eq!(result, Result::Success);
        assert_ne!(thread_count, 0);
        thread_count
    };

    let mut vertex_buffers = Vec::new();
    for _ in 0..vertex_buffer_count {
        let mut vertices = Vec::new();
        vertices.reserve_exact(vertex_buffer_capacity as usize);
        let ptr = vertices.as_mut_ptr();
        std::mem::forget(vertices);

        vertex_buffers.push(VertexBuffer {
            vertices: ptr,
            vertex_count: 0,
        });
    }

    let result = {
        let graph = graph as *const _;
        let path_count: u32 = paths.len().try_into().expect("too many paths");
        let paths = paths.as_ptr();
        let path_ranges = path_ranges.as_mut_ptr();

        unsafe {
            RT_graph_find_paths(
                graph,
                paths,
                path_ranges,
                path_count,
                vertex_buffers.as_mut_ptr(),
                vertex_buffer_capacity,
            )
        }
    };

    assert_eq!(result, Result::Success);

    let vertex_buffers: Vec<_> = vertex_buffers
        .into_iter()
        .map(|vertex_buffer| unsafe {
            Vec::from_raw_parts(
                vertex_buffer.vertices,
                vertex_buffer.vertex_count as usize,
                vertex_buffer_capacity as usize,
            )
        })
        .collect();

    assert!(expected.iter().enumerate().all(|(i, &expected)| {
        let range = path_ranges[i];

        let start = range.vertex_offset as usize;
        let end = start + (range.vertex_count as usize);

        let actual = &vertex_buffers[range.vertex_buffer_index as usize][start..end];
        actual.eq(expected)
    }));
}

fn straight_impl(minimal: bool) {
    let mut graph = Graph::default();
    graph.build(ANCHORS, &[], minimal);

    test_impl(
        &graph,
        &[PathDef {
            start: Point { x: 0, y: 2 },
            end: Point { x: 4, y: 2 },
        }],
        2,
        &[&[Vertex { x: 0.0, y: 2.0 }, Vertex { x: 4.0, y: 2.0 }]],
    );
}

fn one_bend_impl(minimal: bool) {
    let mut graph = Graph::default();
    graph.build(ANCHORS, &[], minimal);

    test_impl(
        &graph,
        &[PathDef {
            start: Point { x: 0, y: 0 },
            end: Point { x: 4, y: 4 },
        }],
        3,
        &[&[
            Vertex { x: 0.0, y: 0.0 },
            Vertex { x: 0.0, y: 4.0 },
            Vertex { x: 4.0, y: 4.0 },
        ]],
    );
}

fn two_bends_impl(minimal: bool) {
    let mut graph = Graph::default();
    graph.build(
        ANCHORS,
        &[BoundingBox {
            center: Point { x: 2, y: 1 },
            half_width: 1,
            half_height: 2,
        }],
        minimal,
    );

    test_impl(
        &graph,
        &[PathDef {
            start: Point { x: 0, y: 0 },
            end: Point { x: 4, y: 0 },
        }],
        4,
        &[&[
            Vertex { x: 0.0, y: 0.0 },
            Vertex { x: 0.0, y: 4.0 },
            Vertex { x: 4.0, y: 4.0 },
            Vertex { x: 4.0, y: 0.0 },
        ]],
    );
}

#[test]
fn straight() {
    straight_impl(false);
}

#[test]
fn straight_minimal() {
    straight_impl(true);
}

#[test]
fn one_bend() {
    one_bend_impl(false);
}

#[test]
fn one_bend_minimal() {
    one_bend_impl(true);
}

#[test]
fn two_bends() {
    two_bends_impl(false);
}

#[test]
fn two_bends_minimal() {
    two_bends_impl(true);
}
