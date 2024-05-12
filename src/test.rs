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

fn init() -> u16 {
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
    thread_count
}

fn test_impl(
    graph: &Graph,
    net_points: [Point; 2],
    vertex_buffer_capacity: u32,
    expected: &[Vertex],
) {
    let vertex_buffer_count = init();

    let mut path_lengths = [0u16];
    let mut net = Net {
        points: net_points.as_ptr(),
        path_lengths: path_lengths.as_mut_ptr(),
        point_count: net_points.len() as u16,
        vertex_buffer_index: 0,
        vertex_offset: 0,
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

    let result = unsafe {
        RT_graph_connect_nets(
            graph as *const _,
            (&mut net) as *mut _,
            1,
            vertex_buffers.as_mut_ptr(),
            vertex_buffer_capacity,
        )
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

    assert_eq!(path_lengths[0] as usize, expected.len());

    let start = net.vertex_offset as usize;
    let end = start + expected.len();
    let actual = &vertex_buffers[net.vertex_buffer_index as usize][start..end];
    assert_eq!(actual, expected);
}

fn straight_impl(minimal: bool) {
    let mut graph = Graph::default();
    graph.build(ANCHORS, &[], minimal);

    test_impl(
        &graph,
        [Point { x: 0, y: 2 }, Point { x: 4, y: 2 }],
        2,
        &[Vertex { x: 4.0, y: 2.0 }, Vertex { x: 0.0, y: 2.0 }],
    );
}

fn one_bend_impl(minimal: bool) {
    let mut graph = Graph::default();
    graph.build(ANCHORS, &[], minimal);

    test_impl(
        &graph,
        [Point { x: 0, y: 0 }, Point { x: 4, y: 4 }],
        3,
        &[
            Vertex { x: 4.0, y: 4.0 },
            Vertex { x: 4.0, y: 0.0 },
            Vertex { x: 0.0, y: 0.0 },
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
        12,
        &[
            Vertex { x: 4.0, y: 0.0 },
            Vertex { x: 4.0, y: 4.0 },
            Vertex { x: 0.0, y: 4.0 },
            Vertex { x: 0.0, y: 0.0 },
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
    use crate::ffi::*;
    use crate::*;

    include!("../test_data/graph.rs");

    fn svg_out(
        anchors: &[Anchor],
        bounding_boxes: &[BoundingBox],
        graph: &Graph,
        routing_data: Option<(&[Net], &[Vec<Point>], &[Vec<u16>], &[Vec<Vertex>])>,
        path: &str,
    ) {
        use std::fmt::Write;
        use svg::node::element::{path, Circle, Line, Path, Rectangle, Script};

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

        if let Some((nets, net_points, net_path_lengths, vertex_buffers)) = routing_data {
            for ((net, points), path_lengths) in nets.iter().zip(net_points).zip(net_path_lengths) {
                for point in points {
                    document = document.add(
                        Circle::new()
                            .set("cx", point.x)
                            .set("cy", point.y)
                            .set("r", "3")
                            .set("fill", "lime"),
                    );

                    let mut vertex_offset = net.vertex_offset as usize;
                    for &path_len in path_lengths {
                        let vertices = &vertex_buffers[net.vertex_buffer_index as usize]
                            [vertex_offset..(vertex_offset + (path_len as usize))];
                        vertex_offset += path_len as usize;

                        document = document.add(
                            Circle::new()
                                .set("cx", vertices[0].x)
                                .set("cy", vertices[0].y)
                                .set("r", "2")
                                .set("fill", "lime"),
                        );

                        let mut path_data = String::new();
                        write!(path_data, "M {} {}", vertices[0].x, vertices[0].y).unwrap();
                        for vertex in vertices.iter().skip(1) {
                            write!(path_data, " L {} {}", vertex.x, vertex.y).unwrap();
                        }

                        document = document.add(
                            Path::new()
                                .set("d", path::Data::parse(&path_data).unwrap())
                                .set("fill", "none")
                                .set("stroke", "lime"),
                        );
                    }
                }
            }
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
        svg_out(ANCHORS, BOUNDING_BOXES, &graph, None, "graph_fast.svg");
    }

    #[test]
    fn minimal() {
        let mut graph = Graph::default();
        graph.build(ANCHORS, BOUNDING_BOXES, true);
        svg_out(ANCHORS, BOUNDING_BOXES, &graph, None, "graph_minimal.svg");
    }

    #[test]
    fn route() {
        let vertex_buffer_count = super::init();

        let mut graph = Graph::default();
        graph.build(ANCHORS, BOUNDING_BOXES, true);

        let mut net_points = Vec::new();
        let mut net_path_lengths = Vec::new();
        for port in PORTS {
            if net_points.len() <= port.net_id {
                net_points.resize_with(port.net_id + 1, || Vec::new());
                net_path_lengths.resize_with(port.net_id + 1, || Vec::new());
            }

            net_points[port.net_id].push(port.position);
            if net_points[port.net_id].len() > 1 {
                net_path_lengths[port.net_id].push(0u16);
            }
        }

        let mut nets: Vec<_> = net_points
            .iter()
            .zip(net_path_lengths.iter_mut())
            .map(|(points, path_lengths)| Net {
                points: points.as_ptr(),
                path_lengths: path_lengths.as_mut_ptr(),
                point_count: points.len() as u16,
                vertex_buffer_index: 0,
                vertex_offset: 0,
            })
            .collect();

        const VERTEX_BUFFER_CAPACITY: usize = 64 * 1024;
        let mut vertex_buffers = Vec::new();
        for _ in 0..vertex_buffer_count {
            let mut vertices = Vec::new();
            vertices.reserve_exact(VERTEX_BUFFER_CAPACITY);
            let ptr = vertices.as_mut_ptr();
            std::mem::forget(vertices);

            vertex_buffers.push(VertexBuffer {
                vertices: ptr,
                vertex_count: 0,
            });
        }

        let result = unsafe {
            RT_graph_connect_nets(
                (&graph) as *const _,
                nets.as_mut_ptr(),
                nets.len(),
                vertex_buffers.as_mut_ptr(),
                VERTEX_BUFFER_CAPACITY as u32,
            )
        };

        assert_eq!(result, Result::Success);

        let vertex_buffers: Vec<_> = vertex_buffers
            .into_iter()
            .map(|vertex_buffer| unsafe {
                Vec::from_raw_parts(
                    vertex_buffer.vertices,
                    vertex_buffer.vertex_count as usize,
                    VERTEX_BUFFER_CAPACITY,
                )
            })
            .collect();

        svg_out(
            ANCHORS,
            BOUNDING_BOXES,
            &graph,
            Some((&nets, &net_points, &net_path_lengths, &vertex_buffers)),
            "routed.svg",
        );
    }
}
