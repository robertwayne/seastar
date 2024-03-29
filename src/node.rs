use std::{
    cmp::Ordering,
    fmt::{Display, Formatter},
    hash::Hasher,
};

use crate::Point;

/// Represents a node in the grid to be checked. Nodes know their position, and
/// have a `g` and `h` cost, which are used to calculate the f-cost.
/// Additionally, each node has a parent node, which is used to reconstruct the
/// path once the end node is found.
///
/// The `g` cost is the distance from the start node to the current node. The
/// `h` cost is the distance from the current node to the end node. The `f` cost
/// is the sum of the `g` and `h` costs.
///
/// You can access the `g` and `h` costs directly, but the `f` cost is
/// calculated by calling the `.f()` method.
#[derive(Debug, Clone)]
pub struct Node {
    pub point: Point,
    pub g: isize,
    pub h: isize,
    pub parent: Option<Box<Node>>,
}

impl Node {
    #[must_use]
    pub fn new(x: isize, y: isize) -> Self {
        Self {
            point: Point::new(x, y),
            g: 0,
            h: 0,
            parent: None,
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
    fn cmp(&self, other: &Self) -> Ordering {
        if self.f() == other.f() {
            other.h.cmp(&self.h)
        } else {
            other.f().cmp(&self.f())
        }
    }
}

impl From<Point> for Node {
    fn from(point: Point) -> Self {
        Self::new(point.x, point.y)
    }
}
