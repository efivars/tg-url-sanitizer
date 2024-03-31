use teloxide::prelude::*;

use url_sanitizer::is_valid_url;

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
    .await
  }
}
