use std::fmt;
use serde_derive::{Deserialize, Serialize};

use crate::objects::Player;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Chat {
    messages: Vec<Message>,
}

impl Chat {
    pub fn new() -> Self {
        Chat {
            messages: Vec::new(),
        }
    }

    pub fn add_new_message(&mut self, message: Message) {
        self.messages.push(message);
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    sender: Player,
    message: String,
}

impl Message {
    pub fn new(sender: Player, message: &str) -> Self {
        Message { sender, message: message.to_string() }
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.sender.name, self.message)
    }
}
