use url_sanitizer::is_valid_url;

fn main() {
  let url = "youtube.com/watch?v=FjUm3C8xcQI";

  match is_valid_url(url) {
    true => println!("OK URL {}", url),
    false => println!("Bad URL: {}", url),
  }
}
