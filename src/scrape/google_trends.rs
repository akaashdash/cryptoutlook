use rtrend::{Client, Country, Keywords, SearchInterest, Period};
use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error};
use serde_json::json;

pub fn scrape() {
    let crypto_query = Client::new(Keywords::new(vec!["crypto", "cryptocurrency"]), Country::ALL).with_period(Period::NinetyDay).build();
    let btc_query = Client::new(Keywords::new(vec!["BTC", "bitcoin"]), Country::ALL).with_period(Period::NinetyDay).build();
    let eth_query = Client::new(Keywords::new(vec!["ETH", "ethereum"]), Country::ALL).with_period(Period::NinetyDay).build();
    let sol_query = Client::new(Keywords::new(vec!["SOL", "solana"]), Country::ALL).with_period(Period::NinetyDay).build();
    let crypto_result = SearchInterest::new(crypto_query).get();
    let btc_result = SearchInterest::new(btc_query).get();
    let eth_result = SearchInterest::new(eth_query).get();
    let sol_result = SearchInterest::new(sol_query).get();

    let combined = json!({
        "crypto" : crypto_result,
        "btc" : btc_result,
        "eth" : eth_result,
        "sol" : sol_result
    });

    let file_path = "../../data/google_trends.json";
    let mut out_file = File::create(file_path);
    write!(out_file, serde_json::to_string(&combined));
}