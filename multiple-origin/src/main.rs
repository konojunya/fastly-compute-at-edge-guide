use fastly::{http::header, Error, Request, Response};

const BACKEND_A: &str = "backend_a";
const BACKEND_B: &str = "backend_b";

#[fastly::main]
fn main(mut req: Request) -> Result<Response, Error> {
    let backend = match req.get_path() {
        "/get" => {
            req.set_header(header::HOST, "httpbin.org");
            BACKEND_A
        }
        _ => {
            req.set_header(header::HOST, "example.com");
            BACKEND_B
        }
    };

    let resp = req.send(backend)?;
    Ok(resp)
}
