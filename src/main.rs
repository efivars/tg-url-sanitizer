mod bot;
mod token;

use bot::UrlSanitizerBot;
use token::{try_get_token, TOKEN_ENV_VAR};

fn print_usage() {
  println!(
    r#"
Usage: tg-url-sanitizer [--token TOKEN]

Or make sure you have .env file provided with {} specified
  "#,
    TOKEN_ENV_VAR
  );
}

#[tokio::main]
async fn main() {
  let token = match try_get_token() {
    Some(token) => token,
    None => {
      print_usage();
      panic!("Cannot find token!");
    }
  };

  let bot = UrlSanitizerBot::new(token);

  bot.run_forever().await;
}
