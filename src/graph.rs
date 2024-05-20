use crate::segment_tree::*;
use crate::HashMap;
use bitflags::bitflags;
use rayon::prelude::*;
use std::ops::{Index, IndexMut};

pub type NodeIndex = u32;

pub const INVALID_NODE_INDEX: NodeIndex = u32::MAX;
pub const INVALID_BOUNDING_BOX_INDEX: BoundingBoxIndex = BoundingBoxIndex(u32::MAX);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct BoundingBoxIndex(u32);

impl BoundingBoxIndex {
    pub const INVALID: Self = {
        // use a block to prevent cbindgen from exporting
        INVALID_BOUNDING_BOX_INDEX
    };

    #[inline]
    pub const fn from_usize(index: usize) -> Option<Self> {
        if index < (u32::MAX as usize) {
            Some(Self(index as u32))
        } else {
            None
        }
    }

    #[inline]
    pub const fn to_usize(self) -> Option<usize> {
        if self.0 < u32::MAX {
            Some(self.0 as usize)
        } else {
            None
        }
    }

    #[inline]
    pub const fn from_u32(index: u32) -> Option<Self> {
        if index < u32::MAX {
            Some(Self(index))
        } else {
            None
        }
    }

    #[inline]
    pub const fn to_u32(self) -> Option<u32> {
        if self.0 < u32::MAX {
            Some(self.0)
        } else {
            None
        }
    }
}

impl Default for BoundingBoxIndex {
    #[inline]
    fn default() -> Self {
        Self::INVALID
    }
}

impl TryFrom<BoundingBoxIndex> for usize {
    type Error = ();

    #[inline]
    fn try_from(index: BoundingBoxIndex) -> Result<Self, Self::Error> {
        index.to_usize().ok_or(())
    }
}

impl TryFrom<usize> for BoundingBoxIndex {
    type Error = ();

    #[inline]
    fn try_from(index: usize) -> Result<Self, Self::Error> {
        BoundingBoxIndex::from_usize(index).ok_or(())
    }
}

impl TryFrom<BoundingBoxIndex> for u32 {
    type Error = ();

    #[inline]
    fn try_from(index: BoundingBoxIndex) -> Result<Self, Self::Error> {
        index.to_u32().ok_or(())
    }
}

impl TryFrom<u32> for BoundingBoxIndex {
    type Error = ();

    #[inline]
    fn try_from(index: u32) -> Result<Self, Self::Error> {
        BoundingBoxIndex::from_u32(index).ok_or(())
    }
}

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

    /// Gets the opposite direction of this one.
    #[inline]
    pub const fn opposite(self) -> Self {
        match self {
            Self::PosX => Self::NegX,
            Self::NegX => Self::PosX,
            Self::PosY => Self::NegY,
            Self::NegY => Self::PosY,
        }
    }
}

