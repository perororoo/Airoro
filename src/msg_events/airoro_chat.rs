
use std::any::Any;
use std::ops::Index;

use serenity::model::channel::Message;
use serenity::gateway::client::Context;
use serenity::model::id::UserId;
use crate::msg_events::gacha::gacha_util::{ roll_chance };
use crate::utils::msg_utils::builders::{ audio_attachment_builder };
use async_openai::{
  config::OpenAIConfig,
  traits::RequestOptionsBuilder,
    types::chat::{
      ChatCompletionRequestAssistantMessage, ChatCompletionRequestSystemMessage,
      ChatCompletionRequestUserMessage, CreateChatCompletionRequestArgs,
  },
};
use crate::airoro_log;

pub async fn catch_message(ctx: &Context, message: &Message, openai_client: &async_openai::Client<OpenAIConfig>, tokens_used: &mut u32, chat_memory: &mut Vec<String>, bot_user_id: &UserId) {
  let user_name= &message.author.name.to_string();
  let message_sanitized = message.content.replace(&format!("<@{}> ", bot_user_id.get()), &format!("{}: ", user_name));
  
  if message.mentions_user_id(*bot_user_id) {
    if *tokens_used <= 1000000 {
      airoro_log!("[AIRORO_CHAT]: Chatbot triggered!!");

      chat_memory.push(message_sanitized);

      if chat_memory.len() > 11 {
        chat_memory.remove(1);
        chat_memory.remove(1);
      }

      let mut memory_payload: Vec<async_openai::types::chat::ChatCompletionRequestMessage> = vec![
        ChatCompletionRequestSystemMessage::from(chat_memory[0].as_str()).into()
      ];
      for (index, curr_message) in chat_memory.iter().enumerate() {
        if index != 0 && index % 2 == 0 {
          memory_payload.push(ChatCompletionRequestAssistantMessage::from(curr_message.as_str()).into());
        } else {
          if memory_payload.len() > 11 { // memory up to 10 messages
            memory_payload.remove(1);
            memory_payload.remove(1);
          }
          memory_payload.push(ChatCompletionRequestUserMessage::from(curr_message.as_str()).into());
        }
      }

      let request = CreateChatCompletionRequestArgs::default()
        .max_completion_tokens(512u32)
        .model("gpt-6-luna")
        .messages(memory_payload)
        .build()
        .unwrap();

      let response = openai_client
        .chat()
        .query(&vec![("limit", 10)]).unwrap()
        .create(request)
        .await;

      let response = response.unwrap();
      let response_usage = response.usage.unwrap();

      // println!("\nResponse:\n");
      // for choice in &response.choices {
      //   println!(
      //     "{}: Role: {}  Content: {:?}",
      //     choice.index, choice.message.role, choice.message.content
      //   );
      // }
      *tokens_used += response_usage.total_tokens;

      let airoro_msg = response.choices.first().unwrap().message.content.as_deref().unwrap();

      chat_memory.push(airoro_msg.to_string());

      for (index, curr_message) in chat_memory.iter().enumerate()  {
        if index != 0 && index % 2 == 0 {
          airoro_log!("[AIRORO_CHAT_LOG::ASSISTANT]: {}", curr_message);
        } else if index % 2 != 0 {
          airoro_log!("[AIRORO_CHAT_LOG::USER]: {}", curr_message);
        } else {
          airoro_log!("[AIRORO_CHAT_LOG]: <SYSTEM_MSG>\n--------------------");
        }
      }

      airoro_log!("[AIRORO_CHAT_USAGE]:\nPrompt Tokens Used = {}\nCompletion Tokens Used = {}\nTotal Tokens Used = {}\nTotal Tokens Used (Session) = {}", response_usage.completion_tokens, response_usage.prompt_tokens, response_usage.total_tokens, tokens_used);

      if let Err(error) = message.reply_ping(&ctx.http, airoro_msg).await {
        airoro_log!("Error sending message: {error:?}");
      }
    } else {
      if let Err(error) = message.reply_ping(&ctx.http, "Sorry, Airoro has hit the 1 million token limit ;w;").await {
        airoro_log!("Error sending message: {error:?}");
      }
    }
  }
  
}