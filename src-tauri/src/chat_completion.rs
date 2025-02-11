use mistralrs::{
    GgufModelBuilder, IsqType, Model, NormalRequest, PagedAttentionMetaBuilder, Request,
    RequestLike, TextMessageRole, TextMessages, TextModelBuilder,
};
use std::sync::Arc;
use tokio::sync::mpsc::channel;
use tokio_stream::wrappers::ReceiverStream;

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
        .await.unwrap();

        // Create the MistralRs, which is a runner
        Ok(Self {
            model: Arc::new(model),
        })
    }

    pub async fn complete(
        &self,
        request_str: &str,
    ) -> anyhow::Result<ReceiverStream<mistralrs::Response>> {
        let mut request = TextMessages::new().add_message(TextMessageRole::User, request_str);

        let (tx, rx) = channel(1);

        let (tools, tool_choice) = if let Some((a, b)) = request.take_tools() {
            (Some(a), Some(b))
        } else {
            (None, None)
        };
        let request = Request::Normal(NormalRequest {
            messages: request.take_messages(),
            sampling_params: request.take_sampling_params(),
            response: tx,
            return_logprobs: request.return_logprobs(),
            is_streaming: true,
            id: 0,
            constraint: request.take_constraint(),
            suffix: None,
            adapters: request.take_adapters(),
            tools,
            tool_choice,
            logits_processors: request.take_logits_processors(),
            return_raw_logits: false,
        });

        self.model.inner().get_sender()?.send(request).await?;

        Ok(ReceiverStream::new(rx))
    }
}
