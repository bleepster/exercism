use std::collections::HashMap;
use std::collections::HashSet;

fn create_char_map(s: &str) -> HashMap<char, u32> {
    let mut m = HashMap::with_capacity(s.len());
    for c in s.chars() {
        m.insert(c, m.get(&c).or_else(|| Some(&0)).unwrap() + 1);
    }
    m
}

pub fn anagrams_for<'a>(word: &'a str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let w_lowered = word.to_lowercase();
    let w_mapped = create_char_map(&w_lowered);
    let anagrams = possible_anagrams
        .iter()
        .filter_map(|a| -> Option<&str> {
            let a_lowered = a.to_lowercase();
            if w_lowered == a_lowered {
                return None;
            }

            let mut w = w_mapped.clone();
            for c in a_lowered.chars() {
                if let Some(x) = w.get_mut(&c) {
                    *x = *x - 1;
                    if *x == 0 {
                        w.remove(&c);
                    }
                } else {
                    return None;
                }
            }

            if !w.is_empty() {
                None
            } else {
                Some(a)
            }
        })
        .collect::<Vec<&str>>();
    HashSet::from(anagrams.iter().cloned().collect())
}
