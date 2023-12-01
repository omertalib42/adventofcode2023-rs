use regex::Regex;
use std::collections::HashMap;

fn main() {
    let binding = std::fs::read_to_string("day2/day2_input.txt").unwrap();
    let games = binding.split('\n').collect::<Vec<&str>>();

    let max_fetch: HashMap<&str, u32> = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    let mut ans = 0;

    games.iter().enumerate().for_each(|(i, game)| {
        let mut flag = true;
        for round in game.split(';') {
            for color in round.split(',') {
                let color = color.trim();

                let re = Regex::new(r"(\d+)\s(\w+)").unwrap();
                let caps = re.captures(color).unwrap();
                let count = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
                let color = caps.get(2).unwrap().as_str();

                if count > max_fetch[color] {
                    flag = false;
                    break;
                }
            }
            if !flag {
                break;
            }
        }
        if flag {
            println!("{} is valid", i);
            ans += i + 1;
        }
    });

    println!("{}", ans);
}
