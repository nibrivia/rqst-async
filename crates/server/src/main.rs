use chatbot::{gen_random_number, query_chat};
use miniserve::{Content, Request, Response};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::join;

async fn index(_req: Request) -> Response {
    let content = include_str!("../index.html").to_string();
    Ok(Content::Html(content))
}

#[derive(Serialize, Deserialize, Clone)]
struct Messages {
    messages: Vec<String>,
}

async fn make_response(input_json: String) -> Option<String> {
    // Parse
    let data: Messages = serde_json::from_str(&input_json).ok()?;

    // Call async
    let response_index_fut = tokio::spawn(gen_random_number());

    let messages_arc = Arc::new(data);
    let messages_clone = Arc::clone(&messages_arc);
    let possible_responses_fut =
        tokio::spawn(async move { query_chat(&messages_clone.messages).await });

    let (ix_res, responses_res) = join!(response_index_fut, possible_responses_fut);

    // Build response
    let mut possible_responses = responses_res.ok()?;
    let response_msg = possible_responses.remove(ix_res.ok()? % possible_responses.len());

    let mut res = Arc::into_inner(messages_arc)?;
    res.messages.push(response_msg);

    serde_json::to_string(&res).ok()
}

async fn chat(_req: Request) -> Response {
    match _req {
        // TODO: Use actual HTTP error codes here instead?
        Request::Get => Ok(Content::Json("Malformed request".to_string())),
        Request::Post(umsgs) => {
            let response = make_response(umsgs)
                .await
                .unwrap_or("response error".to_string());
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
