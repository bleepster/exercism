fn format(input: impl Iterator<Item = char>) -> String {
    input.enumerate().fold(String::new(), |mut res, (i, c)| {
        if i > 0 && i % 5 == 0 {
            res.push(' ');
        }
        res.push(c);
        res
    })
}

fn atbash(input: &str) -> impl Iterator<Item = char> + '_ {
    input.bytes().filter_map(|b: u8| -> Option<char> {
        let l = b.to_ascii_lowercase();
        match (l.is_ascii_alphanumeric(), l.is_ascii_alphabetic()) {
            (true, true) => Some((b'a' + (b'z' - l)) as char),
            (true, false) => Some(l as char),
            _ => None,
        }
    })
}

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    format(atbash(plain))
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    atbash(cipher).collect::<String>()
}
