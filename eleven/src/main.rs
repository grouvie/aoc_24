use std::{collections::HashMap, time::Instant};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Stone {
    value: Vec<u8>,
}

impl Stone {
    fn from_u64(n: u64) -> Self {
        let value = if n == 0 {
            vec![0]
        } else {
            n.to_string()
                .chars()
                .map(|char| char as u8 - b'0')
                .collect()
        };
        Stone { value }
    }

    fn from_string(string: &str) -> Self {
        Self::from_u64(string.parse::<u64>().unwrap())
    }

    fn to_u64(&self) -> u64 {
        self.value
            .iter()
            .fold(0, |acc, &digit| acc * 10 + digit as u64)
    }

    fn blink(&self) -> Vec<Stone> {
        let number = self.to_u64();
        let digits = self.value.len();

        if number == 0 {
            vec![Stone::from_u64(1)]
        } else if digits % 2 == 0 {
            let mid = digits / 2;
            let left = self.value[..mid]
                .iter()
                .fold(0, |acc, &digit| acc * 10 + digit as u64);
            let right = self.value[mid..]
                .iter()
                .fold(0, |acc, &digit| acc * 10 + digit as u64);
            vec![Stone::from_u64(left), Stone::from_u64(right)]
        } else {
            vec![Stone::from_u64(number * 2024)]
        }
    }
}

fn blink_stones(stones: HashMap<Stone, u64>, times: u64) -> HashMap<Stone, u64> {
    (0..times).fold(stones, |current, _| {
        current
            .into_iter()
            .flat_map(|(stone, count)| {
                stone
                    .blink()
                    .into_iter()
                    .map(move |new_stone| (new_stone, count))
            })
            .fold(HashMap::new(), |mut acc, (stone, count)| {
                *acc.entry(stone).or_insert(0) += count;
                acc
            })
    })
}

const TIMES: u64 = 75;

fn main() {
    let input = include_str!("../sample.txt");
    let start_time = Instant::now();
    let initial_stones =
        input
            .split_whitespace()
            .map(Stone::from_string)
            .fold(HashMap::new(), |mut acc, stone| {
                *acc.entry(stone).or_insert(0) += 1;
                acc
            });

    let final_stones = blink_stones(initial_stones, TIMES);

    let total_stones = final_stones.values().sum::<u64>();
    println!("After blinking {TIMES} times, there will be {total_stones} stones.");

    let duration = start_time.elapsed();
    println!(
        "It took {} seconds, {} milliseconds, and {} nanoseconds to calculate the antinodes",
        duration.as_secs(),
        duration.subsec_millis(),
        duration.subsec_nanos() % 1_000_000
    );
}
