use kalosm::language::*;

#[derive(Clone)]
pub struct CompletionModel {
    model: Llama,
}

impl CompletionModel {
    pub async fn new() -> anyhow::Result<Self> {
        let model = Llama::builder()
            .with_source(
                LlamaSource::new(FileSource::Local(
                    "resources/deepseek-llm-7b-chat.Q5_K_S.gguf".into(),
                ))
                .with_tokenizer(FileSource::Local("resources/tokenizer.json".into())),
            )
            .build()
            .await?;
        
        Ok(Self { model })
    }

    // Creates a new chat session
    pub fn create_chat(&self, system_prompt: String) -> Chat<Llama> {
        self.model.chat().with_system_prompt(system_prompt)
    }
}