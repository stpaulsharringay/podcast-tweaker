use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(_req: Request) -> Result<Response<Body>, Error> {
    let channel = podcast_tweaker::get_updated_feed().await;
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
