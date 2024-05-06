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

#[cfg(test)]
mod visual {
    use crate::*;
    use svg::node::element::{Circle, Line, Rectangle, Script};

    include!("../test_data/graph.rs");

    fn svg_out(anchors: &[Anchor], bounding_boxes: &[BoundingBox], graph: &Graph, path: &str) {
        use std::fmt::Write;

        let anchors: ahash::AHashSet<_> = anchors.iter().map(|anchor| anchor.position).collect();
        let nodes = graph.nodes();

        let mut document = svg::Document::new()
            .set("style", "background-color:#303030")
            .add(Script::new(include_str!("../test_data/svg.js")));

        let mut min_x = i32::MAX;
        let mut min_y = i32::MAX;
        let mut max_x = i32::MIN;
        let mut max_y = i32::MIN;

        for bb in bounding_boxes {
            min_x = min_x.min(bb.min_x());
            min_y = min_y.min(bb.min_y());
            max_x = max_x.max(bb.max_x());
            max_y = max_y.max(bb.max_y());

            document = document.add(
                Rectangle::new()
                    .set("x", bb.min_x())
                    .set("y", bb.min_y())
                    .set("width", bb.width())
                    .set("height", bb.height())
                    .set("stroke", "coral")
                    .set("fill", "none"),
            );
        }

        for node in nodes {
            if let Some(pos_x) = node.get_neighbor(Direction::PosX) {
                let pos_x = &nodes[pos_x];

                document = document.add(
                    Line::new()
                        .set("x1", node.position.x)
                        .set("y1", node.position.y)
                        .set("x2", pos_x.position.x)
                        .set("y2", pos_x.position.y)
                        .set("stroke", "lightblue"),
                );
            }

            if let Some(pos_y) = node.get_neighbor(Direction::PosY) {
                let pos_y = &nodes[pos_y];

                document = document.add(
                    Line::new()
                        .set("x1", node.position.x)
                        .set("y1", node.position.y)
                        .set("x2", pos_y.position.x)
                        .set("y2", pos_y.position.y)
                        .set("stroke", "lightblue"),
                );
            }
        }

        for (index, node) in nodes.iter().enumerate() {
            min_x = min_x.min(node.position.x);
            min_y = min_y.min(node.position.y);
            max_x = max_x.max(node.position.x);
            max_y = max_y.max(node.position.y);

            let (radius, fill) = if anchors.contains(&node.position) {
                (2.0, "dodgerblue")
            } else {
                (1.5, "lightskyblue")
            };

            let mut class = String::new();
            for neighbor_index in Direction::ALL
                .iter()
                .filter_map(|&dir| node.get_neighbor(dir))
            {
                if class.len() > 0 {
                    write!(class, " ").unwrap();
                }

                write!(class, "neighbor-of-anchor{neighbor_index}").unwrap();
            }

            document = document.add(
                Circle::new()
                    .set("id", format!("anchor{index}"))
                    .set("class", class)
                    .set("cx", node.position.x)
                    .set("cy", node.position.y)
                    .set("r", radius)
                    .set("fill", fill)
                    .set("stroke", "none")
                    .set("onmouseenter", "anchorMouseEnter(this)")
                    .set("onmouseleave", "anchorMouseLeave(this)"),
            );
        }

        document = document.set(
            "viewBox",
            (
                min_x - 10,
                min_y - 10,
                max_x - min_x + 20,
                max_y - min_y + 20,
            ),
        );

        svg::save(path, &document).unwrap();
    }

    #[test]
    fn fast() {
        let mut graph = Graph::default();
        graph.build(ANCHORS, BOUNDING_BOXES, false);
        svg_out(ANCHORS, BOUNDING_BOXES, &graph, "graph_fast.svg");
    }

    #[test]
    fn minimal() {
        let mut graph = Graph::default();
        graph.build(ANCHORS, BOUNDING_BOXES, true);
        svg_out(ANCHORS, BOUNDING_BOXES, &graph, "graph_minimal.svg");
    }
}
