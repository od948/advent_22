use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    part1();
    part2();
}

fn part1() {
    let file = File::open("src/input.txt").expect("Unable to open file");
    let reader = BufReader::new(file);

    let mut count = 0;
    let mut total = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        if line.len() > 0 {
            count += line.parse::<u32>().unwrap();
        } else {
            if count > total {
                total = count;
            }
            count = 0;
        }
    }

    if count > total {
        total = count;
    }

    println!("The total is: {}", total);
}

fn part2() {
    let file = File::open("src/input.txt").expect("Unable to open file");
    let reader = BufReader::new(file);

    let mut count = 0;
    let mut vec = vec![];

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        if line.len() > 0 {
            count += line.parse::<u32>().unwrap();
        } else {
            // don't want to add 0 the vector
            if count > 0 {
                vec.push(count);
            }
            count = 0;
        }
    }

    // sort in descending order
    vec.sort_by(|a, b| b.cmp(a));
    let sum = vec[0] + vec[1] + vec[2];

    println!("The total is: {}", sum);
}
