use rand::RngExt;
use serenity::model::channel::Message;
use serenity::gateway::client::Context;

pub fn roll_chance_chiyono(user: &str) -> &str {
  let mut rng = rand::rng();
  let rolled_number: i8 = rng.random_range(1..=7);

  println!("[GACHA::ROLL_CHANCE]: {} rolled a {} !!", user, rolled_number);
  
  let rarity = match rolled_number {
    1..=5 => "[chiyono.mp4](https://peroroo.s-ul.eu/KGr6UeDW.mp4)",
    6 => "[chiyono_0.mp4](https://peroroo.s-ul.eu/vBTweMm9.mp4)",
    7 => "[chiyono_1.mp4](https://peroroo.s-ul.eu/VRMvKWry.mp4)",
    _ => "The roll went out of range!! D: (Something went wrong :C)"
  };

  return rarity;
}

pub async fn catch_message(ctx: &Context, message: &Message) {
  let user_name= &message.author.name;

  if message.content.to_lowercase() == "chiyono" {
    let result = roll_chance_chiyono(user_name);

    println!("[GACHA::CATCH_MESSAGE]: \"Chiyono\" Gacha triggered !!");

    match message.reply_ping(&ctx.http, result).await {
        Ok(_) => println!("[GACHA::CATCH_MESSAGE]: Message sent!!"),
        Err(e) => println!("[GACHA::CATCH_MESSAGE]: Error sending message: {e:?}"),
    }
  }
}