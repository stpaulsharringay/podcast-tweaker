use podcast_tweaker::get_updated_feed;

#[tokio::main]
async fn main() {
    eprintln!("Fetching and transforming feed...");
    match get_updated_feed().await {
        Ok(channel) => {
            eprintln!("Success! {} items", channel.items().len());
            println!("{}", channel.to_string());
        }
        Err(e) => {
            eprintln!("ERROR: {:?}", e);
            std::process::exit(1);
        }
    }
}
