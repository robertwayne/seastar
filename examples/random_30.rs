use iridescent::{Styled, GREEN, RED};
use rand::{thread_rng, Rng};
use seastar::{astar, Point};

fn setup(w: usize, h: usize) -> (Vec<Vec<Option<()>>>, Point, Point) {
    let mut grid: Vec<Vec<Option<()>>> = Vec::with_capacity(h);

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
        let mut row = Vec::with_capacity(w);
        for x in 0..w {
            if rng.gen_bool(0.2) {
                if x == start.x as usize && y == start.y as usize {
                    row.push(None);
                } else if x == end.x as usize && y == end.y as usize {
                    row.push(None);
                } else {
                    row.push(Some(()));
                }
            } else {
                row.push(None);
            }
        }
        grid.push(row);
    }

    (grid, start, end)
}

fn draw_grid(grid: &Vec<Vec<Option<()>>>, path: Option<&Vec<Point>>) {
    if let Some(path) = path {
        for (y, row) in grid.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if path.contains(&Point {
                    x: x as isize,
                    y: y as isize,
                }) {
                    if x == path[0].x as usize && y == path[0].y as usize {
                        print!("{}", "S".foreground(RED));
                    } else if x == path[path.len() - 1].x as usize
                        && y == path[path.len() - 1].y as usize
                    {
                        print!("{}", "E".foreground(RED));
                    } else {
                        print!("{}", "o".foreground(GREEN));
                    }
                } else if cell.is_some() {
                    print!("#");
                } else {
                    print!("{}", ".".dim());
                }
            }
            println!();
        }
    } else {
        for (y, row) in grid.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if cell.is_some() {
                    print!("#");
                } else {
                    if x == 0 && y == 0 {
                        print!("{}", "S".foreground(RED));
                    } else if x == grid[0].len() - 1 && y == grid.len() - 1 {
                        print!("{}", "E".foreground(RED));
                    } else {
                        print!("{}", ".".dim());
                    }
                }
            }
            println!();
        }
    }
}

fn main() {
    let (grid, start, end) = setup(30, 30);
    let now = std::time::Instant::now();

    if let Some(path) = astar(&grid, start, end) {
        let elapsed = now.elapsed();
        draw_grid(&grid, Some(&path));

        println!("Estimated Duration: {:?}", elapsed);
    } else {
        draw_grid(&grid, None);
        println!("No path found!");
    }
}
