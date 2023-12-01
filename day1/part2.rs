use std::vec;

fn _fetch_and_add_number(line: &str) -> u32 {
    let mut nums: Vec<u32> = Vec::new();
    line.chars().for_each(|c| {
        if c.is_digit(10) {
            nums.push(c.to_digit(10).unwrap());
        }
    });

    return nums.first().unwrap() * 10 + nums.last().unwrap();
}

fn replace_and_fetch_number(line: &str) -> u32 {
    let line = line.to_string();

    let mut nums: Vec<u32> = Vec::new();
    let num_vec = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
    ];

    line.char_indices().for_each(|(i, c)| {
        if c.is_digit(10) {
            nums.push(c.to_digit(10).unwrap());
        } else {
            for (j, word) in num_vec.iter().enumerate() {
                if i + word.len() - 1 < line.len() && line[i..(i + word.len())].contains(word) {
                    nums.push((j + 1) as u32);
                    break;
                }
            }
        }
    });

    return nums.first().unwrap() * 10 + nums.last().unwrap();
}

fn main() {
    let mut ans = 0;
    std::fs::read_to_string("day1/part1_input.txt")
        .unwrap()
        .lines()
        .for_each(|line| {
            ans += replace_and_fetch_number(line);
        });

    println!("Answer: {}", ans);
}
