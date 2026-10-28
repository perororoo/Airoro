use serenity::all::ChannelAction::Create;
use serenity::model::channel::{ Message, MessageFlags };
use serenity::gateway::client::Context;
use serenity::builder::{
    CreateContainer, 
    CreateContainerComponent, 
    CreateMessage, 
    CreateSection, 
    CreateSectionComponent, 
    CreateSectionAccessory, 
    CreateThumbnail,
    CreateUnfurledMediaItem,
    CreateTextDisplay,
    CreateMediaGallery,
    CreateMediaGalleryItem,
    CreateSeparator
};

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
    let container_components = vec![
      CreateContainerComponent::Section(
        CreateSection::new(vec![CreateSectionComponent::TextDisplay(CreateTextDisplay::new(
          "🇵🇭   **peroroo: 10,046pp (#7,670 PH167)**\n## [**Wolpis Kater - Oxalis [Wizllia] [8.61★]**](https://osu.ppy.sh/beatmapsets/1347859#osu/2791233)\n**<:A_:1411964123369050132> +NFCL  2,360,810  94.98% <t:1793003520:R>**\n__229x/1999x__ • __190.21pp__ • 38❌ `{1320/49/0/38}`\n"
        ))], 
        CreateSectionAccessory::Thumbnail(
          CreateThumbnail::new(CreateUnfurledMediaItem::new("https://a.ppy.sh/10053373?1792664583.png")))
        )
      ),
      CreateContainerComponent::MediaGallery(
        CreateMediaGallery::new(
          vec![CreateMediaGalleryItem::new(CreateUnfurledMediaItem::new("https://assets.ppy.sh/beatmaps/1347859/covers/cover.jpg"))]
        )
      ),
      CreateContainerComponent::Separator(CreateSeparator::new()),
      CreateContainerComponent::TextDisplay(
        CreateTextDisplay::new(
          "-#  CS: 3.5 AR: 10 OD: 9.5 HP: 5 • 04:25 • 252BPM \n-# Mapset by [Tocorn](https://osu.ppy.sh/users/9564072) • Ranked `2021-06-04`"
        )
      )

    ];

    let container = CreateContainer::new(container_components)
      // Optional: Set a left-side accent color bar on the container
      .accent_color(0xFFFFFF); // Blurple color

    let builder = CreateMessage::new()
            // Important: Components V2 requires the IS_COMPONENTS_V2 message flag (1 << 15)
            .flags(MessageFlags::IS_COMPONENTS_V2)
            .components(vec![serenity::builder::CreateComponent::Container(container)]);

    if let Err(why) = message.channel_id.send_message(&ctx.http, builder).await {
        eprintln!("Failed to send V2 container message: {why:?}");
    }
  }
}