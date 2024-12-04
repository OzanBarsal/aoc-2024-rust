use fancy_regex::Regex;
use std::fs;

fn get_valid_sections(input: &str) -> Vec<String> {
    let re = Regex::new(
        r"(?s)^(?:(?!don't\(\)).)*?(?:don't\(\)|$)|do\(\)(?:(?!don't\(\)).)*?(?:don't\(\)|$)",
    )
    .unwrap();

    re.captures_iter(input)
        .filter_map(|cap| cap.ok())
        .map(|cap| cap[0].to_string())
        .collect()
}

fn get_result(input: &str) -> i32 {
    let rx = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    rx.captures_iter(&input).fold(0, |acc, result| {
        let capture = result.unwrap();
        acc + (capture[1].parse::<i32>().unwrap() * capture[2].parse::<i32>().unwrap())
    })
}

pub fn part_one() {
    println!("Solving Day 3 Part 1");
    let text = fs::read_to_string("inputs/day3.txt").unwrap();

    println!("Day 3 Part 1 Result is: {}", get_result(&text));
}

pub fn part_two() {
    println!("Solving Day 3 Part 2");
    let text = fs::read_to_string("inputs/day3.txt").unwrap();

    let valid_sections = get_valid_sections(&text);

    let sum: i32 = valid_sections
        .iter()
        .map(|section| get_result(section))
        .sum();

    println!("Day 3 Part 2 Result is: {}", sum);
}
