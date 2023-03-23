use criterion::{criterion_group, criterion_main, Criterion};
use seastar::{astar, Point};

fn setup(w: usize, h: usize) -> (Vec<Vec<Option<()>>>, Point, Point) {
    let mut grid: Vec<Vec<Option<()>>> = Vec::with_capacity(h);

    for i in 0..h {
        grid.push(Vec::with_capacity(w));
        for j in 0..w {
            let rnd = rand::random::<u32>() % 100;
            if i == 0 && j == 0 {
                grid[i].push(None);
                continue;
            }

            if i == h - 1 && j == w - 1 {
                grid[i].push(None);
                continue;
            }

            if rnd < 10 {
                grid[i].push(Some(()));
            } else {
                grid[i].push(None);
            }
        }
    }

    let start = Point { x: 0, y: 0 };
    let end = Point {
        x: h as isize - 1,
        y: w as isize - 1,
    };

    (grid, start, end)
}

fn very_small_grid(c: &mut Criterion) {
    let (grid, start, end) = setup(30, 30);
    let mut group = c.benchmark_group("average path");
    group.sample_size(1000);

    group.bench_function("astar 30x30", |b| b.iter(|| astar(&grid, start, end)));
}

fn small_grid(c: &mut Criterion) {
    let (grid, start, end) = setup(100, 100);
    let mut group = c.benchmark_group("average path");
    group.sample_size(100);

    group.bench_function("astar 100x100", |b| b.iter(|| astar(&grid, start, end)));
}

fn large_grid(c: &mut Criterion) {
    let (grid, start, end) = setup(256, 256);
    let mut group = c.benchmark_group("average path");
    group.sample_size(10);

    group.bench_function("astar 256x256", |b| b.iter(|| astar(&grid, start, end)));
}

criterion_group!(benches, very_small_grid, small_grid, large_grid);
criterion_main!(benches);