bitflags! {
    /// cbindgen:no-export
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

impl Default for Directions {
    #[inline]
    fn default() -> Self {
        Self::ALL
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
    pub const ZERO: Self = {
        // use a block to prevent cbindgen from exporting
        Self { x: 0, y: 0 }
    };

    #[inline]
    pub const fn manhatten_distance_to(self, other: Self) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

impl Default for Point {
    #[inline]
    fn default() -> Self {
        Self::ZERO
    }
}

#[derive(Debug, Default, Clone, Copy)]
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
            position: Point { x, y },
            bounding_box: BoundingBoxIndex::INVALID,
            connect_directions: Directions::ALL,
        }
    }

    #[inline]
    pub const fn with_bounding_box(self, index: BoundingBoxIndex) -> Self {
        Self {
            bounding_box: index,
            ..self
        }
    }

    #[inline]
    pub const fn with_connect_direction(self, directions: Directions) -> Self {
        Self {
            connect_directions: directions,
            ..self
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

    #[inline]
    pub const fn width(self) -> u32 {
        (self.half_width as u32) * 2
    }

    #[inline]
    pub const fn height(self) -> u32 {
        (self.half_height as u32) * 2
    }

    #[inline]
    pub const fn contains(self, point: Point) -> bool {
        (self.min_x() <= point.x)
            && (self.max_x() >= point.x)
            && (self.min_y() <= point.y)
            && (self.max_y() >= point.y)
    }

    #[inline]
    pub const fn intersects_with(self, min: Point, max: Point) -> bool {
        assert!(min.x <= max.x);
        assert!(min.y <= max.y);

        let intersects_x = ((self.min_x() >= min.x) && (self.max_x() <= max.x))
            || ((self.min_x() <= min.x) && (self.max_x() >= min.x))
            || ((self.min_x() <= max.x) && (self.max_x() >= max.x));
        let intersects_y = ((self.min_y() >= min.y) && (self.max_y() <= max.y))
            || ((self.min_y() <= min.y) && (self.max_y() >= min.y))
            || ((self.min_y() <= max.y) && (self.max_y() >= max.y));
        intersects_x && intersects_y
    }
}

/// cbindgen:field-names=[pos_x, neg_x, pos_y, neg_y]
#[derive(Debug, Clone)]
#[repr(C)]
pub(crate) struct NeighborList(
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

    #[inline]
    fn count(&self) -> usize {
        (self.0 != INVALID_NODE_INDEX) as usize
            + (self.1 != INVALID_NODE_INDEX) as usize
            + (self.2 != INVALID_NODE_INDEX) as usize
            + (self.3 != INVALID_NODE_INDEX) as usize
    }

    pub(crate) fn find(&self, node: NodeIndex) -> Option<Direction> {
        if node == INVALID_NODE_INDEX {
            return None;
        }

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
    pub(crate) neighbors: NeighborList,
    /// Whether this node was created from an anchor.
    pub is_anchor: bool,
}

impl Node {
    /// The number of neighbors this node has.
    #[inline]
    pub fn neighbor_count(&self) -> usize {
        self.neighbors.count()
    }

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
pub(crate) struct NodeList(Vec<Node>);

impl NodeList {
    #[inline]
    fn clear(&mut self) {
        self.0.clear();
    }

    #[inline]
    fn push(&mut self, point: Point, is_anchor: bool) -> NodeIndex {
        let index: NodeIndex = self.0.len().try_into().expect("too many nodes");
        assert_ne!(index, INVALID_NODE_INDEX, "too many nodes");

        self.0.push(Node {
            position: point,
            neighbors: NeighborList::new(),
            is_anchor,
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

struct HorizontalBoundingBox {
    index: BoundingBoxIndex,
    min_x: i32,
    max_x: i32,
}

struct VerticalBoundingBox {
    index: BoundingBoxIndex,
    min_y: i32,
    max_y: i32,
}

#[derive(Default)]
pub(crate) struct BoundingBoxList {
    bounding_boxes: Vec<BoundingBox>,
    horizontal_bounding_boxes: SegmentTree<i32, HorizontalBoundingBox>,
    vertical_bounding_boxes: SegmentTree<i32, VerticalBoundingBox>,
}

impl BoundingBoxList {
    fn build(&mut self, bounding_boxes: &[BoundingBox]) {
        self.bounding_boxes.clear();
        self.bounding_boxes.extend_from_slice(bounding_boxes);

        self.horizontal_bounding_boxes
            .build(bounding_boxes.iter().enumerate().map(|(i, bb)| Segment {
                start_inclusive: bb.min_y(),
                end_inclusive: bb.max_y(),
                value: HorizontalBoundingBox {
                    index: BoundingBoxIndex::from_usize(i).expect("too many bounding boxes"),
                    min_x: bb.min_x(),
                    max_x: bb.max_x(),
                },
            }));

        self.vertical_bounding_boxes
            .build(bounding_boxes.iter().enumerate().map(|(i, bb)| Segment {
                start_inclusive: bb.min_x(),
                end_inclusive: bb.max_x(),
                value: VerticalBoundingBox {
                    index: BoundingBoxIndex::from_usize(i).expect("too many bounding boxes"),
                    min_y: bb.min_y(),
                    max_y: bb.max_y(),
                },
            }));
    }

    #[inline]
    fn get(&self, index: BoundingBoxIndex) -> Option<BoundingBox> {
        usize::try_from(index)
            .ok()
            .map(|index| self.bounding_boxes[index])
    }

    /// Determines if two horizontally aligned points have a sightline to each other.
    fn points_have_horizontal_sightline(
        &self,
        y: i32,
        x1: i32,
        x2: i32,
        ignore_box: BoundingBoxIndex,
    ) -> bool {
        assert!(x1 < x2);

        let fast_result = {
            for bb in self.horizontal_bounding_boxes.iter_containing(&y) {
                if bb.index == ignore_box {
                    continue;
                }

                if (x2 < bb.min_x) || (x1 > bb.max_x) {
                    continue;
                }

                return false;
            }

            true
        };

        #[cfg(debug_assertions)]
        {
            let slow_result = {
                for (i, &bb) in self.bounding_boxes.iter().enumerate() {
                    if Some(i) == ignore_box.to_usize() {
                        continue;
                    }

                    if (y < bb.min_y()) || (y > bb.max_y()) {
                        continue;
                    }

                    if (x2 < bb.min_x()) || (x1 > bb.max_x()) {
                        continue;
                    }

                    return false;
                }

                true
            };

            assert_eq!(slow_result, fast_result);
        }

        fast_result
    }

    /// Determines if two vertically aligned points have a sightline to each other.
    fn points_have_vertical_sightline(
        &self,
        x: i32,
        y1: i32,
        y2: i32,
        ignore_box: BoundingBoxIndex,
    ) -> bool {
        assert!(y1 < y2);

        let fast_result = {
            for bb in self.vertical_bounding_boxes.iter_containing(&x) {
                if bb.index == ignore_box {
                    continue;
                }

                if (y2 < bb.min_y) || (y1 > bb.max_y) {
                    continue;
                }

                return false;
            }

            true
        };

        #[cfg(debug_assertions)]
        {
            let slow_result = {
                for (i, &bb) in self.bounding_boxes.iter().enumerate() {
                    if Some(i) == ignore_box.to_usize() {
                        continue;
                    }

                    if (x < bb.min_x()) || (x > bb.max_x()) {
                        continue;
                    }

                    if (y2 < bb.min_y()) || (y1 > bb.max_y()) {
                        continue;
                    }

                    return false;
                }

                true
            };

            assert_eq!(slow_result, fast_result);
        }

        fast_result
    }
}

/// Finds the last (inclusive) x1 coordinate in the negative direction
/// that shares a sightline with the given point (x2, y).
fn find_neg_x_cutoff(
    bounding_boxes: &BoundingBoxList,
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

    if bounding_boxes.points_have_horizontal_sightline(y, x1, x2, ignore_box) {
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
    bounding_boxes: &BoundingBoxList,
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

    if bounding_boxes.points_have_horizontal_sightline(y, x1, x2, ignore_box) {
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
    bounding_boxes: &BoundingBoxList,
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

    if bounding_boxes.points_have_vertical_sightline(x, y1, y2, ignore_box) {
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
    bounding_boxes: &BoundingBoxList,
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

    if bounding_boxes.points_have_vertical_sightline(x, y1, y2, ignore_box) {
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

/// Determines whether to include a point as node while scanning horizontally.
fn include_point_horizontal(
    point: Point,
    y_index: usize,
    y_coords: &[i32],
    node_map: &HashMap<Point, NodeIndex>,
    bounding_boxes: &BoundingBoxList,
) -> bool {
    if node_map.contains_key(&point) {
        return true;
    }

    for y in y_coords[..y_index].iter().copied().rev() {
        if node_map.contains_key(&Point { x: point.x, y }) {
            if bounding_boxes.points_have_vertical_sightline(
                point.x,
                y,
                point.y,
                BoundingBoxIndex::INVALID,
            ) {
                return true;
            } else {
                break;
            }
        }
    }

    for y in y_coords[(y_index + 1)..].iter().copied() {
        if node_map.contains_key(&Point { x: point.x, y }) {
            if bounding_boxes.points_have_vertical_sightline(
                point.x,
                point.y,
                y,
                BoundingBoxIndex::INVALID,
            ) {
                return true;
            } else {
                break;
            }
        }
    }

    false
}

/// Determines whether to include a point as node while scanning vertically.
fn include_point_vertical(
    point: Point,
    x_index: usize,
    x_coords: &[i32],
    node_map: &HashMap<Point, NodeIndex>,
    bounding_boxes: &BoundingBoxList,
) -> bool {
    if node_map.contains_key(&point) {
        return true;
    }

    for x in x_coords[..x_index].iter().copied().rev() {
        if node_map.contains_key(&Point { x, y: point.y }) {
            if bounding_boxes.points_have_horizontal_sightline(
                point.y,
                x,
                point.x,
                BoundingBoxIndex::INVALID,
            ) {
                return true;
            } else {
                break;
            }
        }
    }

    for x in x_coords[(x_index + 1)..].iter().copied() {
        if node_map.contains_key(&Point { x, y: point.y }) {
            if bounding_boxes.points_have_horizontal_sightline(
                point.y,
                point.x,
                x,
                BoundingBoxIndex::INVALID,
            ) {
                return true;
            } else {
                break;
            }
        }
    }

    false
}

fn node_index(
    node_map: &mut HashMap<Point, NodeIndex>,
    nodes: &mut NodeList,
    point: Point,
) -> (u32, bool) {
    use std::collections::hash_map::Entry;

    match node_map.entry(point) {
        Entry::Occupied(entry) => (*entry.get(), true),
        Entry::Vacant(entry) => {
            let index = nodes.push(point, false);
            entry.insert(index);
            (index, false)
        }
    }
}

#[derive(Default)]
pub(crate) struct GraphData {
    pub(crate) bounding_boxes: BoundingBoxList,
    x_coords: Vec<i32>,
    y_coords: Vec<i32>,
    node_map: HashMap<Point, NodeIndex>,
    pub(crate) nodes: NodeList,
}

impl GraphData {
    fn prescan_neg_x(&mut self, anchor: Anchor, x_index: usize, y_index: usize) {
        let bounding_box = self.bounding_boxes.get(anchor.bounding_box);
        let mut is_in_bounding_box = bounding_box.is_some();

        // Create the first node outside the bounding box.
        for x in self.x_coords[..x_index].iter().copied().rev() {
            let current_point = Point {
                x,
                y: anchor.position.y,
            };

            if is_in_bounding_box {
                if let Some(bounding_box) = bounding_box {
                    if bounding_box.contains(current_point) {
                        continue;
                    } else {
                        is_in_bounding_box = false;
                    }
                }
            }

            if !self.bounding_boxes.points_have_horizontal_sightline(
                anchor.position.y,
                x,
                anchor.position.x,
                anchor.bounding_box,
            ) {
                break;
            }

            if !include_point_horizontal(
                current_point,
                y_index,
                &self.y_coords,
                &self.node_map,
                &self.bounding_boxes,
            ) {
                continue;
            }

            let _ = node_index(&mut self.node_map, &mut self.nodes, current_point);
            break;
        }
    }

    fn prescan_pos_x(&mut self, anchor: Anchor, x_index: usize, y_index: usize) {
        let bounding_box = self.bounding_boxes.get(anchor.bounding_box);
        let mut is_in_bounding_box = bounding_box.is_some();

        // Create the first node outside the bounding box.
        for x in self.x_coords[(x_index + 1)..].iter().copied() {
            let current_point = Point {
                x,
                y: anchor.position.y,
            };

            if is_in_bounding_box {
                if let Some(bounding_box) = bounding_box {
                    if bounding_box.contains(current_point) {
                        continue;
                    } else {
                        is_in_bounding_box = false;
                    }
                }
            }

            if !self.bounding_boxes.points_have_horizontal_sightline(
                anchor.position.y,
                anchor.position.x,
                x,
                anchor.bounding_box,
            ) {
                break;
            }

            if !include_point_horizontal(
                current_point,
                y_index,
                &self.y_coords,
                &self.node_map,
                &self.bounding_boxes,
            ) {
                continue;
            }

            let _ = node_index(&mut self.node_map, &mut self.nodes, current_point);
            break;
        }
    }

    fn prescan_neg_y(&mut self, anchor: Anchor, x_index: usize, y_index: usize) {
        let bounding_box = self.bounding_boxes.get(anchor.bounding_box);
        let mut is_in_bounding_box = bounding_box.is_some();

        // Create the first node outside the bounding box.
        for y in self.y_coords[..y_index].iter().copied().rev() {
            let current_point = Point {
                x: anchor.position.x,
                y,
            };

            if is_in_bounding_box {
                if let Some(bounding_box) = bounding_box {
                    if bounding_box.contains(current_point) {
                        continue;
                    } else {
                        is_in_bounding_box = false;
                    }
                }
            }

            if !self.bounding_boxes.points_have_vertical_sightline(
                anchor.position.x,
                y,
                anchor.position.y,
                anchor.bounding_box,
            ) {
                break;
            }

            if !include_point_vertical(
                current_point,
                x_index,
                &self.x_coords,
                &self.node_map,
                &self.bounding_boxes,
            ) {
                continue;
            }

            let _ = node_index(&mut self.node_map, &mut self.nodes, current_point);
            break;
        }
    }

    fn prescan_pos_y(&mut self, anchor: Anchor, x_index: usize, y_index: usize) {
        let bounding_box = self.bounding_boxes.get(anchor.bounding_box);
        let mut is_in_bounding_box = bounding_box.is_some();

        // Create the first node outside the bounding box.
        for y in self.y_coords[(y_index + 1)..].iter().copied() {
            let current_point = Point {
                x: anchor.position.x,
                y,
            };

            if is_in_bounding_box {
                if let Some(bounding_box) = bounding_box {
                    if bounding_box.contains(current_point) {
                        continue;
                    } else {
                        is_in_bounding_box = false;
                    }
                }
            }

            if !self.bounding_boxes.points_have_vertical_sightline(
                anchor.position.x,
                anchor.position.y,
                y,
                anchor.bounding_box,
            ) {
                break;
            }

            if !include_point_vertical(
                current_point,
                x_index,
                &self.x_coords,
                &self.node_map,
                &self.bounding_boxes,
            ) {
                continue;
            }

            let _ = node_index(&mut self.node_map, &mut self.nodes, current_point);
            break;
        }
    }

    fn prescan(&mut self, anchor: Anchor) {
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
            self.prescan_neg_x(anchor, x_index, y_index);
        }

        if anchor.connect_directions.contains(Directions::POS_X) {
            self.prescan_pos_x(anchor, x_index, y_index);
        }

        if anchor.connect_directions.contains(Directions::NEG_Y) {
            self.prescan_neg_y(anchor, x_index, y_index);
        }

        if anchor.connect_directions.contains(Directions::POS_Y) {
            self.prescan_pos_y(anchor, x_index, y_index);
        }
    }

    fn scan_neg_x(
        &mut self,
        anchor: Anchor,
        anchor_index: u32,
        x_index: usize,
        y_index: usize,
        minimal: bool,
    ) {
        // Find how far in the negative X direction this anchor point has a sightline to.
        let neg_x_cutoff = find_neg_x_cutoff(
            &self.bounding_boxes,
            anchor.position.y,
            &self.x_coords[..x_index],
            anchor.position.x,
            0,
            anchor.bounding_box,
        );

        let bounding_box = self.bounding_boxes.get(anchor.bounding_box);
        let mut is_in_bounding_box = bounding_box.is_some();

        // Create edges for all nodes between `neg_x_cutoff` and `x_index`.
        let mut prev_index = anchor_index;
        for x in self.x_coords[neg_x_cutoff..x_index].iter().copied().rev() {
            let current_point = Point {
                x,
                y: anchor.position.y,
            };

            if is_in_bounding_box {
                if let Some(bounding_box) = bounding_box {
                    if bounding_box.contains(current_point) {
                        continue;
                    } else {
                        is_in_bounding_box = false;
                    }
                }
            }

            if minimal
                && !include_point_horizontal(
                    current_point,
                    y_index,
                    &self.y_coords,
                    &self.node_map,
                    &self.bounding_boxes,
                )
            {
                continue;
            }

            let (current_index, existed) =
                node_index(&mut self.node_map, &mut self.nodes, current_point);

            self.nodes[prev_index].neighbors[Direction::NegX] = current_index;
            self.nodes[current_index].neighbors[Direction::PosX] = prev_index;

            if existed
                && (self.nodes[current_index].neighbors[Direction::NegX] != INVALID_NODE_INDEX)
            {
                break;
            }

            prev_index = current_index;
        }
    }

    fn scan_pos_x(
        &mut self,
        anchor: Anchor,
        anchor_index: u32,
        x_index: usize,
        y_index: usize,
        minimal: bool,
    ) {
        // Find how far in the positive X direction this anchor point has a sightline to.
        let pos_x_cutoff = find_pos_x_cutoff(
            &self.bounding_boxes,
            anchor.position.y,
            anchor.position.x,
            &self.x_coords[(x_index + 1)..],
            x_index + 1,
            anchor.bounding_box,
        );

        let bounding_box = self.bounding_boxes.get(anchor.bounding_box);
        let mut is_in_bounding_box = bounding_box.is_some();

        // Create edges for all nodes between `x_index` and `pos_x_cutoff`.
        let mut prev_index = anchor_index;
        for x in self.x_coords[(x_index + 1)..pos_x_cutoff].iter().copied() {
            let current_point = Point {
                x,
                y: anchor.position.y,
            };

            if is_in_bounding_box {
                if let Some(bounding_box) = bounding_box {
                    if bounding_box.contains(current_point) {
                        continue;
                    } else {
                        is_in_bounding_box = false;
                    }
                }
            }

            if minimal
                && !include_point_horizontal(
                    current_point,
                    y_index,
                    &self.y_coords,
                    &self.node_map,
                    &self.bounding_boxes,
                )
            {
                continue;
            }

            let (current_index, existed) =
                node_index(&mut self.node_map, &mut self.nodes, current_point);

            self.nodes[prev_index].neighbors[Direction::PosX] = current_index;
            self.nodes[current_index].neighbors[Direction::NegX] = prev_index;

            if existed
                && (self.nodes[current_index].neighbors[Direction::PosX] != INVALID_NODE_INDEX)
            {
                break;
            }

            prev_index = current_index;
        }
    }

    fn scan_neg_y(
        &mut self,
        anchor: Anchor,
        anchor_index: u32,
        x_index: usize,
        y_index: usize,
        minimal: bool,
    ) {
        // Find how far in the negative Y direction this anchor point has a sightline to.
        let neg_y_cutoff = find_neg_y_cutoff(
            &self.bounding_boxes,
            anchor.position.x,
            &self.y_coords[..y_index],
            anchor.position.y,
            0,
            anchor.bounding_box,
        );

        let bounding_box = self.bounding_boxes.get(anchor.bounding_box);
        let mut is_in_bounding_box = bounding_box.is_some();

        // Create edges for all nodes between `neg_y_cutoff` and `y_index`.
        let mut prev_index = anchor_index;
        for y in self.y_coords[neg_y_cutoff..y_index].iter().copied().rev() {
            let current_point = Point {
                x: anchor.position.x,
                y,
            };

            if is_in_bounding_box {
                if let Some(bounding_box) = bounding_box {
                    if bounding_box.contains(current_point) {
                        continue;
                    } else {
                        is_in_bounding_box = false;
                    }
                }
            }

            if minimal
                && !include_point_vertical(
                    current_point,
                    x_index,
                    &self.x_coords,
                    &self.node_map,
                    &self.bounding_boxes,
                )
            {
                continue;
            }

            let (current_index, existed) =
                node_index(&mut self.node_map, &mut self.nodes, current_point);

            self.nodes[prev_index].neighbors[Direction::NegY] = current_index;
            self.nodes[current_index].neighbors[Direction::PosY] = prev_index;

            if existed
                && (self.nodes[current_index].neighbors[Direction::NegY] != INVALID_NODE_INDEX)
            {
                break;
            }

            prev_index = current_index;
        }
    }

    fn scan_pos_y(
        &mut self,
        anchor: Anchor,
        anchor_index: u32,
        x_index: usize,
        y_index: usize,
        minimal: bool,
    ) {
        // Find how far in the positive Y direction this anchor point has a sightline to.
        let pos_y_cutoff = find_pos_y_cutoff(
            &self.bounding_boxes,
            anchor.position.x,
            anchor.position.y,
            &self.y_coords[(y_index + 1)..],
            y_index + 1,
            anchor.bounding_box,
        );

        let bounding_box = self.bounding_boxes.get(anchor.bounding_box);
        let mut is_in_bounding_box = bounding_box.is_some();

        // Create edges for all nodes between `y_index` and `pos_y_cutoff`.
        let mut prev_index = anchor_index;
        for y in self.y_coords[(y_index + 1)..pos_y_cutoff].iter().copied() {
            let current_point = Point {
                x: anchor.position.x,
                y,
            };

            if is_in_bounding_box {
                if let Some(bounding_box) = bounding_box {
                    if bounding_box.contains(current_point) {
                        continue;
                    } else {
                        is_in_bounding_box = false;
                    }
                }
            }

            if minimal
                && !include_point_vertical(
                    current_point,
                    x_index,
                    &self.x_coords,
                    &self.node_map,
                    &self.bounding_boxes,
                )
            {
                continue;
            }

            let (current_index, existed) =
                node_index(&mut self.node_map, &mut self.nodes, current_point);

            self.nodes[prev_index].neighbors[Direction::PosY] = current_index;
            self.nodes[current_index].neighbors[Direction::NegY] = prev_index;

            if existed
                && (self.nodes[current_index].neighbors[Direction::PosY] != INVALID_NODE_INDEX)
            {
                break;
            }

            prev_index = current_index;
        }
    }

    fn scan(&mut self, anchor: Anchor, anchor_index: u32, minimal: bool) {
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
            self.scan_neg_x(anchor, anchor_index, x_index, y_index, minimal);
        }

        if anchor.connect_directions.contains(Directions::POS_X) {
            self.scan_pos_x(anchor, anchor_index, x_index, y_index, minimal);
        }

        if anchor.connect_directions.contains(Directions::NEG_Y) {
            self.scan_neg_y(anchor, anchor_index, x_index, y_index, minimal);
        }

        if anchor.connect_directions.contains(Directions::POS_Y) {
            self.scan_pos_y(anchor, anchor_index, x_index, y_index, minimal);
        }
    }

    #[cfg(debug_assertions)]
    fn assert_graph_is_valid(&self) {
        for (node_index, node) in self.nodes.0.iter().enumerate() {
            for dir in Direction::ALL {
                if let Some(neighbor_index) = node.get_neighbor(dir) {
                    let neighbor = &self.nodes.0[neighbor_index];
                    assert_eq!(neighbor.get_neighbor(dir.opposite()), Some(node_index));

                    match dir {
                        Direction::PosX | Direction::NegX => {
                            assert_eq!(node.position.y, neighbor.position.y);
                        }
                        Direction::PosY | Direction::NegY => {
                            assert_eq!(node.position.x, neighbor.position.x);
                        }
                    }

                    match dir {
                        Direction::PosX => {
                            assert!(node.position.x < neighbor.position.x);
                        }
                        Direction::NegX => {
                            assert!(node.position.x > neighbor.position.x);
                        }
                        Direction::PosY => {
                            assert!(node.position.y < neighbor.position.y);
                        }
                        Direction::NegY => {
                            assert!(node.position.y > neighbor.position.y);
                        }
                    }
                }
            }
        }
    }

    #[cfg(not(debug_assertions))]
    fn assert_graph_is_valid(&self) {}

    /// Builds the graph.
    ///
    /// If the graph had previously been built, this will reset it and reuse the resources.
    pub(crate) fn build(
        &mut self,
        anchors: &[Anchor],
        bounding_boxes: &[BoundingBox],
        minimal: bool,
    ) {
        use std::collections::hash_map::Entry;

        self.bounding_boxes.build(bounding_boxes);

        let auto_anchors = bounding_boxes.iter().flat_map(|&bb| {
            [
                Anchor::new(bb.min_x() - 1, bb.min_y() - 1),
                Anchor::new(bb.min_x() - 1, bb.max_y() + 1),
                Anchor::new(bb.max_x() + 1, bb.min_y() - 1),
                Anchor::new(bb.max_x() + 1, bb.max_y() + 1),
            ]
        });

        let anchors = anchors.iter().copied().chain(auto_anchors);

        // Sort all X coordinates.
        self.x_coords.clear();
        self.x_coords
            .extend(anchors.clone().map(|anchor| anchor.position.x));
        self.x_coords.par_sort_unstable();
        self.x_coords.dedup();

        // Sort all Y coordinates.
        self.y_coords.clear();
        self.y_coords
            .extend(anchors.clone().map(|anchor| anchor.position.y));
        self.y_coords.par_sort_unstable();
        self.y_coords.dedup();

        self.node_map.clear();
        self.nodes.clear();

        for anchor in anchors.clone() {
            // Add graph node for this anchor point.
            match self.node_map.entry(anchor.position) {
                Entry::Occupied(_) => (),
                Entry::Vacant(entry) => {
                    let index = self.nodes.push(anchor.position, true);
                    entry.insert(index);
                }
            }
        }

        if minimal {
            // Add dummy nodes for anchors inside bounding boxes.
            for anchor in anchors.clone() {
                if anchor.bounding_box == BoundingBoxIndex::INVALID {
                    continue;
                }

                self.prescan(anchor);
            }
        }

        for anchor in anchors {
            let anchor_index = self.node_map[&anchor.position];
            self.scan(anchor, anchor_index, minimal);
        }

        self.assert_graph_is_valid();
    }

    /// The nodes in the graph.
    #[inline]
    pub(crate) fn nodes(&self) -> &[Node] {
        &self.nodes.0
    }

    /// Finds the index of the node at the given position.
    #[inline]
    pub(crate) fn find_node(&self, position: Point) -> Option<NodeIndex> {
        self.node_map.get(&position).copied()
    }
}
