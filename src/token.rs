use std::env;

use dotenv::dotenv;

pub(crate) const TOKEN_ENV_VAR: &'static str = "TG_URL_SANITIZER_TOKEN";

pub fn try_get_token() -> Option<String> {
  try_get_arg_token().or_else(try_get_env_token)
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
  const TOKEN_ARG_VAR: &'static str = "--token";

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
