use std::collections::{HashMap};

pub fn get_freqs(text: &str) -> HashMap<char, usize> {
    let mut freqs: HashMap<char, usize> = HashMap::new();
    for c in text.chars() {
        *freqs.entry(c).or_insert(0) += 1;
    }
    freqs
}

pub fn get_priority_vec(hashmap: &HashMap<char, usize>) ->Vec<(char, usize)> {

    let mut priority_vec: Vec<_> = hashmap.iter()
        .map(|(&key, &value)| (key, value))
        .collect();
    priority_vec.sort_by_key(|&(_, freq)| freq);
    priority_vec
}

