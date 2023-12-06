use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn make_reader() -> Result<BufReader<File>, Error> {
    let file = File::open("day6/day6_input.txt")?;
    Ok(BufReader::new(file))
}

fn main() {
    let mut reader = make_reader().unwrap();

    let mut times = String::new();
    reader.read_line(&mut times).unwrap();

    let mut distances = String::new();
    reader.read_line(&mut distances).unwrap();

    //     Time:      7  15   30
    //     Distance:  9  40  200

    let time_vec: Vec<u64> = times
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    let distance_vec: Vec<i32> = distances
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    let mut ans_vec = vec![];

    time_vec.iter().zip(distance_vec.iter()).for_each(|(t, d)| {
        let max_hold = *t;
        let mut res = 0;
        for i in 0..max_hold {
            if (max_hold - i) * i > *d as u64 {
                res += 1;
            }
        }
        ans_vec.push(res);
    });

    println!("{:?}", ans_vec.iter().fold(1, |acc, x| acc * x));
}
