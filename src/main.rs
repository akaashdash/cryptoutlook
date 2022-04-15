mod scrape;

fn main() {
    println!("Hello, world!");
    // scrape::google_trends::scrape();
    scrape::crypto_historical::scrape();
}
