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
    endpoints: Vec<Point>,
    waypoints: Vec<Point>,
}

fn create_nets() -> Vec<Net> {
    let mut nets = Vec::new();

    for endpoint in ENDPOINTS {
        if endpoint.net_id >= nets.len() {
            nets.resize_with(endpoint.net_id + 1, || Net::default());
        }

        nets[endpoint.net_id].endpoints.push(endpoint.position);
    }

    for waypoint in WAYPOINTS {
        nets[waypoint.net_id].waypoints.push(waypoint.position);
    }

    nets
}

fn route(c: &mut Criterion) {
    let mut graph = black_box(Graph::default());
    black_box(graph.build(ANCHORS, BOUNDING_BOXES, true));
    let nets = black_box(create_nets());

    c.bench_function("route", |b| {
        b.iter(|| {
            black_box(nets.par_iter().for_each(|net| {
                let mut vertices = [MaybeUninit::uninit(); 128];
                let mut wire_views = [MaybeUninit::uninit(); 32];
                black_box(
                    graph
                        .connect_net(
                            &net.endpoints,
                            &net.waypoints,
                            &mut vertices,
                            &mut wire_views,
                        )
                        .unwrap(),
                );
            }));
        })
    });
}

criterion_group!(benches, build_graph, route);
criterion_main!(benches);
