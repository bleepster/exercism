use std::collections::HashSet;

fn total_bytes(s: &str) -> u32 {
    s.to_lowercase().bytes().fold(0, |a, b| a as u32 + b as u32)
}

pub fn anagrams_for<'a>(word: &'a str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let w_bytes: u32 = total_bytes(word);
    let is_word_ascii = word.chars().all(|c| c.is_ascii());
    let w_lower = word.to_lowercase();
    let anagrams = possible_anagrams
        .iter()
        .filter_map(|x| -> Option<&str> {
            let x_lower = &x.to_lowercase();
            match (
                &w_lower != x_lower, // not equal to self regardless of case
                is_word_ascii && x.chars().all(|c| c.is_ascii()), // both words are ascii
                w_bytes == total_bytes(x), // equal by bytes, regardless of order
                word == w_lower && x == x_lower, // non-ascii and no case
                &word == x,          // exact match
            ) {
                (true, true, true, _, _) => Some(x),
                (true, false, true, false, false) => Some(x),
                _ => None,
            }
        })
        .collect::<Vec<&str>>();
    HashSet::from(anagrams.iter().cloned().collect())
}
