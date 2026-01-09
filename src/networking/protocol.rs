pub const PORT: u16 = 59744;

const SEPERATOR: char = '+';

#[derive(Copy, Clone)]
enum Position {
    Type = 0,
    Seperator = 1
}

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

    let seperator_option = message.chars().nth(Position::Seperator as usize);
    let seperator_position = match seperator_option {
        Some(character) => character,
        None => ' '
    };
    if seperator_position != SEPERATOR {
        return MessageType::Text;
    }


    let type_option = message.chars().nth(Position::Type as usize);
    let type_position = match type_option {
        Some(character) => character,
        None => ' '
    };
    let message_type_option = MessageType::from_char(type_position);
    return match message_type_option {
        Some(message_type) => message_type,
        None => MessageType::Text
    }
}
