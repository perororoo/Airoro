use rand::seq::IndexedRandom;
use serenity::model::channel::Message;
use serenity::gateway::client::Context;
use super::gacha_util::*;
use crate::utils::msg_utils::builders::{ audio_attachment_builder };

fn mambo_r<'a>() -> &'a str {
  let mut rng = rand::rng();
  let media_url_array:[&str; 4] = [
    "https://tenor.com/view/mambo-ume-usume-chibi-dance-anime-gif-10304596603435351987",
    "https://tenor.com/view/mambo-machikane-tannhauser-hachimi-uma-gif-7984069466482758216", 
    "https://tenor.com/view/matikanetannhauser-mambo-brick-brick-throw-umamusume-gif-14487761251708067934", 
    "https://tenor.com/view/pretty-derby-matikane-tannhauser-anime-girl-gif-13234024330359809043"];

  return media_url_array.choose(&mut rng).unwrap();
}

fn mambo_sr<'a>() -> &'a str {
  let mut rng = rand::rng();
  let media_url_array:[&str; 2] = [
    "./media/mambo.mp3",
    "./media/wow.mp3"
  ];

  return media_url_array.choose(&mut rng).unwrap();
}

fn mambo_ssr<'a>() -> &'a str {
  let mut rng = rand::rng();
  let media_url_array:[&str; 4] = [
    "[mambo_spin.mp4](https://peroroo.s-ul.eu/0wt0LmmC.mp4)",
    "[ling gang guli guli.mp4](https://peroroo.s-ul.eu/VZZ5Dd1R.mp4)", 
    "[littlebitofmambo.mp4](https://peroroo.s-ul.eu/aU1hGoNv.mp4)", 
    "[Wang Shi Zhi Mam bo Hui Wei.mp4](https://peroroo.s-ul.eu/k9Xo8FIb.mp4)"];

  return media_url_array.choose(&mut rng).unwrap();
}

fn get_result(rarity: &GachaResult) -> &str {
  match rarity {
    GachaResult::R => return mambo_r(),
    GachaResult::SR => return mambo_sr(),
    GachaResult::SSR => return mambo_ssr(),
    GachaResult::DEFAULT => return "No function called D: (Something went wrong !!)", 
  }
}

pub async fn catch_message(ctx: &Context, message: &Message) {
  let user_name = &message.author.name;
  if message.content.to_lowercase() == "mambo" {
    let rolled_number = roll_chance(user_name, 1, 100);
    let rarity = get_rarity(&rolled_number);
    let result = get_result(&rarity);

    println!("[GACHA::CATCH_MESSAGE]: \"Mambo\" Gacha triggered !!");
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