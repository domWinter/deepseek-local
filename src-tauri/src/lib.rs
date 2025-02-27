mod chat_completion;
use futures_util::{SinkExt, Stream, StreamExt, TryStreamExt};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite;
use serde::Deserialize;
use kalosm::language::*;
use chat_completion::CompletionModel;

#[derive(Deserialize, Debug)]
struct ChatRequest {
    messages: Vec<(String, String)>,
    system_prompt: String,
}

struct ConnectionHandler {
    chat: Chat<Llama>,
}


impl ConnectionHandler {
    fn new(model: CompletionModel, system_prompt: String) -> Self {
        Self {
            chat: model.create_chat(system_prompt),
        }
    }

    async fn handle_message(&mut self, msg: &str) -> Result<impl Stream<Item = String> + Send + 'static, anyhow::Error> {
        let req: ChatRequest = serde_json::from_str(msg)?;
        
        if let Some((_, content)) = req.messages.last() {
            Ok(self.chat.clone().into_add_message(content))
        } else {
            Ok(self.chat.clone().into_add_message(""))
        }
    }
}

async fn start_server() -> anyhow::Result<()> {
    let addr = "127.0.0.1:8080".to_string();
    let listener = TcpListener::bind(&addr).await?;
    let model = CompletionModel::new().await?;
    
    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(accept_connection(stream, model.clone()));
    }
    Ok(())
}

async fn accept_connection(stream: TcpStream, model: CompletionModel) -> anyhow::Result<()> {
    let ws_stream = tokio_tungstenite::accept_async(stream).await?;
    let (mut write, mut read) = ws_stream.split();

    // Create a new connection handler with its own chat session
    let mut handler = ConnectionHandler::new(
        model,
        "You are an AI assistant helping the user and always answering in german!".to_string()
    );

    while let Some(msg) = read.try_next().await? {
        println!("New message: {:?}", msg);
        match msg {
            tungstenite::Message::Text(msg) => {
                let mut stream = handler.handle_message(&msg).await?;
                
                while let Some(resp) = stream.next().await {
                    write.send(tungstenite::Message::Text(resp.into())).await?;
                }
                
                write.send(tungstenite::Message::Text("<END>".into())).await?;
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
