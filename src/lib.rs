#![doc = include_str!("../README.md")]
mod grid;
mod node;
mod point;

use std::collections::BinaryHeap;

pub use grid::Grid;
pub use node::Node;
pub use point::Point;

/// Attempts to find the shortest path from `start` to `end` using the A*
/// algorithm. Returns `None` if no path is found.
#[must_use]
pub fn astar(grid: &Grid, start: Point, end: Point) -> Option<Vec<Point>> {
    let width = grid.width();
    let height = grid.height();
    let capacity = width * height;

    let mut open_nodes = BinaryHeap::new();
    let mut closed_nodes = vec![false; capacity];
    let mut g_scores = vec![u16::MAX; capacity];
    let mut all_nodes = Vec::with_capacity(capacity);

    let start_node = Node {
        point: start,
        g: 0,
        h: manhattan_distance(&start, &end),
        parent_index: None,
    };

    let start_index = point_to_index(start, width);
    g_scores[start_index] = 0;
    all_nodes.push(start_node);
    open_nodes.push((0, 0));

    while let Some((_f_score, current_index)) = open_nodes.pop() {
        let current = all_nodes[current_index];

        if current.point == end {
            return Some(retrace_path(&all_nodes, current_index));
        }

        let current_point_index = point_to_index(current.point, width);
        if closed_nodes[current_point_index] {
            continue;
        }
        closed_nodes[current_point_index] = true;

        let current_g = current.g;

        for neighbor_point in get_neighbor_points(grid, current.point) {
            let neighbor_index = point_to_index(neighbor_point, width);
            if closed_nodes[neighbor_index] {
                continue;
            }

            let tentative_g = current_g + 1;

            if tentative_g >= g_scores[neighbor_index] as isize {
                continue;
            }

            let h = manhattan_distance(&neighbor_point, &end);
            let f = tentative_g + h;

            let neighbor = Node {
                point: neighbor_point,
                g: tentative_g,
                h,
                parent_index: Some(current_index),
            };

            g_scores[neighbor_index] = tentative_g as u16;
            all_nodes.push(neighbor);
            open_nodes.push((-f, all_nodes.len() - 1));
        }
    }

    None
}

/// Converts a `Point` to an index in a 1D vector.
#[inline]
fn point_to_index(point: Point, width: usize) -> usize {
    point.y as usize * width + point.x as usize
}

/// Returns the path from start to end as a list of points.
fn retrace_path(nodes: &[Node], mut current_index: usize) -> Vec<Point> {
    let start = nodes[0].point;
    let end = nodes[current_index].point;
    let initial_capacity = manhattan_distance(&start, &end) as usize;
    let mut path = Vec::with_capacity(initial_capacity);

    loop {
        let current = &nodes[current_index];
        path.push(current.point);

        if let Some(parent_index) = current.parent_index {
            current_index = parent_index;
        } else {
            break;
        }
    }

    path.reverse();
    path
}

/// Shortest distance between two points.
#[inline]
fn manhattan_distance(a: &Point, b: &Point) -> isize {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}

const NEIGHBORS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn get_neighbor_points(grid: &Grid, point: Point) -> impl Iterator<Item = Point> + '_ {
    NEIGHBORS.iter().filter_map(move |&(dx, dy)| {
        let new_x = point.x + dx;
        let new_y = point.y + dy;
        grid.get(new_x, new_y)
            .filter(|&cell| !cell)
            .map(|_| Point::new(new_x, new_y))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_path() {
        #[rustfmt::skip]
        let grid = Grid::from_2d(vec![
            vec![false, false, false],
            vec![true,  false, true ],
            vec![false, false, false],
        ]);

        let start = Point::new(0, 0);
        let end = Point::new(2, 2);

        let path = astar(&grid, start, end).unwrap();

        assert_eq!(
            path,
            vec![
                start,
                Point::new(1, 0),
                Point::new(1, 1),
                Point::new(1, 2),
                end
            ]
        );
    }

    #[test]
    fn test_no_valid_path() {
        #[rustfmt::skip]
        let grid = Grid::from_2d(vec![
            vec![false, false, false],
            vec![false, true,  true ],
            vec![false, true,  false],
        ]);

        let start = Point::new(0, 0);
        let end = Point::new(2, 2);

        let path = astar(&grid, start, end);

        assert!(path.is_none());
    }

    #[test]
    fn test_collidable_neighbors() {
        #[rustfmt::skip]
        let grid = Grid::from_2d(vec![
            vec![false,     false, false],
            vec![true,     false, false],
            vec![false,     false, false],
        ]);

        let point = Point::new(0, 0);
        let neighbors = get_neighbor_points(&grid, point);
        assert_eq!(neighbors.count(), 1);

        let point2 = Point::new(1, 1);
        let neighbors2 = get_neighbor_points(&grid, point2);
        assert_eq!(neighbors2.count(), 3);

        let point3 = Point::new(2, 1);
        let neighbors3 = get_neighbor_points(&grid, point3);
        assert_eq!(neighbors3.count(), 3);
    }

    #[test]
    fn test_distance() {
        let point1 = Point::new(0, 0);
        let point2 = Point::new(1, 1);
        let point3 = Point::new(2, 2);

        assert_eq!(manhattan_distance(&point1, &point2), 2);
        assert_eq!(manhattan_distance(&point1, &point3), 4);
        assert_eq!(manhattan_distance(&point2, &point3), 2);
    }

    #[test]
    fn test_get_shortest() {
        #[rustfmt::skip]
        let grid = Grid::from_2d(vec![
            vec![false,     false,     false,     false,     false,],
            vec![true,     true,     true,     false,     false],
            vec![false,     false,     false,     false,     false],
            vec![false,     true,     true,     true,     true],
            vec![false,     false,     false,     false,     false],
        ]);

        let start = Point::new(0, 0);
        let end = Point::new(4, 4);

        let path = astar(&grid, start, end).unwrap();

        assert_eq!(path.first(), Some(&start));
        assert_eq!(path.last(), Some(&end));
        assert_eq!(path.len(), 15);

        for window in path.windows(2) {
            let current = window[0];
            let next = window[1];
            assert_eq!(
                manhattan_distance(&current, &next),
                1,
                "Points {current:?} and {next:?} are not adjacent"
            );
        }

        for point in &path {
            assert!(
                !grid.get(point.x, point.y).unwrap(),
                "Path contains collision at {point:?}"
            );
        }
    }
}
