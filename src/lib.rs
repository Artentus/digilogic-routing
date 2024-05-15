#![deny(unsafe_code)]

pub(crate) mod ffi;
mod graph;
mod path_finding;

#[cfg(test)]
mod test;

use graph::GraphData;
use path_finding::PathFinder;
use std::cell::RefCell;
use thread_local::ThreadLocal;

pub use graph::{Anchor, BoundingBox, BoundingBoxIndex, Direction, Directions, Node, Point};
pub use path_finding::{Path, PathFindResult};

type HashSet<T> = ahash::AHashSet<T>;
type HashMap<K, V> = ahash::AHashMap<K, V>;

#[derive(Default)]
pub struct Graph {
    data: GraphData,
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
            .find_path(&self.data, start, [end])
            .map(Path::clone)
    }

    /// Finds the shortest path from `start` to one of `ends`.
    #[inline]
    pub fn find_path_multi(&self, start: Point, ends: &[Point]) -> PathFindResult<Path> {
        let mut path_finder = self.path_finder.get_or_default().borrow_mut();
        path_finder
            .find_path(&self.data, start, ends.iter().copied())
            .map(Path::clone)
    }
}
