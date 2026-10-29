use std::sync::Arc;
use serenity::builder::{CreateCommand, CreateCommandOption, CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::model::application::{CommandOptionType, CommandInteraction};
use serenity::gateway::client::Context;

pub async fn get_command_info<'a>() -> CreateCommand<'a> {
  let command = CreateCommand::new("team_sheet_search")
    .description("Shortcut for looking up team sheet templates for tournaments !! (RussianVaxei Template)")
    .add_option(
      CreateCommandOption::new(CommandOptionType::Integer, "team_size", "how many team members do you have? :D (1-16)")
        .min_int_value(1)
        .max_int_value(16)
        .required(true),
  );

  return command;
}

pub async fn catch_interaction (ctx: &Context, command: &CommandInteraction) { 
  let template_sheet_links: Arc<[&str]> = Arc::new([
    "https://docs.google.com/spreadsheets/d/1wyTPXO713ImIdlVDbs3fzBdJ86rDuhUBMkuRbBX82_U/edit",
    "https://docs.google.com/spreadsheets/d/1zhNy3-f81ifMlHYR2eLXnlqy8LKww9p65L0tIiz3OsU/edit",
    "https://docs.google.com/spreadsheets/d/1zFIMfZueJjNUsRgDIHMvD-z-zfbdchNjC7A3N5CNxdk/edit",
    "https://docs.google.com/spreadsheets/d/1h2zR2Hz8EqgFS-hE9f4ekpKSDa3_kSLwv0R_XjSqhEM/edit",
    "https://docs.google.com/spreadsheets/d/1oZeq_S9VYOy9Epi87eYRUnR2TBIvnSwzdORQcm-gx2U/edit",
    "https://docs.google.com/spreadsheets/d/13sWFkAwe23ROcR-Hf2thnojZM_E2tpet-T9MceRItvY/edit",
    "https://docs.google.com/spreadsheets/d/1aENHdPtfWqCdhoRdRku3WMKN0pUDr5N_I3UJn7rMYA0/edit",
    "https://docs.google.com/spreadsheets/d/1rD2lDJTIShDMmqLD2PcMQwwdjlL6d7wmabV9wjW4jqU/edit",
    "https://docs.google.com/spreadsheets/d/1HQk-nXtJTYsY3Z4jmmC0m87T4IvMOG-pgzU5jZqr7NA/edit",
    "https://docs.google.com/spreadsheets/d/1BAklKaHB7_sz3rYp-yUgPSGto3ftm1_TW8yi-nKseXs/edit",
    "https://docs.google.com/spreadsheets/d/1YI_sXekGjMhO6xLLoqY99-ThRfPg90PpluO5_mPeyKY/edit",
    "https://docs.google.com/spreadsheets/d/1rzKcmeKiCtVwt_LSzAami9MQl9dFzWFPmNGIn3CvZNU/edit",
    "https://docs.google.com/spreadsheets/d/1lXRBBC7SP_-0eMgGBdZ12xNxXJ1rfPaUFcmGncjwzhw/edit",
    "https://docs.google.com/spreadsheets/d/1bNSXn7gk4C3xHGpQxe8ruu3UUvcLMngyGtFQzveUkoo/edit",
    "https://docs.google.com/spreadsheets/d/19-aUqSUIItzgo7w7QnXqBaPQjnZyzjT0_pOvfV1CelU/edit",
    "https://docs.google.com/spreadsheets/d/1an5Kt77u1FffBFLkgonWy6pNsWaS1TqXG2MFhbgooTY/edit"
  ]);
  match command.data.options.iter().find(|opt| opt.name == "team_size").unwrap().value.as_i64().unwrap() { // TODO: Fix bad syntax plz
    1 => {
      let builder = CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new().content("Here you go !! \n".to_owned() + template_sheet_links[0])
      );
      if let Err(why) = command.create_response(&ctx.http, builder).await {
        eprintln!("RIP I think the bot died - peroroo: {why:?}")
      }
    }

    2 => {
      let builder = CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new().content("Here you go !! \n".to_owned() + template_sheet_links[1])
      );
      if let Err(why) = command.create_response(&ctx.http, builder).await {
        eprintln!("RIP I think the bot died - peroroo: {why:?}")
      }
    }

    3 => {
      let builder = CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new().content("Here you go !! \n".to_owned() + template_sheet_links[2])
      );
      if let Err(why) = command.create_response(&ctx.http, builder).await {
        eprintln!("RIP I think the bot died - peroroo: {why:?}")
      }
    }

    4 => {
      let builder = CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new().content("Here you go !! \n".to_owned() + template_sheet_links[3])
      );
      if let Err(why) = command.create_response(&ctx.http, builder).await {
        eprintln!("RIP I think the bot died - peroroo: {why:?}")
      }
    }

    5 => {
      let builder = CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new().content("Here you go !! \n".to_owned() + template_sheet_links[4])
      );
      if let Err(why) = command.create_response(&ctx.http, builder).await {
        eprintln!("RIP I think the bot died - peroroo: {why:?}")
      }
    }

    6 => {
      let builder = CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new().content("Here you go !! \n".to_owned() + template_sheet_links[5])
      );
      if let Err(why) = command.create_response(&ctx.http, builder).await {
        eprintln!("RIP I think the bot died - peroroo: {why:?}")
      }
    }

    7 => {
      let builder = CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new().content("Here you go !! \n".to_owned() + template_sheet_links[6])
      );
      if let Err(why) = command.create_response(&ctx.http, builder).await {
        eprintln!("RIP I think the bot died - peroroo: {why:?}")
      }
    }

    8 => {
      let builder = CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new().content("Here you go !! \n".to_owned() + template_sheet_links[7])
      );
      if let Err(why) = command.create_response(&ctx.http, builder).await {
        eprintln!("RIP I think the bot died - peroroo: {why:?}")
      }
    }

    9 => {
      let builder = CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new().content("Here you go !! \n".to_owned() + template_sheet_links[8])
      );
      if let Err(why) = command.create_response(&ctx.http, builder).await {
        eprintln!("RIP I think the bot died - peroroo: {why:?}")
      }
    }

    10 => {
      let builder = CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new().content("Here you go !! \n".to_owned() + template_sheet_links[9])
      );
      if let Err(why) = command.create_response(&ctx.http, builder).await {
        eprintln!("RIP I think the bot died - peroroo: {why:?}")
      }
    }

    11 => {
      let builder = CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new().content("Here you go !! \n".to_owned() + template_sheet_links[10])
      );
      if let Err(why) = command.create_response(&ctx.http, builder).await {
        eprintln!("RIP I think the bot died - peroroo: {why:?}")
      }
    }

    12 => {
      let builder = CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new().content("Here you go !! \n".to_owned() + template_sheet_links[11])
      );
      if let Err(why) = command.create_response(&ctx.http, builder).await {
        eprintln!("RIP I think the bot died - peroroo: {why:?}")
      }
    }

    13 => {
      let builder = CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new().content("Here you go !! \n".to_owned() + template_sheet_links[12])
      );
      if let Err(why) = command.create_response(&ctx.http, builder).await {
        eprintln!("RIP I think the bot died - peroroo: {why:?}")
      }
    }

    14 => {
      let builder = CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new().content("Here you go !! \n".to_owned() + template_sheet_links[13])
      );
      if let Err(why) = command.create_response(&ctx.http, builder).await {
        eprintln!("RIP I think the bot died - peroroo: {why:?}")
      }
    }

    15 => {
      let builder = CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new().content("Here you go !! \n".to_owned() + template_sheet_links[14])
      );
      if let Err(why) = command.create_response(&ctx.http, builder).await {
        eprintln!("RIP I think the bot died - peroroo: {why:?}")
      }
    }

    16 => {
      let builder = CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new().content("Here you go !! \n".to_owned() + template_sheet_links[15])
      );
      if let Err(why) = command.create_response(&ctx.http, builder).await {
        eprintln!("RIP I think the bot died - peroroo: {why:?}")
      }
    }

    _ => {
      let builder = CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new().content("An error occured!! D:")
      );
      if let Err(why) = command.create_response(&ctx.http, builder).await {
        eprintln!("RIP I think the bot died - peroroo: {why:?}")
      }
    }
  }
}