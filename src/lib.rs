#![deny(soft_unstable)]
#![feature(test)]
pub enum Message {
    Sent { is_manual: bool },
    Received { is_flagged: bool },
}

pub fn history() -> Vec<Message> {
    vec![
        Message::Sent { is_manual: false },
        Message::Sent { is_manual: false },
        Message::Received { is_flagged: false },
        Message::Sent { is_manual: true },
        Message::Sent { is_manual: false },
        Message::Received { is_flagged: false },
        Message::Received { is_flagged: false },
        Message::Sent { is_manual: false },
        Message::Received { is_flagged: false },
        Message::Sent { is_manual: false },
        Message::Received { is_flagged: false },
    ]
}
pub fn history_happy() -> Vec<Message> {
    vec![
        Message::Sent { is_manual: false },
        Message::Sent { is_manual: false },
        Message::Received { is_flagged: false },
        Message::Sent { is_manual: true },
        Message::Sent { is_manual: false },
        Message::Received { is_flagged: false },
        Message::Received { is_flagged: false },
        Message::Sent { is_manual: false },
        Message::Received { is_flagged: false },
        Message::Sent { is_manual: false },
        Message::Received { is_flagged: false },
        Message::Sent { is_manual: true },
    ]
}
pub fn history_happy2() -> Vec<Message> {
    vec![
        Message::Sent { is_manual: false },
        Message::Sent { is_manual: false },
        Message::Received { is_flagged: false },
        Message::Sent { is_manual: true },
        Message::Sent { is_manual: false },
        Message::Received { is_flagged: false },
        Message::Received { is_flagged: false },
        Message::Sent { is_manual: false },
        Message::Received { is_flagged: false },
        Message::Sent { is_manual: false },
        Message::Received { is_flagged: true },
    ]
}
pub fn history_happy3() -> Vec<Message> {
    vec![
        Message::Sent { is_manual: false },
        Message::Sent { is_manual: false },
        Message::Sent { is_manual: false },
        Message::Sent { is_manual: false },
    ]
}
