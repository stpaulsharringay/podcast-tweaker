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
                // Cache at Vercel's edge for 1h, and keep serving a stale copy (while
                // refreshing in the background) for up to a day, so most podcast-app
                // polls never invoke the function. Clients may cache for 15 min.
                .header(
                    "Cache-Control",
                    "public, max-age=900, s-maxage=3600, stale-while-revalidate=86400",
                )
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
