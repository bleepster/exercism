use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &'a str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let w_bytes: u32 = word
        .to_lowercase()
        .bytes()
        .fold(0, |a, b| a as u32 + b as u32);
    let anagrams = possible_anagrams
        .iter()
        .filter_map(|x| -> Option<&str> {
            let x_bytes: u32 = x.to_lowercase().bytes().fold(0, |a, b| a as u32 + b as u32);
            match (
                &word.to_lowercase() != &x.to_lowercase(),
                w_bytes == x_bytes,
            ) {
                (true, true) => Some(x),
                _ => None,
            }
        })
        .collect::<Vec<&str>>();
    HashSet::from(anagrams.iter().cloned().collect())
}
