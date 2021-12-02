use fastly::http::{header, StatusCode};
use fastly::{mime, Error, Request, Response};

static BASIC_AUTH: &str = "Basic aWQ6cGFzc3dvcmQ=";

fn response_unauthorized() -> Result<Response, Error> {
    Ok(Response::from_status(StatusCode::UNAUTHORIZED)
        .with_content_type(mime::TEXT_HTML_UTF_8)
        .with_header(header::WWW_AUTHENTICATE, "Basic")
        .with_body(include_str!("unauthorized.html")))
}

#[fastly::main]
fn main(req: Request) -> Result<Response, Error> {
    if let Some(v) = req.get_header_str("Authorization") {
        if v != BASIC_AUTH {
            return response_unauthorized();
        }

        return Ok(Response::from_status(StatusCode::OK)
            .with_content_type(mime::TEXT_HTML_UTF_8)
            .with_body(include_str!("welcome-to-compute@edge.html")));
    }

    response_unauthorized()
}
