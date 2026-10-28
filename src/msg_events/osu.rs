use serenity::model::channel::Message;
use serenity::gateway::client::Context;

pub async fn catch_message(ctx: &Context, message: &Message) {
  let text = message.content.to_lowercase();

  if text.contains("quit w") {
    if let Err(error) = message.reply_ping(&ctx.http, "[pov3.mp4](https://peroroo.s-ul.eu/KfYiQgxK.mp4)").await {
      println!("Error sending message: {error:?}");
    }
  }

  if text.contains("ice angel") {
    if let Err(error) = message.reply_ping(&ctx.http, "[umawhat.mp4](https://peroroo.s-ul.eu/LkZznDwP.mp4)").await {
      println!("Error sending message: {error:?}");
    }
  }

  if text.contains("crystal snow") {
    if let Err(error) = message.reply_ping(&ctx.http, "[crystalsnow.mp4](https://peroroo.s-ul.eu/lcpzKd6i.mp4)").await {
      println!("Error sending message: {error:?}");
    }
  }

  if text.contains("astrosexy") {
    if let Err(error) = message.reply_ping(&ctx.http, "[astrosexy.mp4](https://peroroo.s-ul.eu/6XtHhc93.mp4)").await {
      println!("Error sending message: {error:?}");
    }
  }

  if text == "!!rs" {
    if let Err(error) = message.reply_ping(&ctx.http, "[astrosexy.mp4](https://peroroo.s-ul.eu/6XtHhc93.mp4)").await {
      println!("Error sending message: {error:?}");
    }
  }
}