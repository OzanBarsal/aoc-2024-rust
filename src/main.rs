use std::fs;
use std::collections::HashMap;

fn get_sorted_cols() -> Result<Vec<Vec<String>>, std::io::Error> {
    let text = fs::read_to_string("inputs/day1.txt").unwrap();

    let mut cols: Vec<Vec<String>> = vec![vec![], vec![]];

    for line in text.lines() {
        let split_line = line.split_whitespace();

        for (i, value) in split_line.enumerate() {
            cols[i].push(value.to_string());
        }
    }

    for list in cols.iter_mut() {
        list.sort();
    }

    Ok(cols)
}


fn day_one_part_one() {
    println!("Solving Day 1 Part 1");

    let cols = get_sorted_cols().unwrap();
    let mut sum: i32 = 0;

    for i in 0..cols[0].len() {
        let values: (i32, i32) = (cols[0][i].parse().unwrap(), cols[1][i].parse().unwrap());
        sum += (values.0 - values.1).abs();
    }

    println!("Day 1 Part 1 Result is: {}", sum);
}

fn count_occurrences(vec1: &Vec<String>, vec2: &Vec<String>) -> Vec<usize> {
    let vec2_counts: HashMap<_, _> = vec2.iter()
        .fold(HashMap::new(), |mut map, item| {
            *map.entry(item).or_insert(0) += 1;
            map
        });

    vec1.iter()
        .map(|item| *vec2_counts.get(item).unwrap_or(&0))
        .collect()
}

fn day_one_part_two() {
    println!("Solving Day 1 Part 2");

    let cols = get_sorted_cols().unwrap();
    let mut sum: i32 = 0;

    let counts = count_occurrences(&cols[0], &cols[1]);

    for (i, value) in cols[0].iter().enumerate() {
        let parsed_value: i32 = value.parse().unwrap();
        sum += parsed_value * counts[i] as i32;
    }

    println!("Day 1 Part 2 Result is: {}", sum);
}

fn main() {
    day_one_part_one();
    day_one_part_two();
}
