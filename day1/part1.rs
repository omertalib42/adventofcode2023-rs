fn fetch_and_add_number(line: &str) -> u32 {
    let mut nums: Vec<u32> = Vec::new();
    line.chars().for_each(|c| {
        if c.is_digit(10) {
            nums.push(c.to_digit(10).unwrap());
        }
    });

    return nums.first().unwrap() * 10 + nums.last().unwrap();
}

fn main() {
    let mut ans = 0;
    std::fs::read_to_string("day1/input1.txt")
        .unwrap()
        .lines()
        .for_each(|line| {
            ans += fetch_and_add_number(line);
            // println!("{ans}\n");
        });

    println!("Answer: {}", ans);
}
