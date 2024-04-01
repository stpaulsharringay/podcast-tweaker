use std::{error::Error, str::FromStr};

use rss::Channel;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let feed = get_feed("https://stpaulsharringay.com/wp-content/uploads/podcast.xml")
        .await
        .unwrap();
    let mut channel = Channel::from_str(&feed)?;
    for item in channel.items_mut() {
        let sermon_title = item.title.as_ref();
        let passage = item.description.as_ref().map(|s| {
            s.trim()
                .replace(": ", ":")
                .replace(" :", ":")
                .replace("- ", "-")
                .replace(" -", "-")
        });

        let new_title: Option<String> = match (sermon_title, passage) {
            (Some(title), Some(passage)) => Some(format!("{title} ({passage})")),
            (None, Some(passage)) => Some(format!("{}", &passage)),
            (Some(title), None) => Some(format!("{}", &title)),
            (None, None) => None,
        };
        item.title = new_title;
        let author = item
            .itunes_ext
            .as_ref()
            .and_then(|e| e.author.as_ref())
            .map(|a| a.trim());
        item.description = author.map(|a| a.to_owned());
    }
    println!("{}", channel.to_string());

    Ok(())
}

async fn get_feed(url: &str) -> Result<String, Box<dyn Error>> {
    let bytes = reqwest::get(url).await?.bytes().await?;
    let content = std::str::from_utf8(&bytes)?;
    Ok(content.into())
}
