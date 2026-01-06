pub const SEPERATOR: char = '+';
pub const NAME: char = 'n';

pub fn encode_message(to_type: char, content: &String) -> String {
    return format!("{to_type}{SEPERATOR}{content}");
}

pub fn decode_message(from_type: char, message: &String) -> Option<&str> {
    return message.strip_prefix(&format!("{from_type}{SEPERATOR}"))
}
