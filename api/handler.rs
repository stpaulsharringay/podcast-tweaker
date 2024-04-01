use rss::Channel;
use std::str::FromStr;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    let host = req.headers().get("Host").map(|h| h.to_str().unwrap());

    println!("Request from host: {}", host.unwrap_or("Unknown"));
    let channel = get_updated_feed().await;
    let response: Result<Response<Body>, Error> = if let Ok(channel) = channel {
        Ok(Response::builder()
            .status(StatusCode::OK)
            .header("Cache-Control", "public, s-maxage=60")
            .body(channel.to_string().into())?)
    } else {
        Ok(Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .header("Cache-Control", "public, s-maxage=60")
            .body("".into())?)
    };
    response
}

async fn get_updated_feed() -> Result<Channel, Box<dyn std::error::Error>> {
    let feed = get_feed("https://stpaulsharringay.com/wp-content/uploads/podcast.xml").await?;
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

    Ok(channel)
}

async fn get_feed(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let bytes = reqwest::get(url).await?.bytes().await?;
    let content = std::str::from_utf8(&bytes)?;
    Ok(content.into())
}
