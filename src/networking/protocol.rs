pub const SEPERATOR: char = '+';
pub const TEXT: char = 't';
pub const NAME: char = 'n';

pub fn encode_message(to_type: char, content: &String) -> String {
    if to_type == TEXT {
        return content.clone();
    }

    return format!("{to_type}{SEPERATOR}{content}");
}

pub fn decode_message(from_type: char, message: &String) -> String {
    if from_type == TEXT {
        return message.clone();
    }

    let strip_result = message.strip_prefix(&format!("{from_type}{SEPERATOR}"));
    return match strip_result {
        Some(content) => content.to_string(),
        None => message.clone()
    }
}

pub fn get_message_type(message: &String) -> char {
    if message.len() < 2 {
        return TEXT;
    }

    if message.chars().nth(1).unwrap() != SEPERATOR {
        return TEXT;
    }

    return message.chars().nth(0).unwrap();
}
