use std::{
    cmp::Ordering,
    fmt::{Display, Formatter},
    hash::Hasher,
};

use crate::Point;

/// Represents a node in the `Grid` to be checked. Nodes know their position,
/// and have a `g` and `h` cost, which are used to calculate the f-cost.
/// Additionally, each node has a parent index, which is used to reconstruct the
/// path once the end node is found.
///
/// The `g` cost is the distance from the start node to the current node. The
/// `h` cost is the distance from the current node to the end node. The `f` cost
/// is the sum of the `g` and `h` costs.
#[derive(Debug, Clone, Copy)]
pub struct Node {
    pub point: Point,
    pub g: isize,
    pub h: isize,
    pub parent_index: Option<usize>,
}

impl Node {
    #[must_use]
    pub fn new(x: isize, y: isize) -> Self {
        Self {
            point: Point::new(x, y),
            g: 0,
            h: 0,
            parent_index: None,
        }
    }

    /// Calculates the `f` cost, which is the sum of the `g` and `h` costs.
    #[must_use]
    pub fn f(&self) -> isize {
        self.g + self.h
    }
}

impl std::hash::Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.point.hash(state);
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.point == other.point
    }
}

impl Eq for Node {}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.point)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    // This reverses the order of the comparison, so our
    // `BinaryHeap` will be a min-heap instead of a max-heap.
    fn cmp(&self, other: &Self) -> Ordering {
        let self_f = self.g + self.h;
        let other_f = other.g + other.h;

        match other_f.cmp(&self_f) {
            Ordering::Equal => other.h.cmp(&self.h),
            ord => ord,
        }
    }
}

impl From<Point> for Node {
    fn from(point: Point) -> Self {
        Self::new(point.x, point.y)
    }
}
