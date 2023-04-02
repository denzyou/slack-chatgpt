use slack_flows::{listen_to_channel, send_message_to_channel};
use openai_flows::{chat_completion, ChatOptions};
use std::env;
use dotenv::dotenv;

#[no_mangle]
pub fn run() {
    listen_to_channel("Callaoers", "general", |sm| {
        send_message_to_channel("Callaoers", "general", format!("Hello, {}", sm.text));
    });
}


