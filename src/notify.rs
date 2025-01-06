use crate::config::Config;
use serde_json::json;
use crate::ccrllive::{CcrlLivePlayer, CcrlLiveRoom};

pub struct NotifyContent {
    pub engine: CcrlLivePlayer,
    pub opponent: CcrlLivePlayer,
    pub color: Color,
    pub room: CcrlLiveRoom,
}

pub enum Color {
    White,
    Black,
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", match self {
            Color::White => "White",
            Color::Black => "Black",
        })
    }
}

pub fn notify(config: &Config, content: NotifyContent) {
    let client = reqwest::blocking::Client::new();

    let title = format!("{} started a game playing as {} vs. {}", content.engine, content.color, content.opponent);
    let description = format!("Watch live: {}", content.room.url());

    let body = json!({
        "username": "ccrl-live-notifier",
        "embeds": [{
            "title": title,
            "description": description,
        }]
    });

    client.post(&config.webhook_url)
        .json(&body)
        .send()
        .expect("Unable to send webhook")
        .error_for_status()
        .expect("Unable to send webhook");
}


