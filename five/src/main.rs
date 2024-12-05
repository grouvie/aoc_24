use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../sample.txt");
    let parts = input.split("\n\n").collect::<Vec<_>>();

    let mut rules = HashMap::new();
    for line in parts[0].lines() {
        let parts = line.split('|').collect::<Vec<_>>();
        let before = parts[0].trim().parse::<u32>().unwrap();
        let after = parts[1].trim().parse::<u32>().unwrap();
        rules
            .entry(before)
            .or_insert_with(HashSet::new)
            .insert(after);
    }

    let orders = parts[1]
        .lines()
        .map(|line| {
            line.split(',')
                .map(|str| str.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let (middle_numbers_sum, incorrect_orders): (u32, Vec<Vec<u32>>) =
        orders
            .into_iter()
            .fold((0, Vec::new()), |(sum, mut incorrect), order| {
                if is_correct_order(&order, &rules) {
                    let middle = get_middle(&order);
                    // println!("Correctly ordered: {:?}, Middle: {}", order, middle);
                    return (sum + middle, incorrect);
                }
                incorrect.push(order);
                (sum, incorrect)
            });

    println!(
        "Sum of middle numbers from correctly ordered updates: {}",
        middle_numbers_sum
    );

    let total_middle_sum = incorrect_orders
        .into_iter()
        .map(|order| correct_order(&order, &rules))
        .map(|corrected_order| get_middle(&corrected_order))
        .sum::<u32>();

    println!(
        "Sum of middle numbers from corrected orders: {}",
        total_middle_sum
    );
}

fn is_correct_order(order: &[u32], rules: &HashMap<u32, HashSet<u32>>) -> bool {
    let index_map = order
        .iter()
        .enumerate()
        .map(|(i, value)| (value, i))
        .collect::<HashMap<_, _>>();

    rules.iter().all(|(before, after_set)| {
        if let Some(&before_index) = index_map.get(before) {
            after_set.iter().all(|&after| {
                index_map
                    .get(&after)
                    .map_or(true, |&after_index| before_index <= after_index)
            })
        } else {
            true
        }
    })
}

fn get_middle(order: &[u32]) -> u32 {
    let len = order.len();
    order[len / 2]
}

fn correct_order(order: &[u32], rules: &HashMap<u32, HashSet<u32>>) -> Vec<u32> {
    let mut corrected_orders = order.to_vec();
    let mut sorted = true;

    while sorted {
        sorted = false;
        for i in 0..corrected_orders.len() {
            if let Some(after_set) = rules.get(&corrected_orders[i]) {
                for after in after_set {
                    if let Some(after_index) = corrected_orders.iter().position(|x| x == after) {
                        if after_index < i {
                            corrected_orders.swap(i, after_index);
                            sorted = true;
                        }
                    }
                }
            }
        }
    }

    corrected_orders
}
