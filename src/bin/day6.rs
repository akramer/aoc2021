use std::fs;
use std::io::{self, BufRead};

fn main() {
    part1(80);
    part1(256);
}


fn part1(days: i64) {
    let file = fs::File::open("day6.txt").unwrap();
    let mut br = io::BufReader::new(file);

    let mut input = String::new();
    br.read_line( &mut input).unwrap();
    let input = input.strip_suffix('\n').unwrap();

    let mut fish_counts: [i64; 9] = [0; 9];

    for s in input.split(',') {
        println!("digit: {}", s);
        let val: usize = s.parse().unwrap();
        fish_counts[val] += 1;
    }

    for _ in 0..days {
        fish_counts = do_fish(fish_counts);
    }

    println!("{}", fish_counts.iter().map(|x| *x as i128).sum::<i128>());
}

fn do_fish(input: [i64; 9]) -> [i64; 9] {
    let mut temp: [i64; 9] = [0; 9];
    for i in 1..9 {
        temp[i-1] = input[i];
    }
    temp[8] = input[0];
    temp[6] += input[0];
    temp
}