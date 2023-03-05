fn contains_alphabetics(message: &str) -> bool {
    message.chars().any(|c| c.is_alphabetic())
}

fn is_yell(message: &str) -> bool {
    message.to_uppercase() == message && contains_alphabetics(message)
}

fn is_question(message: &str) -> bool {
    message.ends_with("?")
}

pub fn reply(message: &str) -> &str {
    let message = message.trim();

    if message.is_empty() {
        return "Fine. Be that way!";
    }

    if is_yell(message) {
        if is_question(message) {
            return "Calm down, I know what I'm doing!";
        } else {
            return "Whoa, chill out!";
        }
    }

    if is_question(message) {
        return "Sure.";
    }

    return "Whatever.";
}
