use std::{error::Error, fs};
use winnow::{
    ascii::digit1,
    error::InputError,
    token::{literal, one_of},
    IResult, PResult, Parser,
};

fn parse_number<'str>(input: &mut &'str str) -> PResult<isize, InputError<&'str str>> {
    digit1.try_map(str::parse).parse_next(input)
}

fn parse_mul(input: &str) -> IResult<&str, (isize, isize)> {
    let (input, _) = literal("mul").parse_peek(input)?;
    let (input, _) = one_of('(').parse_peek(input)?;
    let (input, x) = parse_number.parse_peek(input)?;
    let (input, _) = one_of(',').parse_peek(input)?;
    let (input, y) = parse_number.parse_peek(input)?;
    let (input, _) = one_of(')').parse_peek(input)?;
    Ok((input, (x, y)))
}

fn parse_do(input: &str) -> IResult<&str, bool> {
    let (input, _) = literal("do()").parse_peek(input)?;
    Ok((input, true))
}

fn parse_dont(input: &str) -> IResult<&str, bool> {
    let (input, _) = literal("don't()").parse_peek(input)?;
    Ok((input, false))
}

fn parse_mul_calls(input: &str) -> (&str, Vec<(isize, isize)>) {
    let mut results = Vec::new();
    let mut remaining_input = input;

    while !remaining_input.is_empty() {
        match parse_mul(remaining_input) {
            Ok((next_input, (x, y))) => {
                results.push((x, y));
                remaining_input = next_input;
            }
            Err(_) => {
                // If parsing fails, consume one character and continue
                remaining_input = &remaining_input[1..];
            }
        }
    }

    (remaining_input, results)
}

fn parse_mul_calls_with_conditions(input: &str) -> (&str, Vec<(isize, isize)>) {
    let mut results = Vec::new();
    let mut remaining_input = input;
    let mut enabled = true;

    while !remaining_input.is_empty() {
        if let Ok((next_input, _)) = parse_do(remaining_input) {
            enabled = true;
            remaining_input = next_input;
            continue;
        } else if let Ok((next_input, _)) = parse_dont(remaining_input) {
            enabled = false;
            remaining_input = next_input;
            continue;
        }

        match parse_mul(remaining_input) {
            Ok((next_input, (x, y))) => {
                if enabled {
                    results.push((x, y));
                }
                remaining_input = next_input;
            }
            Err(_) => {
                // If parsing fails, consume one character and continue
                remaining_input = &remaining_input[1..];
            }
        }
    }

    (remaining_input, results)
}

fn process_results(results: Vec<(isize, isize)>) -> isize {
    let mut total = 0;
    for (x, y) in results {
        // println!("Found: {} and {}", x, y);
        total += x * y;
    }
    total
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./three/sample.txt")?;

    let (_, results_without_logic) = parse_mul_calls(&input);
    if results_without_logic.is_empty() {
        eprintln!("No valid 'mul' calls found in the input without do/don't logic.");
    } else {
        let total_without_logic = process_results(results_without_logic);
        println!(
            "The total result of all multiplications (without do/don't logic) is: {total_without_logic:>15}"
        );
    }

    let (_, results_with_logic) = parse_mul_calls_with_conditions(&input);
    if results_with_logic.is_empty() {
        eprintln!("No valid 'mul' calls found in the input with do/don't logic.");
    } else {
        let total_with_logic = process_results(results_with_logic);
        println!(
            "The total result of all enabled multiplications (with do/don't logic) is: {total_with_logic:>10}"
        );
    }

    Ok(())
}
