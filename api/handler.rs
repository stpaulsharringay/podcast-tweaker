use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(_req: Request) -> Result<Response<Body>, Error> {
    match podcast_tweaker::get_updated_feed().await {
        Ok(channel) => {
            eprintln!("Feed fetched and transformed successfully ({} items)", channel.items().len());
            Ok(Response::builder()
                .status(StatusCode::OK)
                .header("Cache-Control", "public, s-maxage=60")
                .body(channel.to_string().into())?)
        }
        Err(e) => {
            eprintln!("ERROR: failed to fetch/transform feed: {e:?}");
            Ok(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .header("Cache-Control", "public, s-maxage=60")
                .body("".into())?)
        }
    }
}
