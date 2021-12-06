use std::fs;
use std::io::{self, BufRead};

fn main() {
    part1();
    part2();
}

fn part2() {
    let file = fs::File::open("day1.txt").unwrap();
    let br = io::BufReader::new(file);

    let mut count = 0;
    let mut vec = Vec::new();

    for line in br.lines() {
        let current = line.unwrap().parse::<i32>().unwrap();
        if vec.len() > 2 {
            if vec[0] + vec[1] + vec[2] < vec[1] + vec[2] + current {
                count += 1;
            }
            vec.remove(0);
        }
        vec.push(current)
    }
    println!("{}", count)
}

fn part1() {
    let file = fs::File::open("day1.txt").unwrap();
    let br = io::BufReader::new(file);

    let mut prev: Option<i32> = None;

    let mut count = 0;

    for line in br.lines() {
        let current = line.unwrap().parse::<i32>().unwrap();
        match prev {
            Some(x) => {
                if current > x {
                    count += 1;
                }
            }
            _ => (),
        }
        prev = Some(current)
    }

    println!("{}", count)
}