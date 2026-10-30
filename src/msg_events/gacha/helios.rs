use serenity::model::channel::Message;
use serenity::gateway::client::Context;
use super::gacha_util::*;
use crate::{airoro_log, utils::msg_utils::builders::audio_attachment_builder};


fn get_result_helios(roll: &i8) -> String {
  let rolled_number = *roll as usize;
  let media_url_array:[&str; 9] = [
    "[HeliosWeiii.mov](https://peroroo.s-ul.eu/gwV8jZJa.mov)",
    "[wei_1.mp4](https://peroroo.s-ul.eu/q8QHouBU.mp4)",
    "[wei_2.mp4](https://peroroo.s-ul.eu/Y31GKM4m.mp4)",
    "[wei_3.mp4](https://peroroo.s-ul.eu/TxutZ3Ut.mp4)",
    "./media/helios_1.mp3",
    "./media/helios_2.mp3",
    "./media/wei.mp3",
    "./media/wei_2.mp3",
    "./media/wei_3.mp3"
    ];

  return media_url_array[rolled_number].to_string();
}

fn get_result_weii(roll: i8) -> String {
  let result = match roll {
    1..=4 => "[HeliosWeiii.mov](https://peroroo.s-ul.eu/gwV8jZJa.mov)",
    5 => "[dualweii.mp4](https://peroroo.s-ul.eu/7nZMdO1t.mp4)",
    _ => "The roll went out of range!! D: (Something went wrong :C)"
  };

  return result.to_string();
}

pub async fn catch_message(ctx: &Context, message: &Message) {
  let user_name = &message.author.name;
  if message.content.to_lowercase() == "helios" {
    let rolled_number = roll_chance(user_name, 0, 8);
    let result = get_result_helios(&rolled_number);

    airoro_log!("[GACHA::CATCH_MESSAGE]: \"Helios\" Gacha triggered !!");

    match rolled_number {
      0..=3 => {
        match message.reply_ping(&ctx.http, result).await {
          Ok(_) => airoro_log!("[GACHA::CATCH_MESSAGE]: Message sent!!"),
          Err(why) => airoro_log!("[GACHA::CATCH_MESSAGE]: Error sending message: {why:?}"),
        }
      }
      4..=8 => {
        match message.channel_id.send_message(&ctx.http, audio_attachment_builder(&result, Some(message)).await).await {
          Ok(_) => airoro_log!("[GACHA::CATCH_MESSAGE]: Message sent!!"),
          Err(why) => airoro_log!("[GACHA::CATCH_MESSAGE]: Error sending message: {why:?}"),
        }
      }
      _ => airoro_log!("The roll went out of range!! D: (Something went wrong :C)")
    }
  } 
  
  if message.content.to_lowercase().contains("wei") == true {
    let rolled_number = roll_chance(user_name, 1, 5);
    let result = get_result_weii(rolled_number);

    airoro_log!("[GACHA::CATCH_MESSAGE]: \"Helios.weii\" Gacha triggered !!");
      match message.reply_ping(&ctx.http, result).await {
        Ok(_) => airoro_log!("[GACHA::CATCH_MESSAGE]: Message sent!!"),
        Err(why) => airoro_log!("[GACHA::CATCH_MESSAGE]: Error sending message: {why:?}"),
      }
    }
}