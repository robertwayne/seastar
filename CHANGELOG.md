# Changelog

## 2023.03.27 - v0.2.0

- Fixed an issue where closed nodes with lower f-costs were not being considered
  for the ideal path when checking neighbors.
- Added an optional `serde` feature for the `Point` struct.
- Nodes now use `Point`s to represent their (x, y) coordinates.
- Nodes now impl `From<Point>` and `Into<Point>`.
- Added missing more doc comments.

## 2023.03.23 - v0.1.0

Initial release.
