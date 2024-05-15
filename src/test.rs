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

fn init() -> usize {
    use std::sync::Once;

    static INIT: Once = Once::new();
    INIT.call_once(|| {
        let result = unsafe { RT_init_thread_pool() };
        assert_eq!(result, Result::Success);
    });

    let mut thread_count = 0u16;
    let result = unsafe { RT_get_thread_count((&mut thread_count) as *mut _) };
    assert_eq!(result, Result::Success);
    assert_ne!(thread_count, 0);
    thread_count as usize
}

impl<'a, T> From<&'a [T]> for Slice<T> {
    fn from(value: &'a [T]) -> Self {
        Slice {
            ptr: value.as_ptr(),
            len: value.len(),
        }
    }
}

impl<'a, T> From<&'a mut [T]> for MutSlice<T> {
    fn from(value: &'a mut [T]) -> Self {
        MutSlice {
            ptr: value.as_mut_ptr(),
            len: value.len(),
        }
    }
}

fn test_impl(graph: &Graph, net_points: [Point; 2], expected: &[Vertex]) {
    let thread_count = init();

    let nets = [Net {
        first_endpoint: 0,
        first_waypoint: INVALID_WAYPOINT_INDEX,
    }];

    let endpoints = [
        Endpoint {
            position: net_points[0],
            next: 1,
        },
        Endpoint {
            position: net_points[1],
            next: INVALID_ENDPOINT_INDEX,
        },
    ];

    let mut vertices = vec![Vertex::default(); expected.len() * thread_count];
    let mut wire_views = vec![WireView::default(); thread_count];
    let mut net_views = vec![NetView::default(); 1];

    let result = unsafe {
        RT_graph_connect_nets(
            graph as *const _,
            nets.as_slice().into(),
            endpoints.as_slice().into(),
            [].as_slice().into(),
            vertices.as_mut_slice().into(),
            wire_views.as_mut_slice().into(),
            net_views.as_mut_slice().into(),
        )
    };

    assert_eq!(result, Result::Success);

    let net_view = &net_views[0];
    assert_eq!(net_view.wire_count, 1);

    let wire_view = &wire_views[net_view.wire_offset as usize];
    assert_eq!(wire_view.vertex_count as usize, expected.len());

    let vertices = &vertices[(net_view.vertex_offset as usize)..(wire_view.vertex_count as usize)];
    assert_eq!(vertices, expected);
}

fn straight_impl(minimal: bool) {
    let mut graph = Graph::default();
    graph.build(ANCHORS, &[], minimal);

    test_impl(
        &graph,
        [Point { x: 0, y: 2 }, Point { x: 4, y: 2 }],
        &[Vertex { x: 0.0, y: 2.0 }, Vertex { x: 4.0, y: 2.0 }],
    );
}

fn one_bend_impl(minimal: bool) {
    let mut graph = Graph::default();
    graph.build(ANCHORS, &[], minimal);

    test_impl(
        &graph,
        [Point { x: 0, y: 0 }, Point { x: 4, y: 4 }],
        &[
            Vertex { x: 0.0, y: 0.0 },
            Vertex { x: 4.0, y: 0.0 },
            Vertex { x: 4.0, y: 4.0 },
        ],
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
        [Point { x: 0, y: 0 }, Point { x: 4, y: 0 }],
        &[
            Vertex { x: 0.0, y: 0.0 },
            Vertex { x: 0.0, y: 4.0 },
            Vertex { x: 4.0, y: 4.0 },
            Vertex { x: 4.0, y: 0.0 },
        ],
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

    include!("../test_data/graph.rs");

    fn svg_out(anchors: &[Anchor], bounding_boxes: &[BoundingBox], graph: &Graph, path: &str) {
        use std::fmt::Write;
        use svg::node::element::{Circle, Line, Rectangle, Script};

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
