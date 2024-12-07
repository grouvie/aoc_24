use rayon::prelude::*;
use std::time::Instant;

fn main() {
    let start_time = Instant::now();

    let input_data = include_str!("../sample.txt");

    let total_simple_result: isize = input_data
        .lines()
        .par_bridge()
        .flat_map(|line| evaluate_simple_expression(parse_line(line)))
        .sum();

    let total_complex_result: isize = input_data
        .lines()
        .par_bridge()
        .flat_map(|line| evaluate_complex_expression(parse_line(line)))
        .sum();

    println!("Total simple result: {total_simple_result}");
    println!("Total complex result: {total_complex_result}");

    let duration = start_time.elapsed();
    println!(
        "It took {} seconds and {} milliseconds to evaluate the expressions",
        duration.as_secs(),
        duration.subsec_millis()
    );
}

fn parse_line(line: &str) -> Option<(isize, Vec<isize>)> {
    let mut parts = line.split(':');
    let target_value_str = parts.next()?.trim();
    let numbers_str = parts.next()?;

    let target_value = target_value_str.parse::<isize>().ok()?;
    let number_list: Vec<isize> = numbers_str
        .split_whitespace()
        .filter_map(|num_str| num_str.parse().ok())
        .collect();

    Some((target_value, number_list))
}

fn evaluate_simple_expression(parsed_line: Option<(isize, Vec<isize>)>) -> Option<isize> {
    let (target_value, number_list) = parsed_line?;
    let num_count = number_list.len();
    let total_combinations = 2_isize.pow((num_count - 1) as u32);

    (0..total_combinations)
        .into_par_iter()
        .find_map_any(|combination_index| {
            let mut current_result = number_list[0];

            for operator_index in 0..(num_count - 1) {
                let operator = if combination_index % 2 == 0 { '+' } else { '*' };
                let next_number = number_list[operator_index + 1];

                current_result = match operator {
                    '+' => current_result + next_number,
                    '*' => current_result * next_number,
                    _ => unreachable!(),
                };
            }

            if current_result == target_value {
                Some(target_value)
            } else {
                None
            }
        })
}

fn evaluate_complex_expression(parsed_line: Option<(isize, Vec<isize>)>) -> Option<isize> {
    let (target_value, number_list) = parsed_line?;
    let num_count = number_list.len();
    let total_combinations = 3_usize.pow((num_count - 1) as u32);

    (0..total_combinations)
        .into_par_iter()
        .find_map_any(|combination_index| {
            let mut current_result = number_list[0];

            for operator_index in 0..(num_count - 1) {
                let operators = ["+", "*", "||"];
                let operator =
                    operators[(combination_index / 3_usize.pow(operator_index as u32)) % 3];
                let next_number = number_list[operator_index + 1];

                current_result = match operator {
                    "+" => current_result + next_number,
                    "*" => current_result * next_number,
                    "||" => {
                        let concatenated = format!("{current_result}{next_number}");
                        concatenated.parse::<isize>().ok()?
                    }
                    _ => unreachable!(),
                };
            }

            if current_result == target_value {
                Some(target_value)
            } else {
                None
            }
        })
}
