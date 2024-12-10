use pathfinding::prelude::bfs_reach;
use std::collections::{HashSet, VecDeque};

const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)]; // right, down, left, up

fn calculate_total_score(map: &[&str]) -> usize {
    map.iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.chars()
                .enumerate()
                .filter(|&(_x, cell)| cell == '0')
                .map(move |(x, _cell)| reachable_nines(map, (x, y)))
        })
        .sum()
}

fn reachable_nines(map: &[&str], start: (usize, usize)) -> usize {
    let successors = |&(x, y): &(usize, usize)| {
        let current_height = map[y].chars().nth(x).unwrap().to_digit(10).unwrap() as isize;

        DIRECTIONS.iter().filter_map(move |&(dx, dy)| {
            let new_x = x as isize + dx;
            let new_y = y as isize + dy;

            if new_x >= 0 && new_y >= 0 {
                let new_x = new_x as usize;
                let new_y = new_y as usize;

                if new_y < map.len() && new_x < map[new_y].len() {
                    let cell = map[new_y].chars().nth(new_x).unwrap();

                    if let Some(new_height) = cell.to_digit(10) {
                        if (new_height as isize - current_height) == 1 {
                            return Some((new_x, new_y));
                        }
                    }
                }
            }

            None
        })
    };

    let reachable_nodes: HashSet<(usize, usize)> = bfs_reach(start, successors).collect();
    reachable_nodes
        .iter()
        .filter(|&&(x, y)| map[y].chars().nth(x).unwrap() == '9')
        .count()
}

fn calculate_total_rating(map: &[&str]) -> usize {
    map.iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.chars()
                .enumerate()
                .filter(|&(_x, cell)| cell == '0')
                .map(move |(x, _cell)| count_distinct_paths(map, (x, y)))
        })
        .sum()
}

fn count_distinct_paths(map: &[&str], start: (usize, usize)) -> usize {
    let mut visited = HashSet::new();
    let mut paths = HashSet::new();
    let mut path = VecDeque::new();
    path.push_back(start);

    dfs(map, start, &mut visited, &mut path, &mut paths, &DIRECTIONS);

    paths.len()
}

fn dfs(
    map: &[&str],
    current: (usize, usize),
    visited: &mut HashSet<(usize, usize)>,
    path: &mut VecDeque<(usize, usize)>,
    paths: &mut HashSet<VecDeque<(usize, usize)>>,
    directions: &[(isize, isize)],
) {
    let (x, y) = current;
    let current_height = map[y].chars().nth(x).unwrap().to_digit(10).unwrap() as isize;

    if map[y].chars().nth(x).unwrap() == '9' {
        paths.insert(path.clone());
        return;
    }

    visited.insert((x, y));

    for &(dx, dy) in directions {
        let new_x = x as isize + dx;
        let new_y = y as isize + dy;

        if new_x >= 0 && new_y >= 0 {
            let new_x = new_x as usize;
            let new_y = new_y as usize;

            if new_y < map.len() && new_x < map[new_y].len() {
                let cell = map[new_y].chars().nth(new_x).unwrap();

                if !visited.contains(&(new_x, new_y)) {
                    if let Some(new_height) = cell.to_digit(10) {
                        if (new_height as isize - current_height) == 1 {
                            path.push_back((new_x, new_y));
                            dfs(map, (new_x, new_y), visited, path, paths, directions);
                            path.pop_back(); // Backtrack
                        }
                    }
                }
            }
        }
    }

    visited.remove(&(x, y));
}

fn main() {
    let input_data = include_str!("../input.txt");
    let map: Vec<&str> = input_data.lines().collect();

    let total_score = calculate_total_score(&map);
    let total_rating = calculate_total_rating(&map);

    println!("Total score: {}", total_score);
    println!("Total rating: {}", total_rating);
}
