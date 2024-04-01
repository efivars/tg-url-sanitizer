use teloxide::prelude::*;

use url_sanitizer::{
  sanitizer::{UrlSanitizer, YouTubeUrlSanitizer},
  to_url,
};

pub struct UrlSanitizerBot {
  token: String,
}
impl UrlSanitizerBot {
  pub fn new(token: String) -> UrlSanitizerBot {
    UrlSanitizerBot { token }
  }

  fn process_message(bot: Bot, message: Message) {
    // FIXME: do not create sanitizer each time
    let sanitizer = YouTubeUrlSanitizer::new();

    match message.text() {
      Some(text) => {
        let reply_text: String = match to_url(text) {
          Ok(url) => match sanitizer.sanitize(&url) {
            Ok(link) => link,
            Err(e) => match e {
              url_sanitizer::sanitizer::SanitizeError::NoDomain => {
                String::from("Error: specify domain!")
              }
              url_sanitizer::sanitizer::SanitizeError::UnacceptableDomain => {
                String::from("Error: Unsupported domain")
              }
            },
          },
          Err(e) => format!("ParseError: {}", e.to_string()),
        };

        let _ = bot.send_message(message.chat.id, reply_text);
      }
      None => {
        let _ = bot.send_dice(message.chat.id);
      }
    }
  }

  pub async fn run_forever(&self) {
    let bot = Bot::new(&self.token);

    println!("Bot started in long-polling mode!");

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
      Ok(UrlSanitizerBot::process_message(bot, msg))
    })
    .await
  }
}
