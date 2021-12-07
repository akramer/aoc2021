use std::error::Error;
use std::fs;

fn main() {
    let data = fs::read_to_string("day7.txt").unwrap();
    //let data = "16,1,2,0,4,2,7,1,2,14";
    let _ = part1(&data).unwrap();
}

fn fuel_consumption(positions: &Vec<i32>, position: i32) -> i32 {
    let mut count = 0;

    for p in positions.iter() {
        count += (p - position).abs();
    }
    count
}

fn fuel_consumption2(positions: &Vec<i32>, position: i32) -> i32 {
    let mut count = 0;

    for p in positions.iter() {
        let n = (p - position).abs();
        let fuel = n * (n + 1) / 2;
        count += fuel;
    }
    count
}

fn part1(data: &str) -> Result<(), Box<dyn Error>> {
    let mut positions = Vec::new();
    for line in data.lines() {
        for i in line.split(',') {
            positions.push(i.parse::<i32>()?);
        }
    }
    let max = positions.iter().max().unwrap();
    let min = positions.iter().min().unwrap();

    let mut values = Vec::new();
    let mut values2 = Vec::new();

    for i in *min..*max + 1 {
        values.push(fuel_consumption(&positions, i));
        values2.push(fuel_consumption2(&positions, i));
    }

    let lowest_fuel = values.iter().min().unwrap();
    let lowest_fuel2 = values2.iter().min().unwrap();

    println!("lowest fuel: {}", lowest_fuel);
    println!("lowest fuel 2: {}", lowest_fuel2);

    Ok(())
}
