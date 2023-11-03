use crate::utils::read_lines;

const FILENAME: &str = "src/day6/input.txt";

pub fn solve() -> Result<(), String> {
    let lantern_fish: Vec<i32> = read_lines(FILENAME)[0]
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let mut result: i64 = part1(lantern_fish.clone());
    println!("Part 1: {}", result);

    result = part2(lantern_fish.clone());
    println!("Part 2: {}", result);

    Ok(())
}

fn part1(mut lantern_fish: Vec<i32>) -> i64 {
    for _i in 0..80 {
        let mut new_fish = 0;
        for fish in lantern_fish.iter_mut() {
            if *fish == 0 {
                *fish = 7;
                new_fish += 1;
            }

            *fish -= 1;
        }

        for _ in 0..new_fish {
            lantern_fish.push(8);
        }
    }

    lantern_fish.len() as i64
}

fn part2(lantern_fish: Vec<i32>) -> i64 {
    let mut lantern_fish_map: [i64; 10] = [
        0,
        lantern_fish.iter().filter(|x| **x == 1).count() as i64,
        lantern_fish.iter().filter(|x| **x == 2).count() as i64,
        lantern_fish.iter().filter(|x| **x == 3).count() as i64,
        lantern_fish.iter().filter(|x| **x == 4).count() as i64,
        lantern_fish.iter().filter(|x| **x == 5).count() as i64,
        0,
        0,
        0,
        0,
    ];

    for _i in 0..256 {
        if lantern_fish_map[0] > 0 {
            lantern_fish_map[7] += lantern_fish_map[0];
            lantern_fish_map[9] += lantern_fish_map[0];
            lantern_fish_map[0] = 0;
        }

        for j in 1..lantern_fish_map.len() {
            lantern_fish_map[j - 1] = lantern_fish_map[j];
        }

        lantern_fish_map[9] = 0;
    }

    lantern_fish_map.iter().sum()
}
