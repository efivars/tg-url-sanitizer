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

  pub async fn run_forever(&self) {
    let bot = Bot::new(&self.token);
    println!("Bot started in long-polling mode!");

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
      // FIXME: do not create sanitizer each time
      let sanitizer = YouTubeUrlSanitizer::new();

      match msg.text() {
        Some(text) => {
          let reply_text: String = match to_url(text) {
            Ok(url) => match sanitizer.sanitize(&url) {
              Ok(link) => format!("Success: {}", link),
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

          bot.send_message(msg.chat.id, reply_text).await?;
        }
        None => {
          // TODO: answer "I accept text only?"
          bot.send_dice(msg.chat.id).await?;
        }
      }

      Ok(())
    })
    .await
  }
}
