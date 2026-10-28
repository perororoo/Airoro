use rand::seq::IndexedRandom;
use serenity::model::channel::Message;
use serenity::gateway::client::Context;
use super::gacha_util::*;
use crate::utils::msg_utils::builders::{ audio_attachment_builder };

fn hachimi_r() -> String {
  let mut rng = rand::rng();
  let media_url_array:[&str; 4] = [
    "https://tenor.com/view/meme-horse-you-have-alerted-the-horse-pretty-derby-uma-musume-gif-15063465391514740847",
    "https://tenor.com/view/tokai-teio-uma-musume-teio-gif-7905070132435664956", 
    "https://tenor.com/view/hachimi-uma-musume-tokai-teio-hoomfies-oomfie-gif-4843742398632739116", 
    "https://tenor.com/view/uma-musume-tokai-teio-dance-gif-26313975"];

  return media_url_array.choose(&mut rng).unwrap().to_string();
}

fn hachimi_sr() -> String {
  let mut rng = rand::rng();
  let media_url_array:[&str; 2] = [
    "./media/hachimi.mp3",
    "./media/teio teio teio.mp3"
  ];

  return media_url_array.choose(&mut rng).unwrap().to_string();
}

fn hachimi_ssr() -> String {
  let mut rng = rand::rng();
  let media_url_array:[&str; 4] = [
    "[ling gang guli guli 4D Dolby Surround HD audio.mp4](https://peroroo.s-ul.eu/ENLXQwi6.mp4)",
    "[watching_teio.mp4](https://peroroo.s-ul.eu/k9pgN1A6.mp4)", 
    "[hachistellar.mp4](https://peroroo.s-ul.eu/ZrFbnLBX.mp4)", 
    "[I WOKE UP IN A NEW BUGATTI.mp4](https://peroroo.s-ul.eu/aO0pS24X.mp4)"];

  return media_url_array.choose(&mut rng).unwrap().to_string();
}

fn get_result(rarity: &GachaResult) -> String {
  match rarity {
    GachaResult::R => return hachimi_r(),
    GachaResult::SR => return hachimi_sr(),
    GachaResult::SSR => return hachimi_ssr(),
    GachaResult::DEFAULT => return "No function called D: (Something went wrong !!)".to_string(), 
  }
}

pub async fn catch_message(ctx: &Context, message: &Message) {
  let user_name= &message.author.name;
  if message.content.to_lowercase() == "hachimi" {
    let rolled_number = roll_chance(user_name, 1, 100);
    let rarity = get_rarity(&rolled_number);
    let result: String = get_result(&rarity);

    println!("[GACHA::CATCH_MESSAGE]: \"hachimi\" Gacha triggered !!");
    
    if rarity == GachaResult::R || rarity == GachaResult::SSR {
      match message.reply_ping(&ctx.http, result).await {
        Ok(_) => println!("[GACHA::CATCH_MESSAGE]: Message sent!!"),
        Err(e) => println!("[GACHA::CATCH_MESSAGE]: Error sending message: {e:?}"),
      }
    } else {
      match message.channel_id.send_message(&ctx.http, audio_attachment_builder(&result, Some(message)).await).await {
        Ok(_) => println!("[GACHA::CATCH_MESSAGE]: Message sent!!"),
        Err(e) => println!("[GACHA::CATCH_MESSAGE]: Error sending message: {e:?}"),
      }
    }
    
  }
}