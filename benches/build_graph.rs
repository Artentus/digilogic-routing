use criterion::{black_box, criterion_group, criterion_main, Criterion};
use digilogic_routing::*;
use rayon::prelude::*;
use std::mem::MaybeUninit;

include!("../test_data/graph.rs");

fn build_graph(c: &mut Criterion) {
    let mut graph = black_box(Graph::default());
    let mut group = c.benchmark_group("build graph");

    group.bench_function("fast", |b| {
        b.iter(|| black_box(graph.build(ANCHORS, BOUNDING_BOXES, false)))
    });

    group.bench_function("minimal", |b| {
        b.iter(|| black_box(graph.build(ANCHORS, BOUNDING_BOXES, true)))
    });
}

#[derive(Default)]
struct Net {
    endpoints: Vec<digilogic_routing::Endpoint<Vec<Point>>>,
}

fn find_closest_endpoint(
    waypoint: Point,
    endpoints: &[digilogic_routing::Endpoint<Vec<Point>>],
) -> usize {
    let mut min_dist = 0;
    let mut min_index = 0;

    for (i, endpoint) in endpoints.iter().enumerate() {
        let dist = waypoint.manhatten_distance_to(endpoint.position);
        if dist <= min_dist {
            min_dist = dist;
            min_index = i;
        }
    }

    min_index
}

fn create_nets() -> Vec<Net> {
    let mut nets = Vec::new();

    for endpoint in ENDPOINTS {
        if endpoint.net_id >= nets.len() {
            nets.resize_with(endpoint.net_id + 1, || Net::default());
        }

        nets[endpoint.net_id]
            .endpoints
            .push(digilogic_routing::Endpoint {
                position: endpoint.position,
                waypoints: Vec::new(),
            });
    }

    for waypoint in WAYPOINTS {
        let closest_endpoint =
            find_closest_endpoint(waypoint.position, &nets[waypoint.net_id].endpoints);

        nets[waypoint.net_id].endpoints[closest_endpoint]
            .waypoints
            .push(waypoint.position);
    }

    nets
}

fn route(c: &mut Criterion) {
    let mut graph = Graph::default();
    graph.build(ANCHORS, BOUNDING_BOXES, true);
    let nets = create_nets();

    c.bench_function("route", |b| {
        b.iter(|| {
            black_box(nets.par_iter().for_each(|net| {
                let mut vertices = [MaybeUninit::uninit(); 128];
                let mut wire_views = [MaybeUninit::uninit(); 32];
                black_box(
                    graph
                        .connect_net::<[Point]>(
                            &net.endpoints,
                            &mut vertices,
                            &mut wire_views,
                            true,
                        )
                        .unwrap(),
                );
            }));
        })
    });
}

criterion_group!(benches, build_graph, route);
criterion_main!(benches);
