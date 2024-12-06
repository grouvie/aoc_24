use rayon::prelude::*;
use std::collections::HashSet;
use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_right(self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn move_position(self, current_position: (usize, usize)) -> Option<(usize, usize)> {
        match self {
            Direction::Up => current_position
                .0
                .checked_sub(1)
                .map(|x| (x, current_position.1)),
            Direction::Down => current_position
                .0
                .checked_add(1)
                .map(|x| (x, current_position.1)),
            Direction::Left => current_position
                .1
                .checked_sub(1)
                .map(|y| (current_position.0, y)),
            Direction::Right => current_position
                .1
                .checked_add(1)
                .map(|y| (current_position.0, y)),
        }
    }
}

fn main() {
    let input = include_str!("../sample.txt");
    let grid = input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<_>>>();

    let (guard_start_x, guard_start_y) = find_guard(&grid);

    let visited_positions = find_visited_positions(&grid, (guard_start_x, guard_start_y));

    println!("The guard visited {} positions", visited_positions.len());

    let start_time = Instant::now();

    let circle_obstacle_positions =
        find_circle_obstacle_positions(&grid, (guard_start_x, guard_start_y), &visited_positions);

    println!(
        "There are {} positions that force the guard to walk in a circle",
        circle_obstacle_positions.len()
    );

    let duration = start_time.elapsed();
    println!(
        "It took {} seconds and {} milliseconds to find positions that force the guard to walk in a circle",
        duration.as_secs(),
        duration.subsec_millis()
    );
}

fn find_guard(grid: &[Vec<char>]) -> (usize, usize) {
    grid.par_iter()
        .enumerate()
        .find_map_any(|(row_index, row)| {
            row.par_iter()
                .enumerate()
                .find_map_any(|(col_index, &cell)| match cell {
                    '^' | 'v' | '<' | '>' => Some((row_index, col_index)),
                    _ => None,
                })
        })
        .expect("Guard not found in the grid!")
}

fn find_visited_positions(
    grid: &[Vec<char>],
    start_position: (usize, usize),
) -> HashSet<(usize, usize)> {
    let mut visited_positions = HashSet::new();
    visited_positions.insert(start_position);

    let mut current_position = start_position;
    let mut current_direction = Direction::Up;

    while let Some(next_position) = current_direction.move_position(current_position) {
        // Check if next_position is out of bounds
        if next_position.0 >= grid.len() || next_position.1 >= grid[0].len() {
            break; // Stop the loop if out of bounds
        }

        // Check for obstacle
        if grid[next_position.0][next_position.1] == '#' {
            current_direction = current_direction.turn_right();
            continue;
        }

        visited_positions.insert(next_position);
        current_position = next_position;
    }

    visited_positions
}

fn find_circle_obstacle_positions(
    grid: &[Vec<char>],
    start_position: (usize, usize),
    visited_positions: &HashSet<(usize, usize)>,
) -> HashSet<(usize, usize)> {
    visited_positions
        .par_iter()
        .filter_map(|&(row_index, col_index)| {
            if grid[row_index][col_index] == '.' {
                // Simulate with an obstacle at (row_index, col_index)
                let mut temp_grid = grid.to_vec();
                temp_grid[row_index][col_index] = 'O';

                let loop_count = simulate_guard(&temp_grid, start_position);
                (loop_count > 0).then_some((row_index, col_index))
            } else {
                None
            }
        })
        .collect::<HashSet<_>>()
}

/*
fn find_circle_obstacle_positions(
    grid: &[Vec<char>],
    start_position: (usize, usize),
) -> HashSet<(usize, usize)> {
    (0..grid.len())
        .into_par_iter()
        .flat_map(|row_index| {
            (0..grid[0].len())
                .into_par_iter()
                .filter_map(|col_index| {
                    if grid[row_index][col_index] == '.' {
                        // Simulate with an obstacle at (row_index, col_index)
                        let mut temp_grid = grid.to_vec();
                        temp_grid[row_index][col_index] = 'O';

                        let loop_count = simulate_guard(&temp_grid, start_position);
                        (loop_count > 0).then_some((row_index, col_index))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<HashSet<_>>()
}
*/

fn simulate_guard(grid: &[Vec<char>], start_position: (usize, usize)) -> usize {
    let mut visited_positions = HashSet::new();
    let mut current_position = start_position;
    let mut current_direction = match grid[start_position.0][start_position.1] {
        '^' => Direction::Up,
        'v' => Direction::Down,
        '<' => Direction::Left,
        '>' => Direction::Right,
        _ => unreachable!("Invalid guard position!"),
    };

    visited_positions.insert((current_position, current_direction));

    while let Some(next_position) = current_direction.move_position(current_position) {
        // Check if next_position is out of bounds
        if next_position.0 >= grid.len() || next_position.1 >= grid[0].len() {
            break; // Stop the loop if out of bounds
        }

        // Check for obstacles
        if grid[next_position.0][next_position.1] == '#'
            || grid[next_position.0][next_position.1] == 'O'
        {
            current_direction = current_direction.turn_right();
            continue;
        }

        // Check if the guard revisits a position with the same direction
        if visited_positions.contains(&(next_position, current_direction)) {
            return visited_positions.len(); // Guard is in a loop
        }

        visited_positions.insert((next_position, current_direction));
        current_position = next_position;
    }

    0 // No loop detected
}
