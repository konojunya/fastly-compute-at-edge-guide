use fastly::http::StatusCode;
use fastly::{mime, Error, Request, Response};

#[fastly::main]
fn main(_: Request) -> Result<Response, Error> {
    Ok(Response::from_status(StatusCode::OK)
        .with_content_type(mime::TEXT_HTML_UTF_8)
        .with_body(include_str!("welcome-to-compute@edge.html")))
}
