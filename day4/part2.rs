use std::collections::HashMap;

fn main() {
    let binding = std::fs::read_to_string("day4/day4_input.txt").unwrap();
    let cards: Vec<&str> = binding.lines().collect();

    let mut ans = 0;

    let mut card_counter: HashMap<u32, u32> = HashMap::new();
    card_counter.insert(1, 1);

    cards.iter().enumerate().for_each(|(idx, line)| {
        let idx: u32 = (idx + 1).try_into().unwrap();
        let curr_num = *card_counter.get(&idx).unwrap_or_else(|| &1);
        ans += curr_num;

        let mut winners = HashMap::new();

        let temp_split = line.split(":").collect::<Vec<&str>>()[1];
        let state = temp_split.split("|").collect::<Vec<&str>>();

        let win = state[0];
        let curr = state[1];

        win.split_whitespace().for_each(|str_num| {
            *winners.entry(str_num).or_insert(0) += 1;
        });

        let mut temp_idx = idx + 1;
        curr.split_whitespace().for_each(|str_num| {
            if let Some(val) = winners.get_mut(str_num) {
                if *val > 0 {
                    *card_counter.entry(temp_idx).or_insert(1) += curr_num;
                    temp_idx += 1;
                    *val -= 1;
                }
            }
        });
        // println!("=============");
        // for (k, v) in card_counter.iter() {
        //     println!("{}: {}", k, v);
        // }
        // println!("=============");
        // println!("{}", res);
    });

    println!("ans: {}", ans);
}
