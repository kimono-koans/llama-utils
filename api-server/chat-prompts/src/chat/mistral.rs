use super::BuildChatPrompt;
use crate::error::Result;
use endpoints::chat::{ChatCompletionRequestMessage, ChatCompletionRole};

/// Generate prompts for the `Mistral-instruct-v0.1` model.
#[derive(Debug, Default, Clone)]
pub struct MistralInstructPrompt;
impl MistralInstructPrompt {
    /// Create a user prompt from a chat completion request message.
    fn append_user_message(&self, content: impl AsRef<str>) -> String {
        format!(
            "<s>[INST] {user_message} [/INST]",
            user_message = content.as_ref().trim(),
        )
    }
}
impl BuildChatPrompt for MistralInstructPrompt {
    fn build(&self, messages: &mut Vec<ChatCompletionRequestMessage>) -> Result<String> {
        while !messages.is_empty() && messages[0].role != ChatCompletionRole::User {
            messages.remove(0);
        }

        if messages.is_empty() {
            return Ok(String::new());
        }

        let message = messages.remove(0);
        let prompt = self.append_user_message(message.content.as_str());

        messages.clear();

        // println!("*** [prompt begin] ***");
        // println!("{}", &prompt);
        // println!("*** [prompt end] ***");

        Ok(prompt)
    }
}

/// Generate prompts for the amazon `MistralLite-7B` model.
#[derive(Debug, Default, Clone)]
pub struct MistralLitePrompt;
impl MistralLitePrompt {
    /// Create a user prompt from a chat completion request message.
    fn append_user_message(&self, content: impl AsRef<str>) -> String {
        format!(
            "<|prompter|>{user_message}</s><|assistant|>",
            user_message = content.as_ref().trim(),
        )
    }
}
impl BuildChatPrompt for MistralLitePrompt {
    fn build(&self, messages: &mut Vec<ChatCompletionRequestMessage>) -> Result<String> {
        while !messages.is_empty() && messages[0].role != ChatCompletionRole::User {
            messages.remove(0);
        }

        if messages.is_empty() {
            return Ok(String::new());
        }

        let message = messages.remove(0);
        let prompt = self.append_user_message(message.content.as_str());

        messages.clear();

        // println!("*** [prompt begin] ***");
        // println!("{}", &prompt);
        // println!("*** [prompt end] ***");

        Ok(prompt)
    }
}

/// Generate prompts for the amazon `MistralLite-7B` model.
#[derive(Debug, Default, Clone)]
pub struct OpenChatPrompt;
impl OpenChatPrompt {
    /// Create a user prompt from a chat completion request message.
    fn append_user_message(&self, content: impl AsRef<str>) -> String {
        format!(
            "GPT4 User: {user_message}<|end_of_turn|>GPT4 Assistant:",
            user_message = content.as_ref().trim(),
        )
    }
}
impl BuildChatPrompt for OpenChatPrompt {
    fn build(&self, messages: &mut Vec<ChatCompletionRequestMessage>) -> Result<String> {
        while !messages.is_empty() && messages[0].role != ChatCompletionRole::User {
            messages.remove(0);
        }

        if messages.is_empty() {
            return Ok(String::new());
        }

        let message = messages.remove(0);
        let prompt = self.append_user_message(message.content.as_str());

        messages.clear();

        // println!("*** [prompt begin] ***");
        // println!("{}", &prompt);
        // println!("*** [prompt end] ***");

        Ok(prompt)
    }
}
