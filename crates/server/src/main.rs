use miniserve::{Content, Request, Response};

fn index(_req: Request) -> Response {
    let content = include_str!("../index.html").to_string();
    Ok(Content::Html(content))
}

fn chat(_req: Request) -> Response {
    match _req {
        // TODO: Use actual HTTP error codes here instead
        Request::Get => Ok(Content::Json("Malformed request".to_string())),
        Request::Post(_msgs) => Ok(Content::Json("{\"messages\": [\"hi back\"]}".to_string())),
    }
}

fn main() {
    miniserve::Server::new()
        .route("/", index)
        .route("/chat", chat)
        .run()
}
