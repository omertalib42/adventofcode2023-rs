use regex::Regex;
use std::collections::HashMap;

fn main() {
    let binding = std::fs::read_to_string("day2/day2_input.txt").unwrap();
    let games = binding.split('\n').collect::<Vec<&str>>();

    let mut ans = 0;

    games.iter().enumerate().for_each(|(_, game)| {
        let mut curr_ans = 1;
        let mut max_fetch: HashMap<&str, u32> =
            HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);

        for round in game.split(';') {
            for color in round.split(',') {
                let color = color.trim();

                let re = Regex::new(r"(\d+)\s(\w+)").unwrap();
                let caps = re.captures(color).unwrap();
                let count = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
                let color = caps.get(2).unwrap().as_str();

                let curr = max_fetch.get_mut(color).unwrap();
                if *curr < count {
                    *curr = count;
                }
            }
        }
        max_fetch.iter().for_each(|(_, v)| {
            curr_ans *= v;
        });
        ans += curr_ans;
    });

    println!("ans: {}", ans);
}
