use std::collections::{HashSet, VecDeque};

const GRID_WIDTH: i32 = 101;
const GRID_HEIGHT: i32 = 103;
const SIMULATION_DURATION: i32 = 100;
const MAX_ITERATIONS: i32 = 10_403;
const ADJACENCY_THRESHOLD: f32 = 0.9995;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Robot {
    position_x: i32,
    position_y: i32,
    velocity_x: i32,
    velocity_y: i32,
}

impl Robot {
    fn move_robot(&mut self) {
        self.position_x = (self.position_x + self.velocity_x).rem_euclid(GRID_WIDTH);
        self.position_y = (self.position_y + self.velocity_y).rem_euclid(GRID_HEIGHT);
    }
}

fn count_adjacent_robots(robot_positions: &HashSet<(i32, i32)>) -> usize {
    let movement_directions = [
        (0, 1),   // Down
        (1, 0),   // Right
        (0, -1),  // Up
        (-1, 0),  // Left
        (1, 1),   // Down-Right
        (1, -1),  // Down-Left
        (-1, 1),  // Up-Right
        (-1, -1), // Up-Left
    ];

    let mut visited_positions = HashSet::new();
    let mut total_adjacent_count = 0;

    for &position in robot_positions {
        if visited_positions.contains(&position) {
            continue;
        }

        let mut position_queue = VecDeque::new();
        position_queue.push_back(position);
        let mut count = 0;

        while let Some((x, y)) = position_queue.pop_front() {
            if visited_positions.contains(&(x, y)) {
                continue;
            }
            visited_positions.insert((x, y));
            count += 1;

            for &(dx, dy) in &movement_directions {
                let new_position = (
                    (x + dx).rem_euclid(GRID_WIDTH),
                    (y + dy).rem_euclid(GRID_HEIGHT),
                );
                if robot_positions.contains(&new_position)
                    && !visited_positions.contains(&new_position)
                {
                    position_queue.push_back(new_position);
                }
            }
        }

        total_adjacent_count += count;
    }

    total_adjacent_count
}

fn render_grid(robot_positions: &HashSet<(i32, i32)>) {
    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            if robot_positions.contains(&(x, y)) {
                print!("*");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn main() {
    let input_data = include_str!("../input.txt");

    let mut robots_part_two: Vec<Robot> = input_data
        .lines()
        .filter_map(|line| {
            let parts: Vec<_> = line
                .split([',', 'p', 'v', '=', ' '].as_ref())
                .filter(|&str| !str.is_empty())
                .collect();
            if parts.len() != 4 {
                return None;
            }
            Some(Robot {
                position_x: parts[0].parse().ok()?,
                position_y: parts[1].parse().ok()?,
                velocity_x: parts[2].parse().ok()?,
                velocity_y: parts[3].parse().ok()?,
            })
        })
        .collect();

    let mut robots_part_one = robots_part_two.clone();

    for _ in 0..SIMULATION_DURATION {
        for robot in &mut robots_part_one {
            robot.move_robot();
        }
    }

    for iteration in 0..MAX_ITERATIONS {
        for robot in &mut robots_part_two {
            robot.move_robot();
        }

        let robot_positions = robots_part_two
            .iter()
            .map(|robot| (robot.position_x, robot.position_y))
            .collect::<HashSet<(i32, i32)>>();

        let total_adjacent_count = count_adjacent_robots(&robot_positions);
        let percentage_adjacent = total_adjacent_count as f32 / robots_part_two.len() as f32;

        if percentage_adjacent >= ADJACENCY_THRESHOLD {
            println!(
                "Percentage of adjacent robots: {:.2}%, which exceeds the threshold of {:.2}%",
                percentage_adjacent * 100.0,
                ADJACENCY_THRESHOLD * 100.0
            );
            println!("Loop number: {iteration}");
            println!(
                "The Christmas tree is visible after {} steps",
                iteration + 1
            );
            // render_grid(&robot_positions);
        } else {
            // println!(
            //     "Percentage of adjacent robots: {:.2}%, which is below the threshold of {:.2}%",
            //     percentage_adjacent * 100.0,
            //     ADJACENCY_THRESHOLD * 100.0
            // );
        }
    }

    let mut quadrant_counts = [0, 0, 0, 0];
    for robot in &robots_part_one {
        if robot.position_x == GRID_WIDTH / 2 || robot.position_y == GRID_HEIGHT / 2 {
            continue;
        }

        let quadrant_index = if robot.position_x < GRID_WIDTH / 2 {
            if robot.position_y < GRID_HEIGHT / 2 {
                0 // Top-left
            } else {
                2 // Bottom-left
            }
        } else if robot.position_y < GRID_HEIGHT / 2 {
            1 // Top-right
        } else {
            3 // Bottom-right
        };

        quadrant_counts[quadrant_index] += 1;
    }

    let safety_factor = quadrant_counts.iter().product::<i32>();
    println!("The safety factor is: {safety_factor}");
}
