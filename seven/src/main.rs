use std::time::Instant;

use rayon::prelude::*;

fn main() {
    let start_time = Instant::now();

    let input_data = include_str!("../sample.txt");

    let total_simple_result: isize = input_data
        .lines()
        .par_bridge()
        .filter_map(evaluate_line_simple)
        .sum();

    let total_complex_result: isize = input_data
        .lines()
        .par_bridge()
        .filter_map(evaluate_line_complex)
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
    let number_list = numbers_str
        .split_whitespace()
        .filter_map(|num_str| num_str.parse().ok())
        .collect();

    Some((target_value, number_list))
}

fn evaluate_line_simple(line: &str) -> Option<isize> {
    let (target_value, number_list) = parse_line(line)?;
    evaluate_simple_expression(&number_list, target_value)
}

fn evaluate_line_complex(line: &str) -> Option<isize> {
    let (target_value, number_list) = parse_line(line)?;
    evaluate_complex_expression(&number_list, target_value)
}

fn evaluate_simple_expression(numbers: &[isize], target: isize) -> Option<isize> {
    let num_count = numbers.len();

    // Calculate the total number of operator combinations ('+' or '*')
    // for (n - 1) positions between n numbers: 2^(n - 1).
    let total_combinations = 2_isize.pow((num_count - 1) as u32);

    for combination_index in 0..total_combinations {
        let mut current_result = numbers[0];

        for operator_index in 0..(num_count - 1) {
            let operator = if combination_index % 2 == 0 { '+' } else { '*' };
            let next_number = numbers[operator_index + 1];

            current_result = match operator {
                '+' => current_result + next_number,
                '*' => current_result * next_number,
                _ => unreachable!(),
            };
        }

        if current_result == target {
            return Some(target);
        }
    }

    None
}

fn evaluate_complex_expression(numbers: &[isize], target: isize) -> Option<isize> {
    let num_count = numbers.len();

    // The total combinations of operators for n numbers is 3^(n-1) because there are (n-1)
    // positions between the numbers, and each position can have one of three operators:
    // addition ('+'), multiplication ('*'), or concatenation ('||').
    let total_combinations = 3_usize.pow((num_count - 1) as u32);

    for combination_index in 0..total_combinations {
        let mut current_result = numbers[0];

        for operator_index in 0..(num_count - 1) {
            let operators = ["+", "*", "||"];
            // Selects the operator for the current position in the expression:
            // 1. combination_index indicates the specific arrangement of operators being evaluated.
            // 2. operator_index is the position between numbers for the operator.
            // 3. For n numbers, there are (n - 1) positions for operators, each can be '+', '*', or '||'.
            // 4. The total number of combinations is 3^(n - 1).
            // 5. Divide combination_index by 3^(operator_index) to determine the relevant set of combinations
            //    for the current operator position.
            // 6. Use modulo 3 to get the operator index (0 => '+', 1 => '*', 2 => '||').
            //
            // Example with 3 numbers (a, b, c):
            // - Possible combinations of operators for the two positions are:
            //   0 => + +   (combination_index 0)
            //   1 => + *   (combination_index 1)
            //   2 => + ||  (combination_index 2)
            //   3 => * +   (combination_index 3)
            //   4 => * *   (combination_index 4)
            //   5 => * ||  (combination_index 5)
            //   6 => || +  (combination_index 6)
            //   7 => || *  (combination_index 7)
            //   8 => || || (combination_index 8)
            //
            // This allows cycling through operators based on the current combination of operators.
            let operator = operators[(combination_index / 3_usize.pow(operator_index as u32)) % 3];

            let next_number = numbers[operator_index + 1];

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

        if current_result == target {
            return Some(target);
        }
    }

    None
}
