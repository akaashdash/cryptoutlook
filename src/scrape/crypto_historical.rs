use coinbase_pro_rs::{Public, Sync, SANDBOX_URL};

pub fn scrape() {
    let client: Public<Sync> = Public::new(SANDBOX_URL);
    let stats = client.get_stats24h("BTC");
    println!("{}", stats.unwrap().open);
}