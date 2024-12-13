use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

#[derive(Debug)]
struct FarmPlot {
    regions: Vec<HashSet<(usize, usize)>>,
}

impl FarmPlot {
    fn new() -> Self {
        Self {
            regions: Vec::new(),
        }
    }

    fn add_position(&mut self, row_index: usize, column_index: usize) {
        let mut top_region_index = None;
        let mut left_region_index = None;

        for (region_index, region) in self.regions.iter().enumerate() {
            if top_region_index.is_none()
                && row_index > 0
                && region.contains(&(row_index - 1, column_index))
            {
                top_region_index = Some(region_index);
            }
            if left_region_index.is_none()
                && column_index > 0
                && region.contains(&(row_index, column_index - 1))
            {
                left_region_index = Some(region_index);
            }

            if top_region_index.is_some() && left_region_index.is_some() {
                break;
            }
        }

        match (top_region_index, left_region_index) {
            (Some(i), Some(j)) if i == j => {
                self.regions[i].insert((row_index, column_index));
            }
            (Some(i), Some(j)) => {
                let (smaller_index, larger_index) = if j < i { (j, i) } else { (i, j) };
                self.regions[smaller_index].insert((row_index, column_index));
                let merged_region = self.regions.remove(larger_index);
                self.regions[smaller_index].extend(merged_region);
            }
            (Some(i), None) | (None, Some(i)) => {
                self.regions[i].insert((row_index, column_index));
            }
            (None, None) => {
                self.regions
                    .push(HashSet::from([(row_index, column_index)]));
            }
        }
    }

    fn calculate_corner_costs(&self) -> usize {
        let mut total_cost = 0;

        for region in &self.regions {
            let area = region.len();
            let corner_count = Self::get_corner_count(region);
            total_cost += area * corner_count;
        }

        total_cost
    }

    fn calculate_perimeter_costs(&self) -> usize {
        let mut total_cost = 0;

        for region in &self.regions {
            let area = region.len();
            let perimeter = Self::get_perimeter(region);
            total_cost += area * perimeter;
        }

        total_cost
    }

    fn get_corner_count(region: &HashSet<(usize, usize)>) -> usize {
        let mut corner_count = 0;

        for &(row_index, column_index) in region {
            let is_top = region.contains(&(row_index.wrapping_sub(1), column_index));
            let is_bottom = region.contains(&(row_index + 1, column_index));
            let is_left = region.contains(&(row_index, column_index.wrapping_sub(1)));
            let is_right = region.contains(&(row_index, column_index + 1));

            let is_top_left =
                region.contains(&(row_index.wrapping_sub(1), column_index.wrapping_sub(1)));
            let is_top_right = region.contains(&(row_index.wrapping_sub(1), column_index + 1));
            let is_bottom_left = region.contains(&(row_index + 1, column_index.wrapping_sub(1)));
            let is_bottom_right = region.contains(&(row_index + 1, column_index + 1));

            if !is_top && !is_right || is_top && is_right && !is_top_right {
                corner_count += 1;
            }
            if !is_top && !is_left || is_top && is_left && !is_top_left {
                corner_count += 1;
            }
            if !is_bottom && !is_right || is_bottom && is_right && !is_bottom_right {
                corner_count += 1;
            }
            if !is_bottom && !is_left || is_bottom && is_left && !is_bottom_left {
                corner_count += 1;
            }
        }

        corner_count
    }

    fn get_perimeter(region: &HashSet<(usize, usize)>) -> usize {
        let mut perimeter = 0;

        for &(row_index, column_index) in region {
            let is_top = region.contains(&(row_index.wrapping_sub(1), column_index));
            let is_bottom = region.contains(&(row_index + 1, column_index));
            let is_left = region.contains(&(row_index, column_index.wrapping_sub(1)));
            let is_right = region.contains(&(row_index, column_index + 1));

            if !is_top {
                perimeter += 1;
            }
            if !is_bottom {
                perimeter += 1;
            }
            if !is_left {
                perimeter += 1;
            }
            if !is_right {
                perimeter += 1;
            }
        }

        perimeter
    }
}

fn main() {
    let start_time = Instant::now();
    let input_data = include_str!("../sample.txt");
    let grid = input_data
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<_>>>();

    let mut plots = HashMap::new();

    for (row_index, row) in grid.iter().enumerate() {
        for (column_index, area) in row.iter().enumerate() {
            plots
                .entry(*area)
                .or_insert_with(FarmPlot::new)
                .add_position(row_index, column_index);
        }
    }

    let total_costs_one = plots
        .values()
        .map(FarmPlot::calculate_perimeter_costs)
        .sum::<usize>();

    let total_costs_two = plots
        .values()
        .map(FarmPlot::calculate_corner_costs)
        .sum::<usize>();

    println!("Total Perimeter Costs: {total_costs_one}");

    println!("Total Corner Costs: {total_costs_two}");

    let duration = start_time.elapsed();
    println!(
        "It took {} seconds, {} milliseconds, and {} nanoseconds to calculate the antinodes",
        duration.as_secs(),
        duration.subsec_millis(),
        duration.subsec_nanos() % 1_000_000
    );
}
