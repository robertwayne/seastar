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
<div align="center">
    <img src="assets/example.webp" alt="terminal screenshot showing off paths from start to end">
</div>

---

`seastar` is a dependency-free implementation of the __[A*
pathfinding](https://en.wikipedia.org/wiki/A*_search_algorithm)__ algorithm. It
is specifically designed to operate over unform-cost, 2D grids in cardinal
directions.

You can check out the library in action at
__[seastar.sombia.com](https://seastar.sombia.com/)__.

I can't necessarily recommend using this over the
__[pathfinding](https://github.com/samueltardieu/pathfinding)__ crate, but I
wanted a different API for my own use-case, as well as a deeper understanding of
the algorithm.

## Usage

```sh
cargo add seastar
```

```rust
use seastar::{astar, Point, Grid};

let grid = Grid::new(3, 3);

let start = Point { x: 0, y: 0 }; // top left corner
let end = Point { x: 2, y: 2 }; // bottom right corner

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

## Features

| Flag    | Default  | Description                                          | Dependencies |
|---------|----------|------------------------------------------------------|--------------|
| `serde` | Disabled | Adds `Serialize`, `Deserialize` derives for `Point`. | `serde`      |

## Benchmarks

You can run benchmarks with the command `cargo bench`.

As usual, take into account that benchmarks will vary wildly depending on the grid size,
the distance between the start and end points, and the distribution of walls. Don't take these as
perfect indicators of performance given a specific grid size.

### Stable

| Grid Size | Seed                | Time (µs) | Config          |
|-----------|---------------------|-----------|-----------------|
| 30x30     | 2210748027404127321 | 6.45      | 3s, 100 samples |
| 100x100   | 2210748027404127321 | 7.79      | 3s, 100 samples |
| 30x30     | 8658502531503517188 | 1.07      | 3s, 100 samples |
| 100x100   | 8658502531503517188 | 37.54     | 3s, 100 samples |
| 30x30     | 4514647571430385868 | 0.51      | 3s, 100 samples |
| 100x100   | 4514647571430385868 | 1.33      | 3s, 100 samples |

### Unstable

| Grid Size | Seed   | Time (µs) | Config           |
|-----------|--------|-----------|------------------|
| 30x30     | Random | 4.23      | 30s, 100 samples |
| 100x100   | Random | 27.70     | 30s, 100 samples |

_Note: Benchmarks run on Intel i9-9900K (16) @ 5.000GHz._

### Comparison Notes

`cargo bench -- --save-baseline before`

`cargo bench -- --load-baseline after`

`critcmp before after`

_Requires [critcmp](https://github.com/BurntSushi/critcmp) to be installed._

## License

Seastar is dual-licensed under either

- __[MIT License](/LICENSE-MIT)__
- __[Apache License, Version 2.0](/LICENSE-APACHE)__

at your option.
