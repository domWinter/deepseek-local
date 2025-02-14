mod chat_completion;
use futures_util::{SinkExt, StreamExt};
use mistralrs::Response;
use tokio::net::{TcpListener, TcpStream};
use tokio_stream::StreamExt as TokioStreamExt;
use tokio_tungstenite::tungstenite;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct ChatRequest {
    messages: Vec<(String, String)>,
    system_prompt: String,
}

async fn start_server() -> anyhow::Result<()> {
    let addr = "127.0.0.1:8080".to_string();

    // Create the event loop and TCP listener we'll accept connections on.
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");

    let model = chat_completion::CompletionModel::new().await.unwrap();

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(accept_connection(stream, model.clone()));
    }

    Ok(())
}

async fn accept_connection(stream: TcpStream, model: chat_completion::CompletionModel) -> anyhow::Result<()> {
    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");

    let (mut write, mut read) = ws_stream.split();

    while let Some(msg) = read.try_next().await? {
        match msg {
            tungstenite::Message::Text(msg) => {
                let req: ChatRequest = serde_json::from_str(&msg)?;

                let mut stream = model.complete(req).await.unwrap();
                while let Some(resp) = TokioStreamExt::next(&mut stream).await {
                    match resp {
                        Response::Chunk(chunk) => {
                            let choice = &chunk.choices[0];
                            write
                                .send(tungstenite::Message::Text(
                                    choice.delta.content.clone().unwrap().into(),
                                ))
                                .await?;
                        },
                        Response::Done(_) => {
                            println!("Completed!!!")
                        }
                        Response::ModelError(error,_) => {
                            println!("Error: {}", error)
                        },
                        Response::InternalError(error) => {
                            println!("Error: {:?}", error)
                        }
                        _ => {
                            println!("Did not match Chunk or CompletionChunk");
                        },
                    }
                }
                write
                .send(tungstenite::Message::Text(
                    "<END>".into()
                ))
                .await?;
            }
            _ => (),
        }
    }
    Ok(())
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::async_runtime::spawn(start_server());
    tauri::Builder::default()
        .plugin(tauri_plugin_websocket::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
