use serenity::model::channel::Message;
use serenity::gateway::client::Context;
use crate::msg_events::gacha::gacha_util::{ roll_chance };
use crate::utils::msg_utils::builders::{ audio_attachment_builder };

pub async fn catch_message(ctx: &Context, message: &Message) {
  let user_name = &message.author.name;
  let text = message.content.to_lowercase();

  if text == "<o>" && roll_chance(user_name, 1, 25) != 1 {
    if let Err(error) = message.reply_ping(&ctx.http, "https://osu.ppy.sh/beatmapsets/2005387").await {
      println!("Error sending message: {error:?}");
    }
  } else if text == "<o>" {
    if let Err(error) = message.reply_ping(&ctx.http, "<@692795669894791349>").await {
      println!("Error sending message: {error:?}");
    }
  }

  if text.contains("heat abnormal") {
    let roll = roll_chance(user_name, 1, 7);
    let media_url = "./media/iyawa - heat anormal.mp3";
    let media_url_2 = "./media/Lagabnormal.mp3";
    match roll {
      1..=6 => {
        if let Err(error) = message.channel_id.send_message(&ctx.http, audio_attachment_builder(media_url, Some(message)).await).await {
          println!("Error sending message: {error:?}");
        }
      },
      7 => {
        if let Err(error) = message.channel_id.send_message(&ctx.http, audio_attachment_builder(&media_url_2, Some(message)).await).await {
          println!("Error sending message: {error:?}");
        }
      }
      _ => println!("The roll went out of range!! D: (Something went wrong :C)")
    }
    
  }

  if text == "bicol" && roll_chance(user_name, 1, 4) == 1 {
    if let Err(error) = message.reply_ping(&ctx.http, "March 15th, 2025").await {
      println!("Error sending message: {error:?}");
    }
  }

  if text.contains("proroo") {
    match message.author.id.get() {
      242100921629278218 => {
        if let Err(error) = message.reply_ping(&ctx.http, "prohap").await {
          println!("Error sending message: {error:?}");
        }
      }
      527447698245156864 => {
        if let Err(error) = message.reply_ping(&ctx.http, "prosorchou").await {
          println!("Error sending message: {error:?}");
        }
      }
      _ => println!("[FIVE_DIGIT_PH::PROROO]: No matching ids!")
    }
  }

  if text == "<c" && message.author.id == 242100921629278218 {
    if let Err(error) = message.reply_ping(&ctx.http, "https://peroroo.s-ul.eu/SH7BjTd1.jpg").await {
      println!("Error sending message: {error:?}");
    }
  }

  if text == "gimble" {
    if let Err(error) = message.channel_id.say(&ctx.http, "Gimble").await {
      println!("Error sending message: {error:?}");
    }
  }
}