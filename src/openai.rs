use anyhow::Result;
use reqwest;
use serde::{Deserialize, Serialize};

use crate::prompt::SYSTEM_PROMPT;

const COMPLETION_ENDPOINT: &str = "https://api.openai.com/v1/chat/completions";

pub async fn call_gpt(token: &str, query: &str) -> Result<String> {
    let request_payload = Chat::new_from_query(query);

    let client = reqwest::Client::new();
    let resp = client
        .post(COMPLETION_ENDPOINT)
        .bearer_auth(token)
        .json(&request_payload)
        .send()
        .await?;

    if resp.status() != 200 {
        println!("Error: {}", resp.status());
        println!("Error: {}", resp.text().await?);
        return Ok("".to_owned());
    }

    let resp_text = resp.text().await?;
    let completion: Completion = serde_json::from_str(&resp_text)?;
    let response_text = completion.choices.first().unwrap().message.content.clone();

    Ok(response_text)
}

#[derive(Serialize, Deserialize)]
struct Chat {
    model: String,
    messages: Vec<Message>,
}

impl Chat {
    fn new_from_query(query: &str) -> Self {
        Chat {
            model: "gpt-4".to_owned(),
            messages: vec![
                Message {
                    role: Role::System,
                    content: SYSTEM_PROMPT.to_owned(),
                },
                Message {
                    role: Role::User,
                    content: query.to_owned(),
                },
            ],
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Completion {
    model: String,
    choices: Vec<Choice>,
}

#[derive(Serialize, Deserialize)]
struct Choice {
    message: Message,
}

#[derive(Serialize, Deserialize)]
struct Message {
    role: Role,
    content: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
enum Role {
    #[serde(rename = "system")]
    System,
    #[serde(rename = "user")]
    User,
    #[serde(rename = "assistant")]
    Assistant,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chat_new_from_query() {
        let query = "hi, how are you doing?";
        let chat = Chat::new_from_query(query);
        assert_eq!(chat.model, "gpt-4");
        assert_eq!(chat.messages.len(), 2);
        assert_eq!(chat.messages[0].role, Role::System);
        assert_eq!(chat.messages[0].content, SYSTEM_PROMPT);
        assert_eq!(chat.messages[1].role, Role::User);
        assert_eq!(chat.messages[1].content, query);
    }
}
