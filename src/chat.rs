use std::fmt;
use crate::objects::Player;

pub struct Chat {
    messages: Vec<Message>,
}

impl Chat {
    pub fn new() -> Self {
        Chat {
            messages: Vec::new(),
        }
    }
}

impl fmt::Display for Chat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for message in &self.messages {
            writeln!(f, "{}", message)?;
        }
        write!(f, "")
    }
}

pub struct Message {
    sender: Player,
    message: String,
}

impl Message {
    pub fn new(sender: Player, message: String) -> Self {
        Message { sender, message }
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.sender.name, self.message)
    }
}
