use std::collections::{HashMap, HashSet};

pub struct Stars {
    stars: HashSet<(usize, usize)>,
    num: u32,
}

fn check(grid: &mut Vec<Vec<char>>, i: usize, j: usize, n: usize, m: usize) -> bool {
    if i <= 0 || i >= n - 1 || j <= 0 || j >= m - 1 {
        return false;
    }
    if grid[i][j] == '*' {
        return true;
    }
    false
}

fn iterate_and_apply(
    grid: &mut Vec<Vec<char>>,
    i: usize,
    mut j: usize,
    n: usize,
    m: usize,
) -> Stars {
    let mut star_vec: HashSet<(usize, usize)> = HashSet::new();
    let mut num = 0;
    let mut flag = false;
    while j < m {
        if grid[i][j].is_digit(10) {
            num = num * 10 + grid[i][j].to_digit(10).unwrap();
            grid[i][j] = '.';

            // A direction array mapping to all the 8 directions
            let v1 = vec![-1, -1, -1, 0, 0, 1, 1, 1];
            let v2 = vec![1, 0, -1, 1, -1, 1, 0, -1];

            for idx in 0..8 {
                let curr_x = (i as i32 + v1[idx]) as usize;
                let curr_y = (j as i32 + v2[idx]) as usize;
                if check(grid, curr_x, curr_y, n, m) {
                    flag = true;
                    // Yeah let's add the same star again and again and again
                    star_vec.insert((curr_x, curr_y));
                }
            }

            j += 1;
            continue;
        }
        break;
    }

    return Stars {
        stars: star_vec,
        num: (flag == true && num > 0) as u32 * num,
    };
}

fn main() {
    let binding = std::fs::read_to_string("day3/day3_input.txt").unwrap();
    let mut grid: Vec<Vec<char>> = binding
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let n = grid.len();
    let m = grid[0].len();

    let mut ans = 0;

    let mut star_map: HashMap<(usize, usize), Vec<u32>> = HashMap::new();

    for i in 0..n {
        for j in 0..m {
            if grid[i][j].is_digit(10) {
                let stats: Stars = iterate_and_apply(&mut grid, i, j, n, m);
                for star in stats.stars {
                    let star_vec = star_map.entry(star).or_insert(Vec::new());
                    star_vec.push(stats.num);
                }
            }
        }
    }

    for (k, v) in star_map {
        println!("k: {:?}, v: {:?}", k, v);
        if v.len() == 2 {
            ans += v[0] * v[1];
        }
    }

    println!("ans: {}", ans);
}
