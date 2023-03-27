#![doc = include_str!("../README.md")]
#![feature(binary_heap_retain)]
pub mod node;

use std::collections::{BinaryHeap, HashSet};

use self::node::Node;

pub type Grid = Vec<Vec<Option<()>>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    #[must_use]
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

/// Attempts to find the shorest path from `start` to `end` using the A*
/// algorithm. Returns `None` if no path is found.
#[must_use]
pub fn astar(grid: &Grid, start: Point, end: Point) -> Option<Vec<Point>> {
    let mut open_nodes = BinaryHeap::new();
    let mut closed_nodes = HashSet::new();

    let mut start_node = Node::new(start.x, start.y);
    let end_node = Node::new(end.x, end.y);

    // We'll set a starting h-cost on the initial node.
    start_node.h = distance(&start_node, &end_node);

    open_nodes.push(start_node);

    while let Some(current) = open_nodes.pop() {
        // We've popped a node off, so all we need to do is place it into the
        // closed set.
        closed_nodes.insert(current.clone());

        // If our current node is the end node, we've got enough to make a path
        // and return it in reverse order by backtracing our node parents.
        if current == end_node {
            return Some(retrace_path(current));
        }

        // ...otherwise, we check each valid neighbor (NESW + None).
        for mut neighbor in get_neighbors(grid, &current) {
            // If the neighbor is in the closed set, we've already checked it.
            if closed_nodes.contains(&neighbor) {
                continue;
            }

            // If any of the neighbors are in the open set, we need to check if
            // the current path is better than the one we've already found.
            if open_nodes
                .iter()
                .any(|node| node == &neighbor && node.f() < neighbor.f())
            {
                continue;
            }

            // Likewise, if the neighbor is in the closed set, we need to check
            // for the same thing.
            if closed_nodes
                .iter()
                .any(|node| node == &neighbor && node.f() < neighbor.f())
            {
                continue;
            }

            // ...otherwise, we can set the neighbor's g-cost and h-cost, and
            // add it to the open set.
            neighbor.g = current.g + distance(&current, &neighbor);
            neighbor.h = distance(&neighbor, &end_node);
            neighbor.parent = Some(Box::new(current.clone()));

            open_nodes.push(neighbor);
        }
    }

    None
}

/// Returns the path from the start to the end, including the start and end
/// position - in reverse order.
fn retrace_path(current: Node) -> Vec<Point> {
    let mut path = Vec::new();
    let mut current = Some(Box::new(current));

    while let Some(node) = current {
        path.push(Point::new(node.x, node.y));
        current = node.parent;
    }

    path.reverse();

    path
}

/// Calculates the Manhatten distance between two nodes.
fn distance(start: &Node, end: &Node) -> isize {
    let (x1, y1) = (start.x, start.y);
    let (x2, y2) = (end.x, end.y);

    (x1 - x2).abs() + (y1 - y2).abs()
}

/// Returns all of the non-colliding neighbors of the given node. Neighbors are
/// defined as colliding if they are Some, and empty if they are None.
#[must_use]
pub fn get_neighbors(grid: &Grid, node: &Node) -> Vec<Node> {
    let mut neighbors = Vec::new();
    let (x, y) = (node.x, node.y);

    let possible_positions = [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];

    for (x, y) in &possible_positions {
        if *x < 0 || *y < 0 || *x >= grid.len() as isize || *y >= grid[0].len() as isize {
            continue;
        }

        // If the grid position is None, it's a valid neighbor.
        if grid[*x as usize][*y as usize].is_none() {
            neighbors.push(Node {
                x: *x,
                y: *y,
                g: 0,
                h: 0,
                parent: None,
            });
        }
    }

    neighbors
}

mod tests {
    #[test]
    fn test_valid_path() {
        use super::*;

        #[rustfmt::skip]
        let grid = vec![
            vec![None,     None, None],
            vec![Some(()), None, Some(())],
            vec![None,     None, None],
        ];

        let start = Point::new(0, 0);
        let end = Point::new(2, 2);

        let path = astar(&grid, start, end).unwrap();

        assert_eq!(
            path,
            vec![
                start,
                Point::new(0, 1),
                Point::new(1, 1),
                Point::new(2, 1),
                end
            ]
        );
    }

    #[test]
    fn test_no_valid_path() {
        use super::*;

        #[rustfmt::skip]
        let grid = vec![
            vec![None, None,     None],
            vec![None, Some(()), Some(())],
            vec![None, Some(()), None],
        ];

        let start = Point::new(0, 0);
        let end = Point::new(2, 2);

        let path = astar(&grid, start, end);

        assert!(path.is_none());
    }

    #[test]
    fn test_collidable_neighbors() {
        use super::*;

        #[rustfmt::skip]
        let grid = vec![
            vec![None,     None, None],
            vec![Some(()), None, None],
            vec![None,     None, None],
        ];

        let node = Node::new(0, 0);
        let neighbors = get_neighbors(&grid, &node);
        assert_eq!(neighbors.len(), 1);

        let node2 = Node::new(1, 1);
        let neighbors2 = get_neighbors(&grid, &node2);
        assert_eq!(neighbors2.len(), 3);

        let node3 = Node::new(2, 1);
        let neighbors3 = get_neighbors(&grid, &node3);
        assert_eq!(neighbors3.len(), 3);
    }

    #[test]
    fn test_distance() {
        use super::*;

        let node1 = Node::new(0, 0);
        let node2 = Node::new(1, 1);
        let node3 = Node::new(2, 2);

        assert_eq!(distance(&node1, &node2), 2);
        assert_eq!(distance(&node1, &node3), 4);
        assert_eq!(distance(&node2, &node3), 2);
    }
}
