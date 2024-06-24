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
use std::cell::RefCell;
use std::mem::MaybeUninit;
use thread_local::ThreadLocal;

pub use graph::{Anchor, BoundingBox, BoundingBoxIndex, Direction, Directions, Node, Point};
pub use path_finding::{Path, PathFindResult, PathNode, PathNodeKind};
pub use routing::{Endpoint, NetView, RoutingError, Vertex, WireView};

type HashSet<T> = ahash::AHashSet<T>;
type HashMap<K, V> = ahash::AHashMap<K, V>;

pub trait ReplayCapture {
    fn begin_path_finding(
        &mut self,
        start: graph::NodeIndex,
        ends: impl Iterator<Item = graph::NodeIndex>,
        visit_all: bool,
    );
    fn path_finding_set_g_score(&mut self, node: graph::NodeIndex, g_score: u32);
    fn path_finding_push_open_queue(&mut self, node: graph::NodeIndex, f_score: u32);
    fn path_finding_set_predecessor(
        &mut self,
        node: graph::NodeIndex,
        predecessor: graph::NodeIndex,
    );
    fn path_finding_pop_open_queue(&mut self, node: graph::NodeIndex);
    fn path_finding_clear_state(&mut self);
    fn path_finding_insert_path_node(&mut self, index: usize, node: graph::NodeIndex);
    fn path_finding_remove_path_node(&mut self, index: usize);
    fn end_path_finding(&mut self, found: bool);

    fn routing_begin_root_wire(&mut self, start: Point, end: Point);
    fn routing_begin_branch_wire(&mut self, start: Point);
    fn routing_push_vertex(&mut self, vertex: Vertex);
    fn routing_end_wire_segment(&mut self, ends_in_junction: bool);
    fn routing_end_wire(&mut self);
}

pub struct NoReplay;

impl ReplayCapture for NoReplay {
    fn begin_path_finding(
        &mut self,
        _: graph::NodeIndex,
        _: impl Iterator<Item = graph::NodeIndex>,
        _: bool,
    ) {
    }
    fn path_finding_set_g_score(&mut self, _: graph::NodeIndex, _: u32) {}
    fn path_finding_push_open_queue(&mut self, _: graph::NodeIndex, _: u32) {}
    fn path_finding_set_predecessor(&mut self, _: graph::NodeIndex, _: graph::NodeIndex) {}
    fn path_finding_pop_open_queue(&mut self, _: graph::NodeIndex) {}
    fn path_finding_clear_state(&mut self) {}
    fn path_finding_insert_path_node(&mut self, _: usize, _: graph::NodeIndex) {}
    fn path_finding_remove_path_node(&mut self, _: usize) {}
    fn end_path_finding(&mut self, _: bool) {}

    fn routing_begin_root_wire(&mut self, _: Point, _: Point) {}
    fn routing_begin_branch_wire(&mut self, _: Point) {}
    fn routing_push_vertex(&mut self, _: Vertex) {}
    fn routing_end_wire_segment(&mut self, _: bool) {}
    fn routing_end_wire(&mut self) {}
}

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
    pub fn find_path_replay(
        &self,
        start: Point,
        end: Point,
        replay: &mut impl ReplayCapture,
    ) -> PathFindResult<Path> {
        let mut path_finder = self.path_finder.get_or_default().borrow_mut();
        path_finder
            .find_path(&self.data, start, None, [end], false, replay)
            .map(Path::clone)
    }

    /// Finds the shortest path from `start` to `end`.
    #[inline]
    pub fn find_path(&self, start: Point, end: Point) -> PathFindResult<Path> {
        self.find_path_replay(start, end, &mut NoReplay)
    }

    /// Finds the shortest path from `start` to `ends`, optionally visiting all `ends`.
    #[inline]
    pub fn find_path_multi_replay(
        &self,
        start: Point,
        ends: &[Point],
        visit_all: bool,
        replay: &mut impl ReplayCapture,
    ) -> PathFindResult<Path> {
        let mut path_finder = self.path_finder.get_or_default().borrow_mut();
        path_finder
            .find_path(
                &self.data,
                start,
                None,
                ends.iter().copied(),
                visit_all,
                replay,
            )
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
        self.find_path_multi_replay(start, ends, visit_all, &mut NoReplay)
    }

    pub fn connect_net_replay(
        &self,
        endpoints: &[Endpoint],
        vertices: &mut [MaybeUninit<Vertex>],
        wire_views: &mut [MaybeUninit<WireView>],
        perform_centering: bool,
        replay: &mut impl ReplayCapture,
    ) -> Result<NetView, RoutingError> {
        let mut ends = Vec::new();
        let mut centering_candidates = Vec::new();
        let mut junctions = routing::JunctionMap::default();
        let mut net_view = MaybeUninit::uninit();

        routing::connect_net(
            self,
            endpoints.iter(),
            0,
            0,
            &mut vertices.into(),
            &mut wire_views.into(),
            &mut net_view,
            &mut ends,
            &mut centering_candidates,
            &mut junctions,
            perform_centering,
            replay,
        )?;

        #[allow(unsafe_code)]
        Ok(unsafe { net_view.assume_init() })
    }

    #[inline]
    pub fn connect_net(
        &self,
        endpoints: &[Endpoint],
        vertices: &mut [MaybeUninit<Vertex>],
        wire_views: &mut [MaybeUninit<WireView>],
        perform_centering: bool,
    ) -> Result<NetView, RoutingError> {
        self.connect_net_replay(
            endpoints,
            vertices,
            wire_views,
            perform_centering,
            &mut NoReplay,
        )
    }
}
