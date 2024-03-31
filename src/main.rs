use std::env;

use dotenv::dotenv;
use teloxide::prelude::*;
use tokio::task;

use url_sanitizer::is_valid_url;

const TOKEN_ENV_VAR: &'static str = "TG_URL_SANITIZER_TOKEN";
const TOKEN_ARG_VAR: &'static str = "--token";

async fn run_tg_bot(token: String) {
  let bot = Bot::new(token);

  println!("Bot started in long-polling mode!");

  teloxide::repl(bot, |bot: Bot, msg: Message| async move {
    match msg.text() {
      Some(text) => {
        let reply_text = match is_valid_url(text) {
          true => "Valid URL",
          false => "Invalid URL!",
        };

        bot.send_message(msg.chat.id, reply_text).await?;
      }
      None => {
        // TODO: answer "I accept text only?"
        bot.send_dice(msg.chat.id).await?;
      }
    }

    Ok(())
  })
  .await;
}

fn try_get_env_token() -> Option<String> {
  match dotenv().ok() {
    None => {
      println!("Info: No .env file provided");
    }
    Some(_) => {
      println!("Info: .env file loaded");
    }
  }

  match env::var(TOKEN_ENV_VAR) {
    Ok(token) => Some(token),
    Err(_) => None,
  }
}

fn try_get_arg_token() -> Option<String> {
  let mut stepped_into = false;

  for arg in env::args() {
    if stepped_into {
      return Some(arg);
    }

    if arg.as_str().eq(TOKEN_ARG_VAR) {
      stepped_into = true;
    }
  }

  None
}

fn try_get_token() -> Option<String> {
  try_get_arg_token().or_else(try_get_env_token)
}

fn print_usage() {
  println!(r#"
Usage: tg-url-sanitizer [--token TOKEN]

Or make sure you have .env file provided with {} specified
  "#, TOKEN_ENV_VAR);
}

#[tokio::main]
async fn main() {
  match try_get_token() {
    Some(token) => match task::spawn(run_tg_bot(token)).await {
      Ok(_) => {
        println!("Program stopped. Have a nice day!");
      }
      Err(e) => {
        panic!("JoinError: {}", e.to_string());
      }
    },
    None => {
      println!("Cannot find token!");
      print_usage();
    }
  }
}
