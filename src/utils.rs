use std::collections::HashMap;

pub fn count_occurrences(vec1: &Vec<String>, vec2: &Vec<String>) -> Vec<usize> {
    let vec2_counts: HashMap<_, _> = vec2.iter().fold(HashMap::new(), |mut map, item| {
        *map.entry(item).or_insert(0) += 1;
        map
    });

    vec1.iter()
        .map(|item| *vec2_counts.get(item).unwrap_or(&0))
        .collect()
}

pub fn _remove_char_at(s: &str, index: usize) -> String {
    let mut result = String::with_capacity(s.len() - 1);
    result.push_str(&s[..index]);
    result.push_str(&s[index + 1..]);
    result
}
