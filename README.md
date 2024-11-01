# Seastar


<div align="right">
<a href="https://crates.io/crates/seastar">
    <img src="https://img.shields.io/crates/v/seastar?style=flat-square" alt="crates.io badge">
</a>
<a href="https://docs.rs/seastar/latest/seastar/">
    <img src="https://img.shields.io/docsrs/seastar?style=flat-square" alt="docs.rs badge">
</a>
</div>

<br>

`seastar` is a dependency-free implementation of the __[A*
pathfinding](https://en.wikipedia.org/wiki/A*_search_algorithm)__ algorithm. It
is specifically designed to operate over uniform-cost, 2D grids in cardinal
directions.

You can check out the library in action at
__[seastar.sombia.com](https://seastar.sombia.com/)__.

---

<div align="center">
    <img src="assets/example.webp" alt="terminal screenshot showing off paths from start to end">
</div>

## Usage

```sh
cargo add seastar
```

```rust
use seastar::{astar, Point, Grid};

let grid = Grid::new(3, 3);

let start = Point::new(0, 0); // top left corner
let end = Point::new(2, 2); // bottom right corner

// Assuming a path is found, `path` will be a `Vec<Point>` where each point is
// a step in the path from `start` to `end`.
if let Some(path) = astar(&grid, start, end) {
    // ...do whatever you want with the path!
}
```

## Examples

If you have cloned the `seastar` repository, you can run an example with the
command `cargo run --release --example <example_name>`.

| Example    | File                                         | Description                                                                |
|------------|----------------------------------------------|----------------------------------------------------------------------------|
| random_30  | __[random_30.rs](/examples/random_30.rs)__   | Generate a 30x30 map with random walls and a random start and end point.   |
| random_100 | __[random_100.rs](/examples/random_100.rs)__ | Generate a 100x100 map with random walls and a random start and end point. |

## Feature Flags

| Flag    | Default  | Description                                          | Dependencies |
|---------|----------|------------------------------------------------------|--------------|
| `serde` | Disabled | Adds `Serialize`, `Deserialize` derives for `Point`. | `serde`      |

## Benchmarks

You can run benchmarks with the command `cargo bench`.

As usual, take into account that benchmarks will vary wildly depending on the grid size,
the distance between the start and end points, and the distribution of walls. Don't take these as
perfect indicators of performance given a specific grid size.

### Unstable Grids (30s, 50 samples)

_Uses random seeds to give a better indicator of average execution time._

| Grid Size | Time      |
|-----------|-----------|
| 30x30     |   4.56 µs |
| 100x100   |  30.38 µs |
| 500x500   | 661.94 µs |
| 1000x1000 |   3.01 ms |

### Stable Grids (3s, 50 samples)

_Uses fixed seeds to ensure consistent results for cross-run comparisons._

<table>
<tr><th>Seed: 2210748027404127321</th><th>Seed: 8658502531503517188</th><th>Seed: 4514647571430385868</th></tr>
<tr><td>

| Grid Size | Time      |
|-----------|-----------|
| 30x30     |   7.15 µs |
| 100x100   |   8.08 µs |
| 500x500   |  64.88 µs |
| 1000x1000 |  11.71 ms |

</td><td>

| Grid Size | Time      |
|-----------|-----------|
| 30x30     |   1.18 µs |
| 100x100   |  40.23 µs |
| 500x500   |   1.60 ms |
| 1000x1000 | 932.75 µs |

</td><td>

| Grid Size | Time      |
|-----------|-----------|
| 30x30     | 480.84 ns |
| 100x100   |   1.21 µs |
| 500x500   |  16.36 µs |
| 1000x1000 | 219.31 µs |

</td></tr>
</table>

_Note: Benchmarks run on Intel i9-9900K (16) @ 5.000GHz._

### Comparison Notes

`cargo bench -- --save-baseline before`

`cargo bench -- --load-baseline after`

`critcmp before after`

_Requires [critcmp](https://github.com/BurntSushi/critcmp) to be installed._

## Motivation

The full-featured __[pathfinding](https://github.com/samueltardieu/pathfinding)__ crate exists, and should probably be your first choice if you need a pathfinding implementation. However, I wanted to implement the A* algorithm to actually understand how it works, and I desired a simpler API for my game design needs.

## License

Seastar is dual-licensed under either

- __[MIT License](/LICENSE-MIT)__
- __[Apache License, Version 2.0](/LICENSE-APACHE)__

at your option.
