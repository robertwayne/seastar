use iridescent::{Styled, GREEN, RED};
use rand::{thread_rng, Rng};
use seastar::{astar, Grid, Point};

fn setup(w: usize, h: usize) -> (Grid, Point, Point) {
    let mut grid = Grid::new(w, h);
    let mut rng = thread_rng();

    let start = Point {
        x: rng.gen_range(0..w as isize),
        y: rng.gen_range(0..h as isize),
    };

    let end = Point {
        x: rng.gen_range(0..w as isize),
        y: rng.gen_range(0..h as isize),
    };

    for y in 0..h {
        for x in 0..w {
            if rng.gen_bool(0.2) {
                if x == start.x as usize && y == start.y as usize
                    || x == end.x as usize && y == end.y as usize
                {
                    continue; // Leave as None
                } else {
                    *grid.get_mut(x as isize, y as isize).unwrap() = true;
                }
            }
        }
    }

    (grid, start, end)
}

fn draw_grid(grid: &Grid, path: Option<&Vec<Point>>) {
    if let Some(path) = path {
        for y in 0..grid.height() {
            for x in 0..grid.width() {
                let point = Point::new(x as isize, y as isize);
                if path.contains(&point) {
                    if x == path[0].x as usize && y == path[0].y as usize {
                        print!("{}", "S".foreground(RED));
                    } else if x == path[path.len() - 1].x as usize
                        && y == path[path.len() - 1].y as usize
                    {
                        print!("{}", "E".foreground(RED));
                    } else {
                        print!("{}", "o".foreground(GREEN));
                    }
                } else if grid.get(x as isize, y as isize).unwrap() {
                    print!("#");
                } else {
                    print!("{}", ".".dim());
                }
            }
            println!();
        }
    } else {
        for y in 0..grid.height() {
            for x in 0..grid.width() {
                if grid.get(x as isize, y as isize).unwrap() {
                    print!("#");
                } else if x == 0 && y == 0 {
                    print!("{}", "S".foreground(RED));
                } else if x == grid.width() - 1 && y == grid.height() - 1 {
                    print!("{}", "E".foreground(RED));
                } else {
                    print!("{}", ".".dim());
                }
            }

            println!();
        }
    }
}

fn main() {
    let (grid, start, end) = setup(100, 100);
    let now = std::time::Instant::now();

    if let Some(path) = astar(&grid, start, end) {
        let elapsed = now.elapsed();
        draw_grid(&grid, Some(&path));
        println!(
            "Estimated Duration: {:?} | Path length: {}",
            elapsed,
            path.len()
        );
    } else {
        draw_grid(&grid, None);
        println!("No path found!");
    }
}
