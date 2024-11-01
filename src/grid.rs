use crate::Point;

/// Represents a 2D grid that is backed by a 1D vector.
#[derive(Debug, Clone)]
pub struct Grid {
    width: usize,
    height: usize,
    nodes: Vec<bool>,
}

impl Grid {
    #[must_use]
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            nodes: vec![false; width * height],
        }
    }

    /// Creates a `Grid` from a 2D vector, where a `Grid` is
    /// represented as a 1D vector internally.
    #[must_use]
    pub fn from_2d(grid: Vec<Vec<bool>>) -> Self {
        let height = grid.len();
        let width = grid.first().map_or(0, Vec::len);
        let mut nodes = Vec::with_capacity(width * height);

        for row in grid {
            nodes.extend(row);
        }

        Self {
            width,
            height,
            nodes: nodes,
        }
    }

    /// Returns the width of the grid.
    #[must_use]
    pub fn width(&self) -> usize {
        self.width
    }

    /// Returns the height of the grid.
    #[must_use]
    pub fn height(&self) -> usize {
        self.height
    }

    /// Returns the index of the node at (x, y) coordinates.
    #[inline]
    #[must_use]
    pub fn index(&self, x: isize, y: isize) -> Option<usize> {
        if x >= 0 && y >= 0 && x < self.width as isize && y < self.height as isize {
            Some(y as usize * self.width + x as usize)
        } else {
            None
        }
    }

    /// Returns the value of the node at (x, y) coordinates.
    #[must_use]
    pub fn get(&self, x: isize, y: isize) -> Option<bool> {
        self.index(x, y).map(|i| self.nodes[i])
    }

    /// Returns a mutable reference to the node at (x, y) coordinates.
    #[must_use]
    pub fn get_mut(&mut self, x: isize, y: isize) -> Option<&mut bool> {
        self.index(x, y).map(|i| &mut self.nodes[i])
    }

    /// Returns whether the node at a given `Point` is walkable.
    #[must_use]
    pub fn is_walkable(&self, point: Point) -> bool {
        self.get(point.x, point.y).map_or(false, |node| !node)
    }
}
