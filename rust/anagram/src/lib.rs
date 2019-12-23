use std::collections::HashMap;
use std::collections::HashSet;

fn create_char_map(s: &str) -> HashMap<char, u32> {
    let mut m = HashMap::new();
    for c in s.chars() {
        if let Some(x) = m.get_mut(&c) {
            *x = *x + 1;
        } else {
            m.insert(c, 1);
        }
    }
    m
}

fn is_match(word: &HashMap<char, u32>, maybe_anagram: &HashMap<char, u32>) -> bool {
    for (k, v) in word.iter() {
        if let Some(x) = maybe_anagram.get(&k) {
            if v != x {
                return false;
            }
        } else {
            return false;
        }
    }
    true
}

pub fn anagrams_for<'a>(word: &'a str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let w_lowered = word.to_lowercase();
    let w_mapped = create_char_map(&w_lowered);
    let anagrams = possible_anagrams
        .iter()
        .filter_map(|a| -> Option<&str> {
            let a_lowered = a.to_lowercase();
            let a_mapped = create_char_map(&a_lowered);
            match (
                word.len() == a.len(),
                w_lowered != a_lowered,
                is_match(&w_mapped, &a_mapped),
            ) {
                (true, true, true) => Some(a),
                _ => None,
            }
        })
        .collect::<Vec<&str>>();
    HashSet::from(anagrams.iter().cloned().collect())
}
