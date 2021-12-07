use std::cmp;
use std::error::Error;
use std::fs;

fn main() {
    let data = fs::read_to_string("day5.txt").unwrap();
    let _ = part1(&data, false).unwrap();
    let _ = part1(&data, true).unwrap();
}

struct LineSegment {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

fn part1(data: &str, include_diagonals: bool) -> Result<(), Box<dyn Error>> {
    let mut segments = Vec::new();
    for line in data.lines() {
        let (a, b) = line.split_once(" -> ").unwrap();
        let (x1, y1) = a.split_once(',').unwrap();
        let (x2, y2) = b.split_once(',').unwrap();
        let ls = LineSegment {
            x1: x1.parse()?,
            y1: y1.parse()?,
            x2: x2.parse()?,
            y2: y2.parse()?,
        };
        segments.push(ls);
    }

    let mut vents = [[0 as i32; 1000]; 1000];

    for ls in segments.iter() {
        let (min_x, max_x) = (cmp::min(ls.x1, ls.x2), cmp::max(ls.x1, ls.x2));
        let (min_y, max_y) = (cmp::min(ls.y1, ls.y2), cmp::max(ls.y1, ls.y2));
        if ls.x1 == ls.x2 {
            for i in min_y..max_y + 1 {
                vents[ls.x1][i] += 1;
            }
        } else if ls.y1 == ls.y2 {
            for i in min_x..max_x + 1 {
                vents[i][ls.y1] += 1;
            }
        } else if include_diagonals {
            let mut x = ls.x1;
            let mut y = ls.y1;
            loop {
                vents[x][y] += 1;

                if x == ls.x2 {
                    break;
                }

                if ls.x1 < ls.x2 {
                    x += 1;
                } else {
                    x -= 1;
                }
                if ls.y1 < ls.y2 {
                    y += 1;
                } else {
                    y -= 1;
                }

            }
        }
    }

    let mut count = 0;

    for x in 0..1000 {
        for y in 0..1000 {
            if vents[x][y] > 1 {
                count += 1;
            }
        }
    }

    println!("count: {}", count);
    Ok(())
}
