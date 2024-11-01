use criterion::{criterion_group, criterion_main, Criterion};
use rand::prelude::*;
use seastar::{astar, Grid, Point};

const SEEDS: [u64; 3] = [
    2210748027404127321,
    8658502531503517188,
    4514647571430385868,
];

fn create_test_grid(w: usize, h: usize, seed: u64) -> (Grid, Point, Point) {
    let mut rng = StdRng::seed_from_u64(seed);
    let mut grid = Grid::new(w, h);

    let start = Point {
        x: rng.gen_range(0..w as isize),
        y: rng.gen_range(0..h as isize),
    };

    let end = Point {
        x: rng.gen_range(0..w as isize),
        y: rng.gen_range(0..h as isize),
    };

    let is_endpoint = |x, y| {
        (x == start.x as usize && y == start.y as usize)
            || (x == end.x as usize && y == end.y as usize)
    };

    for y in 0..h {
        for x in 0..w {
            if rng.gen_bool(0.2) && !is_endpoint(x, y) {
                *grid.get_mut(x as isize, y as isize).unwrap() = true;
            }
        }
    }

    (grid, start, end)
}

fn bench_grids_stable(c: &mut Criterion, size: usize, name: &str) {
    let mut group = c.benchmark_group(name);
    group.noise_threshold(0.05);
    group.sample_size(50);
    group.measurement_time(std::time::Duration::from_secs(3));

    let test_cases: Vec<_> = SEEDS
        .map(|seed| create_test_grid(size, size, seed))
        .into_iter()
        .collect();

    for (i, (grid, start, end)) in test_cases.iter().enumerate() {
        group.bench_with_input(
            format!("seed {}", SEEDS[i]),
            &(&grid, start, end),
            |b, input| {
                b.iter(|| {
                    let (grid, start, end) = input;
                    astar(grid, **start, **end)
                })
            },
        );
    }

    group.finish();
}

fn bench_grids_unstable(c: &mut Criterion, size: usize, name: &str) {
    let mut group = c.benchmark_group(name);

    group.measurement_time(std::time::Duration::from_secs(30));
    group.sample_size(50);
    group.noise_threshold(0.05);

    let mut rng = rand::thread_rng();

    group.bench_function("randomseeds", |b| {
        b.iter_batched(
            || {
                let seed = rng.gen::<u64>();
                create_test_grid(size, size, seed)
            },
            |(grid, start, end)| astar(&grid, start, end),
            criterion::BatchSize::LargeInput,
        )
    });

    group.finish();
}

fn bench_grids(c: &mut Criterion) {
    bench_grids_stable(c, 30, "30x30 grid/stable");
    bench_grids_stable(c, 100, "100x100 grid/stable");
    bench_grids_stable(c, 500, "500x500 grid/stable");
    bench_grids_stable(c, 1000, "1000x1000 grid/stable");

    bench_grids_unstable(c, 30, "30x30 grid/unstable");
    bench_grids_unstable(c, 100, "100x100 grid/unstable");
    bench_grids_unstable(c, 500, "500x500 grid/unstable");
    bench_grids_unstable(c, 1000, "1000x1000 grid/unstable");
}

criterion_group!(benches, bench_grids);
criterion_main!(benches);
