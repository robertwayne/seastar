use iridescent::{Styled, GREEN, RED};
use rand::{thread_rng, Rng};
use seastar::{astar, Point};

fn setup(w: usize, h: usize) -> (Vec<Vec<Option<()>>>, Point, Point) {
    let mut grid: Vec<Vec<Option<()>>> = Vec::with_capacity(h);

    let mut rng = thread_rng();
    let start = Point {
        x: rng.gen_range(0..w) as isize,
        y: rng.gen_range(0..h) as isize,
    };
    let end = Point {
        x: rng.gen_range(0..w) as isize,
        y: rng.gen_range(0..h) as isize,
    };

    for i in 0..h {
        grid.push(Vec::with_capacity(w));
        for j in 0..w {
            let rng = rand::random::<f32>();
            if i == start.x as usize && j == start.y as usize {
                grid[i].push(None);
                continue;
            }

            if i == end.x as usize && j == end.y as usize {
                grid[i].push(None);
                continue;
            }

            if rng < 0.2 {
                grid[i].push(Some(()));
            } else {
                grid[i].push(None);
            }
        }
    }

    (grid, start, end)
}

fn main() {
    let (grid, start, end) = setup(100, 100);
    let now = std::time::Instant::now();

    if let Some(path) = astar(&grid, start, end) {
        let elapsed = now.elapsed();
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j].is_some() {
                    print!("#");
                } else if path.contains(&Point {
                    x: i as isize,
                    y: j as isize,
                }) {
                    if i == start.x as usize && j == start.y as usize {
                        print!("{}", "S".foreground(RED));
                        continue;
                    }
                    if i == end.x as usize && j == end.y as usize {
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
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if i == start.x as usize && j == start.y as usize {
                    print!("{}", "S".foreground(RED));
                } else if i == end.x as usize && j == end.y as usize {
                    print!("{}", "E".foreground(RED));
                } else if grid[i][j].is_some() {
                    print!("#");
                } else {
                    print!("{}", ".".dim());
                }
            }
            println!();
        }

        println!("No path found!");
    }
}
