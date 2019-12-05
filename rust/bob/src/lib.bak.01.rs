fn is_all_caps(_message: &str) -> bool {
    for c in _message.chars() {
        if c.is_ascii_alphabetic() && c.is_ascii_lowercase() {
            return false;
        }
    }
    true
}

fn is_all_digits(_message: &str) -> bool {
    for c in _message.chars() {
        if c.is_ascii_alphabetic() && !c.is_ascii_digit() {
            return false;
        }
    }
    true
}

fn is_all_whitespace(_message: &str) -> bool {
    for c in _message.chars() {
        if !c.is_ascii_whitespace() {
            return false;
        }
    }
    true
}

fn is_a_question(_message: &str) -> bool {
    let mut last = '.';
    for c in _message.chars() {
        if !c.is_ascii_whitespace() {
            last = c;
        }
    }
    if last != '?' {
        return false;
    }
    true
}

pub fn reply(message: &str) -> &str {
    if message.len() == 0 || is_all_whitespace(message) {
        "Fine. Be that way!"
    } else if is_a_question(message) {
        if is_all_digits(message) {
            "Sure."
        } else if is_all_caps(message) {
            "Calm down, I know what I'm doing!"
        } else {
            "Sure."
        }
    } else if message.chars().last().unwrap().is_ascii_whitespace() || is_all_digits(message) {
        "Whatever."
    } else if is_all_caps(message) {
        "Whoa, chill out!"
    } else if message.chars().last().unwrap() == '!' {
        "Whatever."
    } else {
        "Whatever."
    }
}
