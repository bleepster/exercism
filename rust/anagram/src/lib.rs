use std::collections::HashSet;

fn make_comparable(w: &str) -> String {
    let mut t: Vec<u8> = w.bytes().map(|b: u8| b.to_ascii_lowercase()).collect();
    t.sort_by(|a, b| (*a as char).cmp(&(*b as char)));
    String::from_utf8(t).unwrap()
}

pub fn anagrams_for<'a>(word: &'a str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let _word: String = make_comparable(word);
    let anagrams = possible_anagrams
        .iter()
        .filter_map(|a| -> Option<&str> {
            let _a = make_comparable(a);
            match _word == _a {
                true => Some(a),
                _ => None,
            }
        })
        .collect::<Vec<&str>>();
    HashSet::from(anagrams.iter().cloned().collect())
}
