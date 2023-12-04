use std::collections::HashMap;

fn main() {
    let binding = std::fs::read_to_string("day4/day4_input.txt").unwrap();
    let cards: Vec<&str> = binding.lines().collect();

    let mut ans = 0;

    cards.iter().for_each(|line| {
        let mut winners = HashMap::new();
        let mut res = 0;

        let temp_split = line.split(":").collect::<Vec<&str>>()[1];
        let state = temp_split.split("|").collect::<Vec<&str>>();

        let win = state[0];
        let curr = state[1];

        win.split_whitespace().for_each(|str_num| {
            *winners.entry(str_num).or_insert(0) += 1;
        });

        curr.split_whitespace().for_each(|str_num| {
            if let Some(val) = winners.get_mut(str_num) {
                if *val > 0 {
                    *val -= 1;
                    res = if res == 0 { 1 } else { res * 2 };
                }
            }
        });
        // println!("{}", res);
        ans += res;
    });

    println!("ans: {}", ans);
}
