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

    let time: String = times
        .split(':')
        .nth(1)
        .unwrap()
        .split_whitespace()
        .collect();

    let distance: String = distances
        .split(':')
        .nth(1)
        .unwrap()
        .split_whitespace()
        .collect();

    let time = time.parse::<u64>().unwrap();
    let distance = distance.parse::<u64>().unwrap();

    let mut ans = 0;
    for i in 0..time {
        if (time - i) * i > distance as u64 {
            ans += 1;
        }
    }

    println!("{}", ans);
}
