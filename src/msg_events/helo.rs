
use serenity::model::channel::Message;
use serenity::gateway::client::Context;
use crate::msg_events::gacha::gacha_util::{ roll_chance };
use crate::utils::msg_utils::builders::{ audio_attachment_builder };
use crate::airoro_log;

pub async fn catch_message(ctx: &Context, message: &Message) {
  let user_name: &String = &message.author.name.to_string();
  let text = message.content.to_lowercase();

  match text.as_str(){
    "america ya" => {
      if let Err(error) = message.reply_ping(&ctx.http, "HALLO :D").await {
        airoro_log!("Error sending message: {error:?}");
      }

      if let Err(error) = message.channel_id.say(&ctx.http, "https://tenor.com/view/america-ya-gif-15374592095658975433").await {
        airoro_log!("Error sending message: {error:?}");
      }
    }

    "hallo" => {
      if let Err(error) = message.reply_ping(&ctx.http, "hallo!! :D").await {
        airoro_log!("Error sending message: {error:?}");
      }
    }

    "o//" => { 
      if roll_chance(user_name, 1, 20) != 1 {
        if let Err(error) = message.reply_ping(&ctx.http, "o//").await {
            airoro_log!("Error sending message: {error:?}");
        }
      } else {
        if let Err(error) = message.reply_ping(&ctx.http, r"\\\o").await {
          airoro_log!("Error sending message: {error:?}");
        }
      }
    }

    _ => {}
  }

  if text.contains("<@1406309729860321373>") { // AiRoro's ID
    if let Err(error) = message.reply_ping(&ctx.http, "hi :D").await {
      airoro_log!("Error sending message: {error:?}");
    }
  }

  if text.contains("i suck") {
    if let Err(error) = message.reply_ping(&ctx.http, "no you dont >:C").await {
      airoro_log!("Error sending message: {error:?}");
    }
  }

  if text.contains("what the helly") {
    if let Err(error) = message.reply_ping(&ctx.http, "https://peroroo.s-ul.eu/LLC0dQQv.png").await {
      airoro_log!("Error sending message: {error:?}");
    }
  }

  if text.contains("goodbye") || text.contains("bye") || text.contains("byebye") || text.contains("i go now") {
    if roll_chance(user_name, 1, 20) != 1 {
        if let Err(error) = message.reply_ping(&ctx.http, "o7").await {
            airoro_log!("Error sending message: {error:?}");
        }
      } else {
        if let Err(error) = message.reply_ping(&ctx.http, "byebye!! o//").await {
          airoro_log!("Error sending message: {error:?}");
        }
      }
  }

  if text.starts_with("wow") || text.starts_with("woah") {
    let media_url = "./media/wow.mp3";
    if let Err(error) = message.channel_id.send_message(&ctx.http, audio_attachment_builder(media_url, Some(message)).await).await {
      airoro_log!("Error sending message: {error:?}");
    }
  }

  if text.starts_with("oh yee") || text.contains("oh yee <:woah:1411192764686794872>") {
    let media_url = "./media/oh yee.mp3";
    if let Err(error) = message.channel_id.send_message(&ctx.http, audio_attachment_builder(&media_url, Some(message)).await).await {
      airoro_log!("Error sending message: {error:?}");
    }
  }
}