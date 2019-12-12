use std::collections::HashSet;

fn make_comparable(w: &str) -> String {
    let mut t: Vec<u8> = w.bytes().collect();
    t.sort_by(|a, b| a.cmp(b));
    String::from_utf8_lossy(&t).to_string()
}

pub fn anagrams_for<'a>(word: &'a str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let _word: String = make_comparable(&word.to_lowercase());
    let anagrams = possible_anagrams
        .iter()
        .filter_map(|a| -> Option<&str> {
            let _a = make_comparable(&a.to_lowercase());
            match (a.to_lowercase() != word.to_lowercase(), _word == _a) {
                (true, true) => Some(a),
                _ => None,
            }
        })
        .collect::<Vec<&str>>();
    HashSet::from(anagrams.iter().cloned().collect())
}
