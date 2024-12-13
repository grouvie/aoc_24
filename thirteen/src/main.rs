use std::time::Instant;

struct Button {
    x: i64,
    y: i64,
}

struct Price {
    x: i64,
    y: i64,
}

fn parse_button(line: &str) -> Button {
    let binding = line.replace(',', "");
    let parts: Vec<&str> = binding.split_whitespace().collect();
    let x = parts[2][2..].parse::<i64>().unwrap();
    let y = parts[3][2..].parse::<i64>().unwrap();
    Button { x, y }
}

fn parse_price(line: &str) -> Price {
    let binding = line.replace(',', "");
    let parts: Vec<&str> = binding.split_whitespace().collect();
    let x = parts[1][2..].parse::<i64>().unwrap();
    let y = parts[2][2..].parse::<i64>().unwrap();
    Price { x, y }
}

fn calculate_min_tokens(
    button_a: &Button,
    button_b: &Button,
    mut price: Price,
    part_one: bool,
) -> Option<i64> {
    let denominator = button_b.y * button_a.x - button_b.x * button_a.y;
    if denominator == 0 {
        return None;
    }

    if !part_one {
        price.x += 10_000_000_000_000;
        price.y += 10_000_000_000_000;
    }

    let times_b = (price.y * button_a.x - price.x * button_a.y) as f64 / denominator as f64;
    let times_a = (price.x - button_b.x * times_b.round() as i64) as f64 / button_a.x as f64;

    if times_a >= 0.0 && times_b >= 0.0 && times_a.fract() == 0.0 && times_b.fract() == 0.0 {
        let total_tokens = (times_a as i64) * 3 + (times_b.round() as i64);
        return Some(total_tokens);
    }
    None
}

fn main() {
    let start_time = Instant::now();
    let input_data = include_str!("../sample.txt");
    let lines: Vec<&str> = input_data.lines().collect();
    let mut total_coins = 0;

    for i in (0..lines.len()).step_by(4) {
        if i + 2 < lines.len() {
            let button_a = parse_button(lines[i]);
            let button_b = parse_button(lines[i + 1]);
            let price = parse_price(lines[i + 2]);

            if let Some(coins) = calculate_min_tokens(&button_a, &button_b, price, false) {
                total_coins += coins;
            }
        }
    }

    println!("Total minimum tokens needed: {}", total_coins);

    let duration = start_time.elapsed();
    println!(
        "It took {} seconds, {} milliseconds, and {} nanoseconds to calculate the antinodes",
        duration.as_secs(),
        duration.subsec_millis(),
        duration.subsec_nanos() % 1_000_000
    );
}
