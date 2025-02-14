use mistralrs::{
    GgufModelBuilder, Model, NormalRequest, Request, RequestLike, TextMessageRole, TextMessages,
};
use std::sync::Arc;
use tokio::sync::mpsc::channel;
use tokio_stream::wrappers::ReceiverStream;

use crate::ChatRequest;

#[derive(Clone)]
pub struct CompletionModel {
    pub model: Arc<Model>,
}

impl CompletionModel {
    pub async fn new() -> anyhow::Result<Self> {
        // Select a Mistral model

        let model = GgufModelBuilder::new(
            "TheBloke/deepseek-llm-7B-chat-GGUF",
            vec!["deepseek-llm-7b-chat.Q5_K_S.gguf"],
        )
        .with_tok_model_id("deepseek-ai/deepseek-llm-7b-chat")
        .with_logging()
        .build()
        .await
        .unwrap();

        Ok(Self {
            model: Arc::new(model),
        })
    }

    pub async fn complete(
        &self,
        req: ChatRequest,
    ) -> anyhow::Result<ReceiverStream<mistralrs::Response>> {
        let mut text_messages =
            TextMessages::new().add_message(TextMessageRole::System, req.system_prompt);

        for (role, content) in req.messages {
            let role = match role.as_str() {
                "user" => TextMessageRole::User,
                "assistant" => TextMessageRole::Assistant,
                _ => continue, // Skip unknown roles
            };
            text_messages = text_messages.add_message(role, content);
        }

        let (tx, rx) = channel(1);

        let (tools, tool_choice) = if let Some((a, b)) = text_messages.take_tools() {
            (Some(a), Some(b))
        } else {
            (None, None)
        };
        let request = Request::Normal(NormalRequest {
            messages: text_messages.take_messages(),
            sampling_params: text_messages.take_sampling_params(),
            response: tx,
            return_logprobs: text_messages.return_logprobs(),
            is_streaming: true,
            id: 0,
            constraint: text_messages.take_constraint(),
            suffix: None,
            adapters: text_messages.take_adapters(),
            tools,
            tool_choice,
            logits_processors: text_messages.take_logits_processors(),
            return_raw_logits: false,
        });

        self.model.inner().get_sender()?.send(request).await?;

        Ok(ReceiverStream::new(rx))
    }
}
