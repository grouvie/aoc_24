use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::time::Instant;

fn main() {
    let start_time = Instant::now();

    let grid = parse_input(include_str!("../sample.txt"));

    let antennas = collect_antennas(&grid);

    let rule1_antinodes = calculate_antinodes(&grid, &antennas, find_antinodes_rule1);
    let rule2_antinodes = calculate_antinodes(&grid, &antennas, find_antinodes_rule2);

    // Generate separate grids for Rule 1 and Rule 2
    let (grid_rule1, grid_rule2) = visualize_grids(&grid, &rule1_antinodes, &rule2_antinodes);

    // Print both grids side by side
    print_grids(&grid_rule1, &grid_rule2);

    println!(
        "\nTotal unique antinodes (Rule 1): {}",
        rule1_antinodes.len()
    );
    println!("Total unique antinodes (Rule 2): {}", rule2_antinodes.len());

    let duration = start_time.elapsed();
    println!(
        "It took {} seconds, {} milliseconds, and {} nanoseconds to calculate the antinodes",
        duration.as_secs(),
        duration.subsec_millis(),
        duration.subsec_nanos() % 1_000_000
    );
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn collect_antennas(grid: &[Vec<char>]) -> HashMap<char, Vec<(usize, usize)>> {
    let mut antennas = HashMap::<char, Vec<(usize, usize)>>::new();
    for (y, row) in grid.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell.is_alphanumeric() {
                antennas.entry(cell).or_default().push((x, y));
            }
        }
    }
    antennas
}

fn calculate_antinodes<F>(
    grid: &[Vec<char>],
    antennas: &HashMap<char, Vec<(usize, usize)>>,
    rule_fn: F,
) -> HashSet<(usize, usize)>
where
    F: Fn(&[Vec<char>], &[(usize, usize)]) -> HashSet<(usize, usize)> + Sync,
{
    antennas
        .par_iter()
        .map(|(_, positions)| rule_fn(grid, positions))
        .reduce(HashSet::new, |mut acc, antinodes| {
            acc.extend(antinodes);
            acc
        })
}

fn visualize_grids(
    grid: &[Vec<char>],
    rule1_antinodes: &HashSet<(usize, usize)>,
    rule2_antinodes: &HashSet<(usize, usize)>,
) -> (Vec<Vec<char>>, Vec<Vec<char>>) {
    let mut grid_rule1 = grid.to_vec();
    let mut grid_rule2 = grid.to_vec();

    for &(x, y) in rule1_antinodes {
        if grid_rule1[y][x] == '.' {
            grid_rule1[y][x] = '#';
        }
    }

    for &(x, y) in rule2_antinodes {
        if grid_rule2[y][x] == '.' {
            grid_rule2[y][x] = '#';
        }
    }

    (grid_rule1, grid_rule2)
}

fn print_grids(grid1: &[Vec<char>], grid2: &[Vec<char>]) {
    let max_width = grid1.iter().map(Vec::len).max().unwrap_or(0);

    println!(
        "{:<width$} | {:<width$}",
        "Grid (Rule 1)",
        "Grid (Rule 2)",
        width = max_width
    );

    println!(
        "{:-<separator_width$}",
        "",
        separator_width = max_width * 2 + 3
    );

    for (row1, row2) in grid1.iter().zip(grid2.iter()) {
        let row1_str: String = row1.iter().collect();
        let row2_str: String = row2.iter().collect();
        println!("{row1_str:<max_width$} | {row2_str:<max_width$}");
    }
}

fn find_antinodes_rule1(
    grid: &[Vec<char>],
    positions: &[(usize, usize)],
) -> HashSet<(usize, usize)> {
    positions
        .par_iter()
        .enumerate()
        .flat_map_iter(|(i, &(x1, y1))| {
            positions.iter().skip(i + 1).map(move |&(x2, y2)| {
                let dx = x2 as isize - x1 as isize;
                let dy = y2 as isize - y1 as isize;

                let antinode1 = (x1 as isize - dx, y1 as isize - dy);
                let antinode2 = (x2 as isize + dx, y2 as isize + dy);

                let valid_antinode1 = is_valid_antinode(antinode1, grid)
                    .then_some((antinode1.0 as usize, antinode1.1 as usize));

                let valid_antinode2 = is_valid_antinode(antinode2, grid)
                    .then_some((antinode2.0 as usize, antinode2.1 as usize));

                valid_antinode1.into_iter().chain(valid_antinode2)
            })
        })
        .flatten_iter()
        .collect()
}

fn find_antinodes_rule2(
    grid: &[Vec<char>],
    positions: &[(usize, usize)],
) -> HashSet<(usize, usize)> {
    let mut antinodes = HashSet::new();

    for &(x1, y1) in positions {
        positions
            .iter()
            .filter(|&&position| position != (x1, y1))
            .for_each(|&(x2, y2)| {
                let dx = x2 as isize - x1 as isize;
                let dy = y2 as isize - y1 as isize;
                let mut x = x1 as isize;
                let mut y = y1 as isize;

                // Collect antinodes along the direction from (x1, y1) to (x2, y2)
                while is_valid_antinode((x, y), grid) {
                    antinodes.insert((x as usize, y as usize));
                    x += dx;
                    y += dy;
                }
            });
    }

    antinodes
}

fn is_valid_antinode((x, y): (isize, isize), grid: &[Vec<char>]) -> bool {
    x >= 0 && y >= 0 && (y as usize) < grid.len() && (x as usize) < grid[0].len()
}
