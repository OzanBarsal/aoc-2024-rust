use std::fs;

fn validate_line(values: &Vec<i32>) -> Result<(), usize> {
    let mut is_ascending: bool = false;

    for (index, value) in values.iter().enumerate() {
        if index + 1 < values.len() {
            let next_value: i32 = values[index + 1];

            if (next_value - value).abs() > 3 || (next_value - value) == 0 {
                return Err(index);
            }

            if index == 0 {
                if value < &next_value {
                    is_ascending = true;
                } else if value > &next_value {
                    is_ascending = false;
                } else {
                    return Err(index);
                }
            }

            if is_ascending && value > &next_value {
                return Err(index);
            }

            if !is_ascending && value < &next_value {
                return Err(index);
            }
        }
    }

    Ok(())
}

pub fn part_one() {
    println!("Solving Day 2 Part 1");

    let text = fs::read_to_string("inputs/day2.txt").unwrap();
    let mut valid_lines: i32 = 0;

    for line in text.lines() {
        let values: Vec<i32> = line
            .split_whitespace()
            .map(|str| str.parse::<i32>().unwrap())
            .collect();

        match validate_line(&values) {
            Ok(()) => {
                valid_lines += 1;
            }
            Err(_e) => {}
        }
    }

    println!("Day 2 Part 1 Result is: {}", valid_lines);
}

pub fn part_two() {
    println!("Solving Day 2 Part 2");

    let text = fs::read_to_string("inputs/day2.txt").unwrap();
    let mut valid_lines: i32 = 0;

    for line in text.lines() {
        let values: Vec<i32> = line
            .split_whitespace()
            .map(|str| str.parse::<i32>().unwrap())
            .collect();

        match validate_line(&values) {
            Ok(()) => {
                valid_lines += 1;
            }
            Err(error_index) => {
                let mut new_values = values.clone();
                new_values.remove(error_index + 1);
                match validate_line(&new_values) {
                    Ok(()) => {
                        valid_lines += 1;
                    }
                    Err(_e) => {
                        new_values = values.clone();
                        new_values.remove(error_index);

                        match validate_line(&new_values) {
                            Ok(()) => {
                                valid_lines += 1;
                            }
                            Err(_e) => {
                                new_values = values.clone();
                                new_values.remove(0);

                                match validate_line(&new_values) {
                                    Ok(()) => {
                                        valid_lines += 1;
                                    }
                                    Err(_e) => {}
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    println!("Day 2 Part 2 Result is: {}", valid_lines);
}
