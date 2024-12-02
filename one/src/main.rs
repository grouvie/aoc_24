use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = Path::new("./one/sample.txt");
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let (mut first_list, mut second_list): (Vec<i32>, Vec<i32>) = reader
        .lines()
        .map_while(Result::ok)
        .filter_map(|line| {
            let parts = line
                .split_whitespace()
                .filter_map(|input| input.parse::<i32>().ok())
                .collect::<Vec<_>>();
            (parts.len() == 2).then(|| (parts[0], parts[1]))
        })
        .unzip();

    println!("First list: {first_list:#?}");
    println!("Second list: {second_list:#?}");

    first_list.sort_unstable();
    second_list.sort_unstable();

    let distances = first_list
        .iter()
        .zip(&second_list)
        .map(|(&first, &second)| (first - second).abs())
        .collect::<Vec<_>>();

    println!("Individual distances: {distances:#?}");

    let combined_distances: i32 = distances.iter().sum();

    println!("Sum of distances: {combined_distances}");

    let counting_map: HashMap<i32, i32> =
        second_list.iter().fold(HashMap::new(), |mut acc, &second| {
            *acc.entry(second).or_insert(0) += 1;
            acc
        });

    let similarity_scores: Vec<i32> = first_list
        .iter()
        .filter_map(|&first| counting_map.get(&first).map(|&count| first * count))
        .collect();

    println!("Similarity scores: {similarity_scores:#?}");

    let combined_similarity_scores: i32 = similarity_scores.iter().sum();
    println!("Sum of similarity scores: {combined_similarity_scores}");

    Ok(())
}
