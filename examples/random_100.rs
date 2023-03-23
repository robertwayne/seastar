use iridescent::{Styled, GREEN, RED};
use seastar::{astar, Point};

fn setup(w: usize, h: usize) -> (Vec<Vec<Option<()>>>, Point, Point) {
    let mut grid: Vec<Vec<Option<()>>> = Vec::with_capacity(h);

    for i in 0..h {
        grid.push(Vec::with_capacity(w));
        for j in 0..w {
            let rng = rand::random::<f32>();
            if i == 0 && j == 0 {
                grid[i].push(None);
                continue;
            }

            if i == h - 1 && j == w - 1 {
                grid[i].push(None);
                continue;
            }

            if rng < 0.1 {
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

fn main() {
    let (grid, start, end) = setup(100, 100);
    let now = std::time::Instant::now();

    if let Some(path) = astar(&grid, start, end) {
        let elapsed = now.elapsed();
        for x in 0..grid.len() {
            for y in 0..grid[x].len() {
                if grid[x][y].is_some() {
                    print!("#");
                } else if path.contains(&Point {
                    x: x as isize,
                    y: y as isize,
                }) {
                    if x == 0 && y == 0 {
                        print!("{}", "S".foreground(RED));
                        continue;
                    }
                    if x == grid.len() - 1 && y == grid[x].len() - 1 {
                        print!("{}", "E".foreground(RED));
                        continue;
                    }
                    print!("{}", "o".foreground(GREEN));
                } else {
                    print!("{}", ".".dim());
                }
            }
            println!();
        }

        println!("Estimated Duration: {:?}", elapsed);
    } else {
        println!("No path found!");
    }
}
