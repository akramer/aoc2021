use std::fs;
use std::io::{self, BufRead};

fn main() {
    part1();
    part2();
}

fn digit_count(string_list: &Vec<String>, index: usize) -> usize {
    let mut count = 0;

    for s in string_list {
        if s.as_bytes()[index] == b'1' {
            count += 1;
        }
    }

    count
}

fn part2() {
    let file = fs::File::open("day3.txt").unwrap();
    let br = io::BufReader::new(file);

    let mut all_lines = Vec::new();
    for line in br.lines() {
        all_lines.push(line.unwrap());
    }

    let mut oxygen = all_lines.clone();
    let mut index = 0;
    println!("doing oxygen");
    while oxygen.len() > 1 && index < oxygen[0].len() {
        let mut temp = Vec::new();
        let count = digit_count(&oxygen, index);
        let digit;
        digit = if count >= oxygen.len() - count {
            b'1'
        } else {
            b'0'
        };
        println!("count: {}, digit: {}, length: {}", count, digit, oxygen.len());
        for line in &oxygen {
            if line.as_bytes()[index] == digit {
                temp.push(line.clone());
            }
        }
        if temp.len() > 0 { oxygen = temp; }
        index += 1;
    }

    let mut co2 = all_lines.clone();
    let mut index = 0;
    println!("doing co2");
    while co2.len() > 1 && index < co2[0].len() {
        let mut temp = Vec::new();
        let count = digit_count(&co2, index);
        let digit;
        digit = if count >= co2.len() - count {
            b'0'
        } else {
            b'1'
        };
        println!("count: {}, digit: {}, length: {}", count, digit, co2.len());
        for line in &co2 {
            if line.as_bytes()[index] == digit {
                temp.push(line.clone());
            }
        }
        if temp.len() > 0 { co2 = temp; }
        index += 1;
    }

    println!("oxygen: {:?}, co2: {:?}", oxygen, co2);
    let oxygen = isize::from_str_radix(&oxygen[0], 2).unwrap();
    let co2 = isize::from_str_radix(&co2[0], 2).unwrap();
    println!("oxygen: {}, co2: {}", oxygen, co2);

    println!("product of oxygen and co2: {}", oxygen * co2);

}

fn part1() -> Vec<i32> {
    let file = fs::File::open("day3.txt").unwrap();
    let br = io::BufReader::new(file);

    let mut digit_sum = Vec::new();

    let mut line_count = 0;

    for line in br.lines() {
        line_count += 1;
        let line = line.unwrap();

        for (i, c) in line.chars().enumerate() {
            if digit_sum.len() < i + 1 {
                digit_sum.push(0);
            }
            if c == '1' {
                digit_sum[i] += 1;
            }
        }
    }
    println!("Total line count {}", line_count);
    let mut gamma = 0;
    let mut epsilon = 0;

    let mut most_common = Vec::new();
    for d in &digit_sum {
        gamma <<= 1;
        epsilon <<= 1;
        println!("digit_sum: {}", d);
        if *d > line_count / 2 {
            most_common.push(1);
            gamma += 1;
        } else {
            most_common.push(0);
            epsilon += 1;
        }
    }

    println!("power consumption: {}", gamma * epsilon);

    return most_common;
}
