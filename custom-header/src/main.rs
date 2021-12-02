// use fastly::{Error, Request, Response};
//
// #[fastly::main]
// fn main(mut req: Request) -> Result<Response, Error> {
//     req.set_header("X-Custom-Header", "from edge");
//     let resp = req.send("backend_a")?;
//
//     Ok(resp)
// }

use fastly::http::StatusCode;
use fastly::{mime, Error, Request, Response};

#[fastly::main]
fn main(_: Request) -> Result<Response, Error> {
    let mut resp = Response::from_status(StatusCode::OK)
        .with_content_type(mime::TEXT_HTML_UTF_8)
        .with_body(include_str!("welcome-to-compute@edge.html"));

    resp.set_header("X-Custom-Header", "from edge");

    Ok(resp)
}
