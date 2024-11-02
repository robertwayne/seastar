# Changelog

## 2024.11.02 - v0.3.1

- Exposed a `Grid::set` method to allow modifying the state of a node at a given position.

## 2024.11.01 - v0.3.0

- Rewrote core algorithm to avoid unnecessary heap allocations, boxed references in `Node`s, and better backing data structures for node sets. In general, the performance improvements were multiple orders of magnitude faster - particularly on larger grids.
- Modified `Grid` to use a `Vec<bool>` for nodes instead of `Vec<Option<()>>`. 

## 2023.03.27 - v0.2.0

- Fixed an issue where closed nodes with lower f-costs were not being considered
  for the ideal path when checking neighbors.
- Added an optional `serde` feature for the `Point` struct.
- Nodes now use `Point`s to represent their (x, y) coordinates.
- Nodes now impl `From<Point>` and `Into<Point>`.
- Added missing doc comments.

## 2023.03.23 - v0.1.0

Initial release.
