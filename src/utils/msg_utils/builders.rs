use std::path::Path;
use serenity::all::{CreateAttachment, Message};
use serenity::builder::{CreateMessage};

pub async fn audio_attachment_builder<'a>(attachment_url: &'a str, ref_msg: Option<&'_ Message>) -> CreateMessage<'a> {
  let audio_attachment_url = Path::new(attachment_url);
  let audio_attachment = CreateAttachment::path(audio_attachment_url).unwrap();
  let message_builder = CreateMessage::new().add_file(audio_attachment);

  return match ref_msg {
    Some(ref_msg) => message_builder.reference_message(ref_msg),
    None => message_builder
  };
}