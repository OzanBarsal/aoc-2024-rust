use phf::phf_map;
use std::fs;

const DIRECTIONS: [&str; 8] = ["N", "NE", "E", "SE", "S", "SW", "W", "NW"];

static DIRECTIONS_MAP: phf::Map<&'static str, (i32, i32)> = phf_map! {
    "N" => (0, -1),
    "NE" => (1, -1),
    "E" => (1, 0),
    "SE" => (1, 1),
    "S" => (0, 1),
    "SW" => (-1, 1),
    "W" => (-1, 0),
    "NW" => (-1, -1),
};

fn check_direction_for_letter(
    x_coord: usize,
    y_coord: usize,
    rows: &Vec<&str>,
    direction: &str,
    letter: char,
) -> Result<(i32, i32), ()> {
    if let Some(coord_diffs) = DIRECTIONS_MAP.get(direction) {
        let coords_to_check = (
            x_coord as i32 + coord_diffs.0,
            y_coord as i32 + coord_diffs.1,
        );

        if coords_to_check.0 < 0
            || coords_to_check.1 < 0
            || coords_to_check.1 >= rows.len() as i32
            || coords_to_check.0 >= rows[coords_to_check.1 as usize].len() as i32
        {
            Err(())
        } else {
            if rows[coords_to_check.1 as usize]
                .chars()
                .nth(coords_to_check.0 as usize)
                == Some(letter)
            {
                Ok(coords_to_check)
            } else {
                Err(())
            }
        }
    } else {
        Err(())
    }
}

fn get_xmas_matches(x_coord: usize, y_coord: usize, rows: &Vec<&str>) -> i32 {
    let mut matches: i32 = 0;
    for direction in DIRECTIONS {
        let mut new_coords = check_direction_for_letter(x_coord, y_coord, rows, direction, 'M');
        if new_coords.is_ok() {
            new_coords = check_direction_for_letter(
                new_coords.unwrap().0 as usize,
                new_coords.unwrap().1 as usize,
                rows,
                direction,
                'A',
            );
            if new_coords.is_ok() {
                new_coords = check_direction_for_letter(
                    new_coords.unwrap().0 as usize,
                    new_coords.unwrap().1 as usize,
                    rows,
                    direction,
                    'S',
                );
                if new_coords.is_ok() {
                    matches += 1;
                }
            }
        }
    }
    matches
}

fn get_x_mas_matches(x_coord: usize, y_coord: usize, rows: &Vec<&str>) -> i32 {
    let mut matches: i32 = 0;

    let patterns = vec![
        vec![("NW", 'M'), ("SE", 'S'), ("NE", 'M'), ("SW", 'S')],
        vec![("NW", 'S'), ("SE", 'M'), ("NE", 'S'), ("SW", 'M')],
        vec![("NW", 'S'), ("SE", 'M'), ("NE", 'M'), ("SW", 'S')],
        vec![("NW", 'M'), ("SE", 'S'), ("NE", 'S'), ("SW", 'M')],
    ];

    for pattern in patterns {
        let mut valid = true;
        for &(direction, letter) in &pattern {
            if !check_direction_for_letter(x_coord, y_coord, rows, direction, letter).is_ok() {
                valid = false;
                break;
            }
        }
        if valid {
            matches += 1;
        }
    }
    matches
}

pub fn part_one() {
    println!("Solving Day 4 Part 1");

    let text = fs::read_to_string("inputs/day4.txt").unwrap();
    let mut start_coords: Vec<(usize, usize)> = vec![];
    let mut rows: Vec<&str> = vec![];

    for (y_coord, row) in text.lines().enumerate() {
        rows.push(row);
        row.match_indices("X").for_each(|(x_coord, _val)| {
            start_coords.push((x_coord, y_coord));
        });
    }

    let mut total_matches = 0;

    for (x_coord, y_coord) in start_coords {
        total_matches += get_xmas_matches(x_coord, y_coord, &rows);
    }

    println!("Day 4 Part 1 Result is: {}", total_matches);
}

pub fn part_two() {
    println!("Solving Day 4 Part 2");

    let text = fs::read_to_string("inputs/day4.txt").unwrap();
    let mut start_coords: Vec<(usize, usize)> = vec![];
    let mut rows: Vec<&str> = vec![];

    for (y_coord, row) in text.lines().enumerate() {
        rows.push(row);
        row.match_indices("A").for_each(|(x_coord, _val)| {
            start_coords.push((x_coord, y_coord));
        });
    }

    let mut total_matches = 0;

    for (x_coord, y_coord) in start_coords {
        total_matches += get_x_mas_matches(x_coord, y_coord, &rows);
    }

    println!("Day 4 Part 1 Result is: {}", total_matches);
}
