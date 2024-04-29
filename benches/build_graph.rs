use criterion::{black_box, criterion_group, criterion_main, Criterion};
use digilogic_routing::*;

include!("data/graph.rs");

fn build_graph(c: &mut Criterion) {
    let mut graph = black_box(Graph::default());

    c.bench_function("build graph", |b| {
        b.iter(|| black_box(graph.build(ANCHOR_POINTS, BOUNDING_BOXES)))
    });
}

criterion_group!(benches, build_graph);
criterion_main!(benches);
