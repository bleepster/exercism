fn is_all_caps(_message: &str) -> bool {
    if _message.is_empty() {
        return false;
    }
    let up: Vec<&str> = _message
        .get(0.._message.len() - 1)
        .unwrap()
        .matches(|c: char| {
            !c.is_whitespace() && (!c.is_ascii_alphabetic() || c.is_ascii_lowercase())
        })
        .collect();
    up.len() == 0
}

fn is_shouting(_message: &str) -> bool {
    if !_message.ends_with("!") {
        return false;
    }
    let up: Vec<&str> = _message.matches(char::is_uppercase).collect();
    up.len() > 1
}

pub fn reply(message: &str) -> &str {
    let m = message.trim();
    match (
        m.is_empty(),
        m.ends_with("?"),
        is_all_caps(m),
        is_shouting(m),
    ) {
        (true, _, _, _) => "Fine. Be that way!",
        (_, true, true, _) => "Calm down, I know what I'm doing!",
        (_, true, false, _) => "Sure.",
        (_, false, true, _) | (_, _, _, true) => "Whoa, chill out!",
        _ => "Whatever.",
    }
}
