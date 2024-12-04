use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let path = "./four/sample.txt";

    let grid = read_file_to_grid(path)?;

    let horizontal_count = count_horizontal(&grid, "XMAS");
    let vertical_count = count_vertical(&grid, "XMAS");
    let diagonal_count = count_diagonal(&grid, "XMAS");
    let reverse_count = count_reverse(&grid, "XMAS");
    let total_count = horizontal_count + vertical_count + diagonal_count + reverse_count;
    let x_shape_count = count_x_shape(&grid);

    println!("Horizontal occurrences of 'XMAS': {horizontal_count}");
    println!("Vertical occurrences of 'XMAS': {vertical_count}");
    println!("Diagonal occurrences of 'XMAS': {diagonal_count}");
    println!("Reverse occurrences of 'XMAS': {reverse_count}");
    println!("Total occurences of 'XMAS': {total_count}");
    println!("X of 'XMAS': {x_shape_count}");

    Ok(())
}

fn read_file_to_grid(filename: &str) -> io::Result<Vec<Vec<char>>> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    let mut grid = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let chars: Vec<char> = line.chars().collect();
        grid.push(chars);
    }

    Ok(grid)
}

fn count_horizontal(grid: &[Vec<char>], word: &str) -> usize {
    let mut count = 0;

    for row in grid {
        let row_str: String = row.iter().collect();
        count += row_str.matches(word).count();
    }

    count
}

fn count_vertical(grid: &[Vec<char>], word: &str) -> usize {
    let mut count = 0;
    let num_rows = grid.len();
    let num_cols = grid[0].len();

    for col in 0..num_cols {
        let mut column_str = String::new();
        for row in 0..num_rows {
            column_str.push(grid[row][col]);
        }
        count += column_str.matches(word).count();
    }

    count
}

fn count_diagonal(grid: &[Vec<char>], word: &str) -> usize {
    let mut count = 0;
    let word_len = word.len();
    let num_rows = grid.len();
    let num_cols = grid[0].len();

    for row in 0..num_rows {
        for col in 0..num_cols {
            if row + word_len <= num_rows && col + word_len <= num_cols {
                let mut diagonal_str = String::new();
                for i in 0..word_len {
                    diagonal_str.push(grid[row + i][col + i]);
                }
                count += diagonal_str.matches(word).count();
            }
        }
    }

    for row in 0..num_rows {
        for col in 0..num_cols {
            if row + word_len <= num_rows && col >= word_len - 1 {
                let mut diagonal_str = String::new();
                for i in 0..word_len {
                    diagonal_str.push(grid[row + i][col - i]);
                }
                count += diagonal_str.matches(word).count();
            }
        }
    }

    count
}

fn count_reverse(grid: &[Vec<char>], word: &str) -> usize {
    let reversed_word: String = word.chars().rev().collect();
    let horizontal_count = count_horizontal(grid, &reversed_word);
    let vertical_count = count_vertical(grid, &reversed_word);
    let diagonal_count = count_diagonal(grid, &reversed_word);

    horizontal_count + vertical_count + diagonal_count
}

fn count_x_shape(grid: &[Vec<char>]) -> usize {
    let mut count = 0;
    let num_rows = grid.len();
    let num_cols = grid[0].len();

    for row in 0..num_rows {
        for col in 0..num_cols {
            if row + 2 < num_rows && col + 2 < num_cols {
                count += check_x_shape(grid, row, col);
            }
        }
    }

    count
}

fn check_x_shape(grid: &[Vec<char>], row: usize, col: usize) -> usize {
    let mut count = 0;

    let top_left = grid[row][col];
    let center = grid[row + 1][col + 1];
    let bottom_left = grid[row + 2][col];
    let top_right = grid[row][col + 2];
    let bottom_right = grid[row + 2][col + 2];

    //   M . S
    //   . A .
    //   M . S
    if top_left == 'M'
        && center == 'A'
        && bottom_left == 'M'
        && top_right == 'S'
        && bottom_right == 'S'
    {
        count += 1;
    }

    //   S . M
    //   . A .
    //   S . M
    if top_left == 'S'
        && center == 'A'
        && bottom_left == 'S'
        && top_right == 'M'
        && bottom_right == 'M'
    {
        count += 1;
    }

    //   M . M
    //   . A .
    //   S . S
    if top_left == 'M'
        && center == 'A'
        && bottom_left == 'S'
        && top_right == 'M'
        && bottom_right == 'S'
    {
        count += 1;
    }

    //   S . S
    //   . A .
    //   M . M
    if top_left == 'S'
        && center == 'A'
        && bottom_left == 'M'
        && top_right == 'S'
        && bottom_right == 'M'
    {
        count += 1;
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_x_shape() {
        let grid0 = vec![
            vec!['M', ' ', 'M'],
            vec![' ', 'A', ' '],
            vec!['S', ' ', 'S'],
        ];
        assert_eq!(count_x_shape(&grid0), 1);

        let grid1 = vec![
            vec!['M', ' ', 'M'],
            vec![' ', 'A', ' '],
            vec!['S', ' ', 'S'],
            vec!['S', ' ', 'M'],
            vec![' ', 'A', ' '],
            vec!['S', ' ', 'M'],
        ];
        assert_eq!(count_x_shape(&grid1), 2);

        let grid2 = vec![
            vec!['S', ' ', 'S'],
            vec![' ', 'A', ' '],
            vec!['M', ' ', 'M'],
            vec!['S', ' ', 'S'],
            vec![' ', 'A', ' '],
            vec!['M', ' ', 'M'],
        ];
        assert_eq!(count_x_shape(&grid2), 2);

        let grid3 = vec![
            vec!['M', ' ', 'S'],
            vec![' ', 'A', ' '],
            vec!['M', ' ', 'S'],
            vec!['S', ' ', 'M'],
            vec![' ', 'A', ' '],
            vec!['S', ' ', 'M'],
        ];
        assert_eq!(count_x_shape(&grid3), 2);

        let grid4 = vec![
            vec!['M', ' ', 'S', 'M', ' ', 'S'],
            vec![' ', 'A', ' ', ' ', 'A', ' '],
            vec!['M', ' ', 'S', 'M', ' ', 'S'],
            vec!['S', ' ', 'M', 'S', ' ', 'S'],
            vec![' ', 'A', ' ', 'A', 'A', ' '],
            vec!['S', ' ', 'M', 'M', ' ', 'M'],
        ];
        assert_eq!(count_x_shape(&grid4), 4);
    }
}
