use miniserve::{Content, Request, Response};
use tokio::join;
use serde::{Deserialize, Serialize};

async fn index(_req: Request) -> Response {
    let content = include_str!("../index.html").to_string();
    Ok(Content::Html(content))
}

#[derive(Serialize, Deserialize)]
struct Messages {
    messages: Vec<String>,
}

async fn chat(_req: Request) -> Response {
    match _req {
        // TODO: Use actual HTTP error codes here instead?
        Request::Get => Ok(Content::Json("Malformed request".to_string())),
        Request::Post(umsgs) => {
            let mut messages: Messages = serde_json::from_str(&umsgs).unwrap_or(Messages {
                messages: vec!["parse messages fail".to_string()],
            });

            let response_index_fut = chatbot::gen_random_number();
            let possible_responses_fut = chatbot::query_chat(&messages.messages);
            let (ix, mut possible_responses) = join!(response_index_fut, possible_responses_fut);


            let response_msg = possible_responses.remove(ix % possible_responses.len());

            messages.messages.push(response_msg);

            let response = serde_json::to_string(&messages).unwrap_or("to_string fail".to_string());
            Ok(Content::Json(response))
        }
    }
}

#[tokio::main]
async fn main() {
    miniserve::Server::new()
        .route("/", index)
        .route("/chat", chat)
        .run()
        .await
}
