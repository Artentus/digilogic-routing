#![deny(unsafe_code)]

mod ffi;
mod graph;
mod path_finding;
mod routing;
mod segment_tree;

#[cfg(test)]
mod test;

use graph::GraphData;
use path_finding::PathFinder;
use serde::{Deserialize, Serialize};
use std::borrow::Borrow;
use std::cell::RefCell;
use std::mem::MaybeUninit;
use thread_local::ThreadLocal;

pub use graph::{Anchor, BoundingBox, BoundingBoxIndex, Direction, Directions, Node, Point};
pub use path_finding::{Path, PathFindResult, PathNode, PathNodeKind};
pub use routing::{Endpoint, NetView, RoutingError, Vertex, WireView};

type HashSet<T> = ahash::AHashSet<T>;
type HashMap<K, V> = ahash::AHashMap<K, V>;

#[derive(Default, Serialize, Deserialize)]
pub struct Graph {
    data: GraphData,
    #[serde(skip)]
    path_finder: ThreadLocal<RefCell<PathFinder>>,
}

impl Graph {
    /// Builds the graph.
    ///
    /// If the graph had previously been built, this will reset it and reuse the resources.
    #[inline]
    pub fn build(&mut self, anchors: &[Anchor], bounding_boxes: &[BoundingBox], minimal: bool) {
        self.data.build(anchors, bounding_boxes, minimal);
    }

    /// The nodes in the graph.
    #[inline]
    pub fn nodes(&self) -> &[Node] {
        self.data.nodes()
    }

    /// Finds the index of the node at the given position.
    #[inline]
    pub fn find_node(&self, position: Point) -> Option<usize> {
        self.data.find_node(position).map(|index| index as usize)
    }

    /// Finds the shortest path from `start` to `end`.
    #[inline]
    pub fn find_path(&self, start: Point, end: Point) -> PathFindResult<Path> {
        let mut path_finder = self.path_finder.get_or_default().borrow_mut();
        path_finder
            .find_path(&self.data, start, None, [end], false)
            .map(Path::clone)
    }

    /// Finds the shortest path from `start` to `ends`, optionally visiting all `ends`.
    #[inline]
    pub fn find_path_multi(
        &self,
        start: Point,
        ends: &[Point],
        visit_all: bool,
    ) -> PathFindResult<Path> {
        let mut path_finder = self.path_finder.get_or_default().borrow_mut();
        path_finder
            .find_path(&self.data, start, None, ends.iter().copied(), visit_all)
            .map(Path::clone)
    }

    #[inline]
    pub fn connect_net<WaypointList: ?Sized>(
        &self,
        endpoints: &[Endpoint<impl Clone + Borrow<WaypointList>>],
        vertices: &mut [MaybeUninit<Vertex>],
        wire_views: &mut [MaybeUninit<WireView>],
        perform_centering: bool,
    ) -> Result<NetView, RoutingError>
    where
        for<'a> &'a WaypointList: IntoIterator<Item: Borrow<Point>>,
    {
        let mut ends = Vec::new();
        let mut centering_candidates = Vec::new();
        let mut junctions = routing::JunctionMap::default();
        let mut net_view = MaybeUninit::uninit();

        routing::connect_net(
            self,
            endpoints.into_iter(),
            0,
            0,
            &mut vertices.into(),
            &mut wire_views.into(),
            &mut net_view,
            &mut ends,
            &mut centering_candidates,
            &mut junctions,
            perform_centering,
        )?;

        #[allow(unsafe_code)]
        Ok(unsafe { net_view.assume_init() })
    }
}
