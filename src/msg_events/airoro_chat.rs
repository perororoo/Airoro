use async_openai::error::OpenAIError;
use serde_json::json;
use serenity::model::channel::Message;
use serenity::gateway::client::Context;
use serenity::model::id::UserId;
use crate::utils::init_utils::event_handler::ChatTurn;
use crate::msg_events::gacha::gacha_util::{ roll_chance };
use crate::utils::msg_utils::builders::{ audio_attachment_builder };
use async_openai::{
  config::OpenAIConfig,
  types::responses::{
    CreateResponse,
    CreateResponseArgs,
    InputParam,
    InputItem,
    Tool,
    WebSearchToolArgs,
  },
  types::chat::ReasoningEffort::Low,
};
use crate::airoro_log;

fn chat_log_to_json(turns: &[ChatTurn]) -> Result<InputParam, Box<dyn std::error::Error>> { // CAUTION: vibecoded function, peroroo is too lazy and have 0 time left because college T^T
    let items: Vec<InputItem> = turns
        .iter()
        .map(|turn| {
            serde_json::from_value(json!({
                "type": "message",
                "role": turn.role,
                "content": turn.content
            }))
        })
        .collect::<Result<_, _>>()?;

    Ok(InputParam::Items(items))
}

fn chat_default(system_msg: &String, chat_memory: &mut Vec<ChatTurn>) ->  Result<CreateResponse, OpenAIError> {
  let request = CreateResponseArgs::default()
    .model("gpt-6-luna")
    .reasoning(Low)
    .instructions(system_msg)
    .input(chat_log_to_json(chat_memory).expect("Cannot turn logs into json :<"))
    .store(false) 
    .max_output_tokens(512u32)
    .tools(vec![Tool::WebSearch(WebSearchToolArgs::default().build().expect("WebSearch failed!! D:"))])
    .build();
  
  return request;
}

pub async fn catch_message(ctx: &Context, message: &Message, openai_client: &async_openai::Client<OpenAIConfig>, tokens_used: &mut u32, chat_memory: &mut Vec<ChatTurn>, bot_user_id: &UserId, total_cost: &mut f64) {
  let user_name= &message.author.name.to_string();
  let mut message_sanitized = message.content.replace(&format!("<@{}> ", bot_user_id.get()), "");
  message_sanitized.insert_str(0, &format!("{}: ", user_name));
  
  if message.mentions_user_id(*bot_user_id) {
    if *tokens_used <= 1000000 {
      airoro_log!("[AIRORO_CHAT]: Chatbot triggered!!\n");

      // push user message to chat history
      chat_memory.push(ChatTurn { // TO-DO: add a handler so that when chatbot fails to respond, it pops this message from the history
        role: "user".to_string(),
        content: message_sanitized,
      });

      if chat_memory.len() > 10 { // Use VecDeque in the future
        chat_memory.remove(0);
        chat_memory.remove(0);
      }

      let system_msg = dotenv::var("AIRORO_SYSTEM_MESSAGE").unwrap();

      let request = chat_default(&system_msg, chat_memory).expect("Request building failed!! D:");

      let response = openai_client.responses().create(request).await.expect("Could not get a response!! TvT");

      let response_usage = response.usage.as_ref().unwrap();

      *tokens_used += response_usage.total_tokens;

      let airoro_msg = response.output_text().unwrap_or_default();

      chat_memory.push(ChatTurn {
        role: "assistant".to_string(),
        content: airoro_msg.clone(),
      });
      airoro_log!("<--------------------Chat Log Start-------------------->");
      for message in chat_memory  {
        if message.role == "assistant" {
          airoro_log!("[AIRORO_CHAT_LOG::ASSISTANT]: {}", message.content);
        }
        else if message.role == "user" {
          airoro_log!("[AIRORO_CHAT_LOG::USER]: {}", message.content);
        }
        else {
          airoro_log!("[AIRORO_CHAT_LOG::UNKNOWN]: <UNKNOWN>");
        }
      }
      airoro_log!("<--------------------Chat Log End-------------------->\n");

      airoro_log!("[AIRORO_CHAT_USAGE]:\nInput Tokens Used = {}\nOutput Tokens Used = {}\nTotal Tokens Used = {}\nTotal Tokens Used (Session) = {}\n", response_usage.input_tokens, response_usage.output_tokens, response_usage.total_tokens, tokens_used);

      airoro_log!("[AIRORO_CHAT_USAGE_DETAILS]:\nReasoning Tokens Used = {}\nCache_write Tokens Used = {}\nCached Input Tokens Used = {}\nTotal Tokens Used (Session) = {}\n", response_usage.output_tokens_details.reasoning_tokens, response_usage.input_tokens_details.cache_write_tokens.unwrap_or_default(), response_usage.input_tokens_details.cached_tokens, tokens_used);

      let input_tokens_spent = response_usage.input_tokens as f32 * 0.0000001;
      let output_tokens_spent = response_usage.output_tokens_details.reasoning_tokens as f32 * 0.0000005;
      let cached_input_tokens_spent = response_usage.input_tokens_details.cached_tokens as f32 * 0.00000001;
      let cached_writes_tokens_spent = response_usage.input_tokens_details.cache_write_tokens.unwrap_or_default() as f32 * 0.000000125;
      // let total_spent_single = input_tokens_spent + output_tokens_spent + cached_input_tokens_spent + cached_writes_tokens_spent;
      let total_spent_single = input_tokens_spent + response_usage.output_tokens as f32 * 0.0000005;
      
      *total_cost += total_spent_single as f64;

      airoro_log!("[AIRORO_CHAT_USAGE_COST]:\nInput = {} USD\nOutput = {} USD\nCache_write Tokens = {} USD\nCached Input Tokens = {}\nTotal Spent = {} USD\nTotal Spent (Session) = {} USD", input_tokens_spent.round(), output_tokens_spent.round(), cached_writes_tokens_spent.round(), cached_input_tokens_spent.round(), total_spent_single, total_cost);

      if airoro_msg == "" {
        if let Err(error) = message.reply_ping(&ctx.http, "...sorry what was it again? I got little confused @_@\n\nCan you make the wording more simple? ;w;").await {
         airoro_log!("Error sending message: {error:?}");
        }
      }
      else {
        if let Err(error) = message.reply_ping(&ctx.http, airoro_msg).await {
          airoro_log!("Error sending message: {error:?}");
        }
      }
    
    } else {
      if let Err(error) = message.reply_ping(&ctx.http, "Sorry, Airoro has hit the 1 million token limit ;w;").await {
        airoro_log!("Error sending message: {error:?}");
      }
    }
  }
  
}