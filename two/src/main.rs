use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

trait SafetyCriteria {
    fn differ_at_most_three(&self) -> bool;
    fn differ_at_least_one(&self) -> bool;
    fn is_increasing(&self) -> bool;
    fn is_decreasing(&self) -> bool;
    fn meets_differ_conditions(&self) -> bool;
    fn can_become_safe_by_removing_one(&self) -> bool;
}

impl SafetyCriteria for [i32] {
    fn is_increasing(&self) -> bool {
        self.windows(2).all(|window| window[0] < window[1])
    }

    fn is_decreasing(&self) -> bool {
        self.windows(2).all(|window| window[0] > window[1])
    }

    fn differ_at_least_one(&self) -> bool {
        self.windows(2)
            .any(|window| window[0].abs_diff(window[1]) > 0)
    }

    fn differ_at_most_three(&self) -> bool {
        self.windows(2)
            .all(|window| window[0].abs_diff(window[1]) < 4)
    }

    fn meets_differ_conditions(&self) -> bool {
        (self.is_increasing() || self.is_decreasing())
            && self.differ_at_least_one()
            && self.differ_at_most_three()
    }

    fn can_become_safe_by_removing_one(&self) -> bool {
        (0..self.len()).any(|i| {
            let mut modified_report = self.to_vec();
            modified_report.remove(i);
            modified_report.meets_differ_conditions()
        })
    }
}

fn main() -> io::Result<()> {
    let path = Path::new("./two/sample.txt");
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let reports = reader
        .lines()
        .map_while(Result::ok)
        .map(|line| {
            line.split_whitespace()
                .filter_map(|input| input.parse::<i32>().ok())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let total_eligible = reports
        .iter()
        .filter(|report| report.meets_differ_conditions())
        .count();

    println!("Total eligible reports part one: {total_eligible}");

    let total_safe = reports
        .iter()
        .filter(|report| report.can_become_safe_by_removing_one())
        .count();

    println!("Total safe reports part two: {total_safe}");

    Ok(())
}
