#![deny(unsafe_code)]

pub(crate) mod ffi;

#[cfg(test)]
mod test;

use bitflags::bitflags;
use rayon::prelude::*;
use std::cell::RefCell;
use std::cmp::Reverse;
use std::ops::{Index, IndexMut};

type HashMap<K, V> = ahash::AHashMap<K, V>;
type PriorityQueue<I, P> = priority_queue::PriorityQueue<I, P, ahash::RandomState>;
type BoundingBoxIndex = u32;
type NodeIndex = u32;

const INVALID_BOUNDING_BOX_INDEX: BoundingBoxIndex = BoundingBoxIndex::MAX;
const INVALID_NODE_INDEX: NodeIndex = NodeIndex::MAX;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Direction {
    PosX = 0,
    NegX = 1,
    PosY = 2,
    NegY = 3,
}

impl Direction {
    pub const ALL: [Self; 4] = [Self::PosX, Self::NegX, Self::PosY, Self::NegY];
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct Directions: u8 {
        const POS_X = 0x1;
        const NEG_X = 0x2;
        const POS_Y = 0x4;
        const NEG_Y = 0x8;

        const X = 0x3;
        const Y = 0xC;

        const NONE = 0x0;
        const ALL = 0xF;
    }
}

impl From<Direction> for Directions {
    #[inline]
    fn from(value: Direction) -> Self {
        Self::from_bits(1 << (value as u8)).expect("invalid direction")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct Point {
    /// The X coordinate of the point.
    pub x: i32,
    /// The Y coordinate of the point.
    pub y: i32,
}

impl Point {
    #[inline]
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    #[inline]
    pub const fn manhatten_distance_to(self, other: Self) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Anchor {
    /// The position of the anchor.
    pub position: Point,
    /// The bounding box this anchor belongs to, or `RT_INVALID_BOUNDING_BOX_INDEX` if none.
    pub bounding_box: BoundingBoxIndex,
    /// The directions in which this anchor connects.
    pub connect_directions: Directions,
}

impl Anchor {
    #[inline]
    pub const fn new(x: i32, y: i32) -> Self {
        Self {
            position: Point::new(x, y),
            bounding_box: INVALID_BOUNDING_BOX_INDEX,
            connect_directions: Directions::ALL,
        }
    }

    #[inline]
    pub const fn with_bounding_box(self, bounding_box: BoundingBoxIndex) -> Self {
        Self {
            position: self.position,
            bounding_box,
            connect_directions: self.connect_directions,
        }
    }

    #[inline]
    pub const fn with_connect_directions(self, connect_directions: Directions) -> Self {
        Self {
            position: self.position,
            bounding_box: self.bounding_box,
            connect_directions,
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct BoundingBox {
    /// The center of the bounding box.
    pub center: Point,
    /// The distance from the center to the left and right of the bounding box.
    pub half_width: u16,
    /// The distance from the center to the top and bottom of the bounding box.
    pub half_height: u16,
}

impl BoundingBox {
    #[inline]
    pub const fn min_x(self) -> i32 {
        self.center.x - (self.half_width as i32)
    }

    #[inline]
    pub const fn max_x(self) -> i32 {
        self.center.x + (self.half_width as i32)
    }

    #[inline]
    pub const fn min_y(self) -> i32 {
        self.center.y - (self.half_height as i32)
    }

    #[inline]
    pub const fn max_y(self) -> i32 {
        self.center.y + (self.half_height as i32)
    }
}

/// cbindgen:field-names=[pos_x, neg_x, pos_y, neg_y]
#[derive(Debug, Clone)]
#[repr(C)]
struct NeighborList(
    /// The neighbor in the positive X direction, or `RT_INVALID_NODE_INDEX` if none.
    NodeIndex,
    /// The neighbor in the negative X direction, or `RT_INVALID_NODE_INDEX` if none.
    NodeIndex,
    /// The neighbor in the positive Y direction, or `RT_INVALID_NODE_INDEX` if none.
    NodeIndex,
    /// The neighbor in the negative Y direction, or `RT_INVALID_NODE_INDEX` if none.
    NodeIndex,
);

impl NeighborList {
    #[inline]
    const fn new() -> Self {
        Self(
            INVALID_NODE_INDEX,
            INVALID_NODE_INDEX,
            INVALID_NODE_INDEX,
            INVALID_NODE_INDEX,
        )
    }

    fn find(&self, node: NodeIndex) -> Option<Direction> {
        for dir in Direction::ALL {
            if self[dir] == node {
                return Some(dir);
            }
        }

        None
    }
}

impl Index<Direction> for NeighborList {
    type Output = NodeIndex;

    #[inline]
    fn index(&self, index: Direction) -> &Self::Output {
        match index {
            Direction::PosX => &self.0,
            Direction::NegX => &self.1,
            Direction::PosY => &self.2,
            Direction::NegY => &self.3,
        }
    }
}

impl IndexMut<Direction> for NeighborList {
    #[inline]
    fn index_mut(&mut self, index: Direction) -> &mut Self::Output {
        match index {
            Direction::PosX => &mut self.0,
            Direction::NegX => &mut self.1,
            Direction::PosY => &mut self.2,
            Direction::NegY => &mut self.3,
        }
    }
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct Node {
    /// The position of the node.
    pub position: Point,
    /// The neighbors of the node.
    neighbors: NeighborList,
}

impl Node {
    /// Gets the index of the nodes neighbor in the specified direction.
    #[inline]
    pub fn get_neighbor(&self, dir: Direction) -> Option<usize> {
        let index = self.neighbors[dir];
        if index == INVALID_NODE_INDEX {
            None
        } else {
            Some(index as usize)
        }
    }

    /// Determines if the given node is a neighbor of this one, and if yes in which direction.
    #[inline]
    pub fn is_neighbor(&self, node: usize) -> Option<Direction> {
        let node: NodeIndex = node.try_into().ok()?;
        self.neighbors.find(node)
    }
}

#[derive(Default, Debug)]
#[repr(transparent)]
struct NodeList(Vec<Node>);

impl NodeList {
    #[inline]
    fn clear(&mut self) {
        self.0.clear();
    }

    #[inline]
    fn push(&mut self, point: Point) -> NodeIndex {
        let index: NodeIndex = self.0.len().try_into().expect("too many nodes");
        self.0.push(Node {
            position: point,
            neighbors: NeighborList::new(),
        });
        index
    }
}

impl Index<NodeIndex> for NodeList {
    type Output = Node;

    #[inline]
    fn index(&self, index: NodeIndex) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl IndexMut<NodeIndex> for NodeList {
    #[inline]
    fn index_mut(&mut self, index: NodeIndex) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}

/// Determines if two horizontally aligned points have a sightline to each other.
fn have_horizontal_sightline(
    bounding_boxes: &[BoundingBox],
    y: i32,
    x1: i32,
    x2: i32,
    ignore_box: BoundingBoxIndex,
) -> bool {
    assert!(x1 < x2);

    for (i, &bb) in bounding_boxes.iter().enumerate() {
        if (ignore_box != INVALID_BOUNDING_BOX_INDEX) && (i == (ignore_box as usize)) {
            continue;
        }

        if (y < bb.min_y()) || (y > bb.max_y()) {
            continue;
        }

        if (x2 <= bb.min_x()) || (x1 >= bb.max_x()) {
            continue;
        }

        return false;
    }

    true
}

/// Determines if two vertically aligned points have a sightline to each other.
fn have_vertical_sightline(
    bounding_boxes: &[BoundingBox],
    x: i32,
    y1: i32,
    y2: i32,
    ignore_box: BoundingBoxIndex,
) -> bool {
    assert!(y1 < y2);

    for (i, &bb) in bounding_boxes.iter().enumerate() {
        if (ignore_box != INVALID_BOUNDING_BOX_INDEX) && (i == (ignore_box as usize)) {
            continue;
        }

        if (x < bb.min_x()) || (x > bb.max_x()) {
            continue;
        }

        if (y2 <= bb.min_y()) || (y1 >= bb.max_y()) {
            continue;
        }

        return false;
    }

    true
}

/// Finds the last (inclusive) x1 coordinate in the negative direction
/// that shares a sightline with the given point (x2, y).
fn find_neg_x_cutoff(
    bounding_boxes: &[BoundingBox],
    y: i32,
    x1_coords: &[i32],
    x2: i32,
    offset: usize,
    ignore_box: BoundingBoxIndex,
) -> usize {
    if x1_coords.len() == 0 {
        return offset;
    }

    let center = x1_coords.len() / 2;
    let x1 = x1_coords[center];

    if have_horizontal_sightline(bounding_boxes, y, x1, x2, ignore_box) {
        find_neg_x_cutoff(
            bounding_boxes,
            y,
            &x1_coords[..center],
            x2,
            offset,
            ignore_box,
        )
    } else {
        find_neg_x_cutoff(
            bounding_boxes,
            y,
            &x1_coords[(center + 1)..],
            x2,
            offset + center + 1,
            ignore_box,
        )
    }
}

/// Finds the last (exclusive) x2 coordinate in the positive direction
/// that shares a sightline with the given point (x1, y).
fn find_pos_x_cutoff(
    bounding_boxes: &[BoundingBox],
    y: i32,
    x1: i32,
    x2_coords: &[i32],
    offset: usize,
    ignore_box: BoundingBoxIndex,
) -> usize {
    if x2_coords.len() == 0 {
        return offset;
    }

    let center = x2_coords.len() / 2;
    let x2 = x2_coords[center];

    if have_horizontal_sightline(bounding_boxes, y, x1, x2, ignore_box) {
        find_pos_x_cutoff(
            bounding_boxes,
            y,
            x1,
            &x2_coords[(center + 1)..],
            offset + center + 1,
            ignore_box,
        )
    } else {
        find_pos_x_cutoff(
            bounding_boxes,
            y,
            x1,
            &x2_coords[..center],
            offset,
            ignore_box,
        )
    }
}

/// Finds the last (inclusive) y1 coordinate in the negative direction
/// that shares a sightline with the given point (x, y2).
fn find_neg_y_cutoff(
    bounding_boxes: &[BoundingBox],
    x: i32,
    y1_coords: &[i32],
    y2: i32,
    offset: usize,
    ignore_box: BoundingBoxIndex,
) -> usize {
    if y1_coords.len() == 0 {
        return offset;
    }

    let center = y1_coords.len() / 2;
    let y1 = y1_coords[center];

    if have_vertical_sightline(bounding_boxes, x, y1, y2, ignore_box) {
        find_neg_y_cutoff(
            bounding_boxes,
            x,
            &y1_coords[..center],
            y2,
            offset,
            ignore_box,
        )
    } else {
        find_neg_y_cutoff(
            bounding_boxes,
            x,
            &y1_coords[(center + 1)..],
            y2,
            offset + center + 1,
            ignore_box,
        )
    }
}

/// Finds the last (exclusive) y2 coordinate in the positive direction
/// that shares a sightline with the given point (x, y1).
fn find_pos_y_cutoff(
    bounding_boxes: &[BoundingBox],
    x: i32,
    y1: i32,
    y2_coords: &[i32],
    offset: usize,
    ignore_box: BoundingBoxIndex,
) -> usize {
    if y2_coords.len() == 0 {
        return offset;
    }

    let center = y2_coords.len() / 2;
    let y2 = y2_coords[center];

    if have_vertical_sightline(bounding_boxes, x, y1, y2, ignore_box) {
        find_pos_y_cutoff(
            bounding_boxes,
            x,
            y1,
            &y2_coords[(center + 1)..],
            offset + center + 1,
            ignore_box,
        )
    } else {
        find_pos_y_cutoff(
            bounding_boxes,
            x,
            y1,
            &y2_coords[..center],
            offset,
            ignore_box,
        )
    }
}

fn include_point_horizontal(
    point: Point,
    y_index: usize,
    y_coords: &[i32],
    node_map: &HashMap<Point, NodeIndex>,
    bounding_boxes: &[BoundingBox],
    ignore_box: BoundingBoxIndex,
) -> bool {
    if node_map.contains_key(&point) {
        return true;
    }

    for y in y_coords[..y_index].iter().copied().rev() {
        if node_map.contains_key(&Point { x: point.x, y }) {
            if have_vertical_sightline(bounding_boxes, point.x, y, point.y, ignore_box) {
                return true;
            } else {
                break;
            }
        }
    }

    for y in y_coords[(y_index + 1)..].iter().copied() {
        if node_map.contains_key(&Point { x: point.x, y }) {
            if have_vertical_sightline(bounding_boxes, point.x, point.y, y, ignore_box) {
                return true;
            } else {
                break;
            }
        }
    }

    false
}

fn include_point_vertical(
    point: Point,
    x_index: usize,
    x_coords: &[i32],
    node_map: &HashMap<Point, NodeIndex>,
    bounding_boxes: &[BoundingBox],
    ignore_box: BoundingBoxIndex,
) -> bool {
    if node_map.contains_key(&point) {
        return true;
    }

    for x in x_coords[..x_index].iter().copied().rev() {
        if node_map.contains_key(&Point { x, y: point.y }) {
            if have_horizontal_sightline(bounding_boxes, point.y, x, point.x, ignore_box) {
                return true;
            } else {
                break;
            }
        }
    }

    for x in x_coords[(x_index + 1)..].iter().copied() {
        if node_map.contains_key(&Point { x, y: point.y }) {
            if have_horizontal_sightline(bounding_boxes, point.y, point.x, x, ignore_box) {
                return true;
            } else {
                break;
            }
        }
    }

    false
}

#[derive(Default)]
pub struct Graph {
    x_coords: Vec<i32>,
    y_coords: Vec<i32>,
    node_map: HashMap<Point, NodeIndex>,
    nodes: NodeList,
}

impl Graph {
    /// Builds the graph.
    ///
    /// If the graph had previously been built, this will reset it and reuse the resources.
    pub fn build(&mut self, anchors: &[Anchor], bounding_boxes: &[BoundingBox], minimal: bool) {
        use std::collections::hash_map::Entry;

        // Sort all X coordinates.
        self.x_coords.clear();
        self.x_coords.reserve(anchors.len());
        self.x_coords
            .extend(anchors.iter().map(|anchor| anchor.position.x));
        self.x_coords.par_sort_unstable();
        self.x_coords.dedup();

        // Sort all Y coordinates.
        self.y_coords.clear();
        self.y_coords.reserve(anchors.len());
        self.y_coords
            .extend(anchors.iter().map(|anchor| anchor.position.y));
        self.y_coords.par_sort_unstable();
        self.y_coords.dedup();

        self.node_map.clear();
        self.nodes.clear();

        for anchor in anchors.iter().copied() {
            // Add graph node for this anchor point.
            match self.node_map.entry(anchor.position) {
                Entry::Occupied(_) => (),
                Entry::Vacant(entry) => {
                    let index = self.nodes.push(anchor.position);
                    entry.insert(index);
                }
            }
        }

        macro_rules! node_index {
            ($point:expr) => {
                match self.node_map.entry($point) {
                    Entry::Occupied(entry) => (*entry.get(), true),
                    Entry::Vacant(entry) => {
                        let index = self.nodes.push($point);
                        entry.insert(index);
                        (index, false)
                    }
                }
            };
        }

        for anchor in anchors.iter().copied() {
            let anchor_index = self.node_map[&anchor.position];

            // Determine coordinate indices of this anchor point.
            let x_index = self
                .x_coords
                .binary_search(&anchor.position.x)
                .expect("invalid anchor point");
            let y_index = self
                .y_coords
                .binary_search(&anchor.position.y)
                .expect("invalid anchor point");

            if anchor.connect_directions.contains(Directions::NEG_X) {
                // Find how far in the negative X direction this anchor point has a sightline to.
                let neg_x_cutoff = find_neg_x_cutoff(
                    bounding_boxes,
                    anchor.position.y,
                    &self.x_coords[..x_index],
                    anchor.position.x,
                    0,
                    anchor.bounding_box,
                );

                // Create edges for all nodes between `neg_x_cutoff` and `x_index`.
                let mut prev_index = anchor_index;
                for x in self.x_coords[neg_x_cutoff..x_index].iter().copied().rev() {
                    let current_point = Point {
                        x,
                        y: anchor.position.y,
                    };

                    if !minimal
                        || include_point_horizontal(
                            current_point,
                            y_index,
                            &self.y_coords,
                            &self.node_map,
                            bounding_boxes,
                            anchor.bounding_box,
                        )
                    {
                        let (current_index, existed) = node_index!(current_point);

                        self.nodes[prev_index].neighbors[Direction::NegX] = current_index;
                        self.nodes[current_index].neighbors[Direction::PosX] = prev_index;

                        if existed
                            && (self.nodes[current_index].neighbors[Direction::NegX]
                                != INVALID_NODE_INDEX)
                        {
                            break;
                        }

                        prev_index = current_index;
                    }
                }
            }

            if anchor.connect_directions.contains(Directions::POS_X) {
                // Find how far in the positive X direction this anchor point has a sightline to.
                let pos_x_cutoff = find_pos_x_cutoff(
                    bounding_boxes,
                    anchor.position.y,
                    anchor.position.x,
                    &self.x_coords[(x_index + 1)..],
                    x_index + 1,
                    anchor.bounding_box,
                );

                // Create edges for all nodes between `x_index` and `pos_x_cutoff`.
                let mut prev_index = anchor_index;
                for x in self.x_coords[(x_index + 1)..pos_x_cutoff].iter().copied() {
                    let current_point = Point {
                        x,
                        y: anchor.position.y,
                    };

                    if !minimal
                        || include_point_horizontal(
                            current_point,
                            y_index,
                            &self.y_coords,
                            &self.node_map,
                            bounding_boxes,
                            anchor.bounding_box,
                        )
                    {
                        let (current_index, existed) = node_index!(current_point);

                        self.nodes[prev_index].neighbors[Direction::PosX] = current_index;
                        self.nodes[current_index].neighbors[Direction::NegX] = prev_index;

                        if existed
                            && (self.nodes[current_index].neighbors[Direction::PosX]
                                != INVALID_NODE_INDEX)
                        {
                            break;
                        }

                        prev_index = current_index;
                    }
                }
            }

            if anchor.connect_directions.contains(Directions::NEG_Y) {
                // Find how far in the negative Y direction this anchor point has a sightline to.
                let neg_y_cutoff = find_neg_y_cutoff(
                    bounding_boxes,
                    anchor.position.x,
                    &self.y_coords[..y_index],
                    anchor.position.y,
                    0,
                    anchor.bounding_box,
                );

                // Create edges for all nodes between `neg_y_cutoff` and `y_index`.
                let mut prev_index = anchor_index;
                for y in self.y_coords[neg_y_cutoff..y_index].iter().copied().rev() {
                    let current_point = Point {
                        x: anchor.position.x,
                        y,
                    };

                    if !minimal
                        || include_point_vertical(
                            current_point,
                            x_index,
                            &self.x_coords,
                            &self.node_map,
                            bounding_boxes,
                            anchor.bounding_box,
                        )
                    {
                        let (current_index, existed) = node_index!(current_point);

                        self.nodes[prev_index].neighbors[Direction::NegY] = current_index;
                        self.nodes[current_index].neighbors[Direction::PosY] = prev_index;

                        if existed
                            && (self.nodes[current_index].neighbors[Direction::NegY]
                                != INVALID_NODE_INDEX)
                        {
                            break;
                        }

                        prev_index = current_index;
                    }
                }
            }

            if anchor.connect_directions.contains(Directions::POS_Y) {
                // Find how far in the positive Y direction this anchor point has a sightline to.
                let pos_y_cutoff = find_pos_y_cutoff(
                    bounding_boxes,
                    anchor.position.x,
                    anchor.position.y,
                    &self.y_coords[(y_index + 1)..],
                    y_index + 1,
                    anchor.bounding_box,
                );

                // Create edges for all nodes between `y_index` and `pos_y_cutoff`.
                let mut prev_index = anchor_index;
                for y in self.y_coords[(y_index + 1)..pos_y_cutoff].iter().copied() {
                    let current_point = Point {
                        x: anchor.position.x,
                        y,
                    };

                    if !minimal
                        || include_point_vertical(
                            current_point,
                            x_index,
                            &self.x_coords,
                            &self.node_map,
                            bounding_boxes,
                            anchor.bounding_box,
                        )
                    {
                        let (current_index, existed) = node_index!(current_point);

                        self.nodes[prev_index].neighbors[Direction::PosY] = current_index;
                        self.nodes[current_index].neighbors[Direction::NegY] = prev_index;

                        if existed
                            && (self.nodes[current_index].neighbors[Direction::PosY]
                                != INVALID_NODE_INDEX)
                        {
                            break;
                        }

                        prev_index = current_index;
                    }
                }
            }
        }
    }

    /// The nodes in the graph.
    #[inline]
    pub fn nodes(&self) -> &[Node] {
        &self.nodes.0
    }

    /// Finds the index of the node at the given position.
    #[inline]
    fn find_node_impl(&self, position: Point) -> Option<NodeIndex> {
        self.node_map.get(&position).copied()
    }

    /// Finds the index of the node at the given position.
    #[inline]
    pub fn find_node(&self, position: Point) -> Option<usize> {
        self.find_node_impl(position).map(|index| index as usize)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum PathFindResult<T> {
    Found(T),
    NotFound,
    InvalidStartPoint,
    InvalidEndPoint,
}

impl<T> PathFindResult<T> {
    #[inline]
    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> PathFindResult<U> {
        match self {
            Self::Found(value) => PathFindResult::Found(f(value)),
            Self::NotFound => PathFindResult::NotFound,
            Self::InvalidStartPoint => PathFindResult::InvalidStartPoint,
            Self::InvalidEndPoint => PathFindResult::InvalidEndPoint,
        }
    }
}

#[derive(Default)]
pub struct PathFinder {
    g_score: HashMap<NodeIndex, u32>,
    predecessor: HashMap<NodeIndex, u32>,
    open_queue: PriorityQueue<NodeIndex, Reverse<u32>>,
}

impl PathFinder {
    fn build_path(&self, graph: &Graph, path: &mut Vec<Point>, start_index: NodeIndex) {
        assert_eq!(path.len(), 0);

        path.push(graph.nodes[start_index].position);

        let mut dir: Option<Direction> = None;
        let mut current_index = start_index;
        loop {
            // The final node in the path has no predecessor.
            let Some(&pred_index) = self.predecessor.get(&current_index) else {
                break;
            };

            let pred_dir = graph.nodes[current_index]
                .neighbors
                .find(pred_index)
                .expect("invalid predecessor");

            // If the predecessor is in the same direction as during the
            // last step, replace it, otherwise add it.
            if Some(pred_dir) == dir {
                *path.last_mut().unwrap() = graph.nodes[pred_index].position;
            } else {
                path.push(graph.nodes[pred_index].position);
                dir = Some(pred_dir);
            }

            current_index = pred_index;
        }
    }

    /// A* path finding.
    fn find_path_impl(
        &mut self,
        graph: &Graph,
        path: &mut Vec<Point>,
        start: Point,
        end: Point,
    ) -> PathFindResult<()> {
        let Some(start_index) = graph.find_node_impl(start) else {
            return PathFindResult::InvalidStartPoint;
        };
        let Some(end_index) = graph.find_node_impl(end) else {
            return PathFindResult::InvalidEndPoint;
        };

        self.g_score.clear();
        self.predecessor.clear();
        self.open_queue.clear();

        // Start at the end node since the final path is getting built in reverse (A* quirk).
        self.g_score.insert(end_index, 0);
        self.open_queue.push(end_index, Reverse(0));

        while let Some((current_index, _)) = self.open_queue.pop() {
            // Shortest path found, construct it and return.
            if current_index == start_index {
                self.build_path(graph, path, start_index);
                return PathFindResult::Found(());
            }

            // Find which direction is straight ahead to apply weights.
            let straight_dir = self
                .predecessor
                .get(&current_index)
                .copied()
                .map(|pred_index| {
                    graph.nodes[pred_index]
                        .neighbors
                        .find(current_index)
                        .expect("invalid predecessor")
                });

            for dir in Direction::ALL {
                let neighbor_index = graph.nodes[current_index].neighbors[dir];
                if neighbor_index == INVALID_NODE_INDEX {
                    continue;
                }

                // Calculate the new path lenght.
                let new_g_score = self.g_score[&current_index]
                    + graph.nodes[current_index]
                        .position
                        .manhatten_distance_to(graph.nodes[neighbor_index].position)
                        * if Some(dir) == straight_dir { 1 } else { 2 };

                // Check whether the new path length is shorter than the previous one.
                let update = match self.g_score.get(&neighbor_index) {
                    Some(&g_score) => new_g_score < g_score,
                    None => true,
                };

                if update {
                    // Shorter path found, update it.
                    self.g_score.insert(neighbor_index, new_g_score);
                    self.predecessor.insert(neighbor_index, current_index);

                    // Calculate the new approximate total cost.
                    let new_f_score = new_g_score
                        + graph.nodes[neighbor_index]
                            .position
                            .manhatten_distance_to(start);
                    self.open_queue.push(neighbor_index, Reverse(new_f_score));
                }
            }
        }

        PathFindResult::NotFound
    }

    /// Finds the shortest path from `start` to `end` through `graph`.
    #[inline]
    pub fn find_path(
        &mut self,
        graph: &Graph,
        start: Point,
        end: Point,
    ) -> PathFindResult<Vec<Point>> {
        let mut path = Vec::new();
        self.find_path_impl(graph, &mut path, start, end)
            .map(|_| path)
    }
}
