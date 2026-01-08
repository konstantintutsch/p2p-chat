pub const SEPERATOR: char = '+';

#[derive(PartialEq)]
pub enum MessageType {
    Text,
    Name
}

impl MessageType {
    fn to_char(&self) -> char {
        match self {
            MessageType::Text => 't',
            MessageType::Name => 'n'
        }
    }

    fn from_char(c: char) -> Option<MessageType> {
        match c {
            't' => Some(MessageType::Text),
            'n' => Some(MessageType::Name),
            _ => None
        }
    }
}

pub fn encode_message(to_type: MessageType, content: &String) -> String {
    if to_type == MessageType::Text {
        return content.clone();
    }

    return format!("{}{SEPERATOR}{content}", to_type.to_char());
}

pub fn decode_message(from_type: MessageType, message: &String) -> String {
    if from_type == MessageType::Text {
        return message.clone();
    }

    let strip_option = message.strip_prefix(&format!("{}{SEPERATOR}", from_type.to_char()));
    return match strip_option {
        Some(content) => content.to_string(),
        None => message.clone()
    }
}

pub fn get_message_type(message: &String) -> MessageType {
    if message.len() < 2 {
        return MessageType::Text;
    }

    if message.chars().nth(1).unwrap() != SEPERATOR {
        return MessageType::Text;
    }

    let type_option = MessageType::from_char(message.chars().nth(0).unwrap());
    return match type_option {
        Some(message_type) => message_type,
        None => MessageType::Text
    }
}
