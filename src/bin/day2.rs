use std::fs;
use std::io::{self, BufRead};

fn main() {
    part1();
    part2();
}

fn part2() {
    let file = fs::File::open("day2.txt").unwrap();
    let br = io::BufReader::new(file);

    let mut hpos = 0;
    let mut depth = 0;

    let mut aim = 0;

    for line in br.lines() {
        let line = line.unwrap();
        let split = line.split(" ");
        let v = split.collect::<Vec<_>>();
        let amount = v[1].parse::<i64>().unwrap();
        match v[0] {
            "forward" => {
                hpos += amount;
                depth += aim * amount;
            },
            "up" => {
                aim -= amount;
            },
            "down" => {
                aim += amount;
            },
            _ => panic!("unexpected input"),
        }
    }

    println!("part2 value: {}", depth * hpos);
}
fn part1() {
    let file = fs::File::open("day2.txt").unwrap();
    let br = io::BufReader::new(file);

    let mut hpos = 0;
    let mut vpos = 0;

    for line in br.lines() {
        let line = line.unwrap();
        let split = line.split(" ");
        let v = split.collect::<Vec<_>>();
        let amount = v[1].parse::<i64>().unwrap();
        match v[0] {
            "forward" => {
                hpos += amount;
            },
            "up" => {
                vpos -= amount;
            },
            "down" => {
                vpos += amount;
            },
            _ => panic!("unexpected input"),
        }
    }

    println!("part1 value: {}", vpos * hpos);
}