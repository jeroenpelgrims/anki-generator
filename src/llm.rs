use async_openai_compat::{
    Client,
    config::AzureConfig,
    types::{
        ChatCompletionRequestMessage, ChatCompletionRequestSystemMessage,
        ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
        CreateChatCompletionRequestArgs, CreateChatCompletionResponse,
    },
};

pub fn get_client() -> Client<AzureConfig> {
    let api_key =
        std::env::var("OPENAI_API_KEY").expect("Failed to load OPENAI_API_KEY from environment");
    let endpoint = std::env::var("MODEL_URI").expect("Failed to load MODEL_URI from environment");
    let config = AzureConfig::new()
        .with_api_base(endpoint)
        .with_api_key(api_key)
        .with_api_version("2024-12-01-preview")
        .with_deployment_id("gpt-5-mini");
    Client::with_config(config)
}

// pub async fn send_message(
//     messages: Vec<ChatCompletionRequestMessage>,
// ) -> Result<CreateChatCompletionResponse, Box<dyn std::error::Error>> {
//     let client = get_client();
//     let request = CreateChatCompletionRequestArgs::default()
//         .model("gpt-5-mini")
//         .messages(messages)
//         .build()?;

//     let response = client.chat().create(request).await?;
//     Ok(response)
// }

pub async fn translate(
    input: Vec<String>,
    target_language: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = get_client();
    let messages: Vec<ChatCompletionRequestMessage> = vec![
        ChatCompletionRequestSystemMessageArgs::default()
            .content(
                "You are a helpful assistant that translates text from one language to another.",
            )
            .build()?
            .into(),
        ChatCompletionRequestUserMessageArgs::default()
            .content("Your prompt here")
            .build()?
            .into(),
    ];
    let request = CreateChatCompletionRequestArgs::default()
        .model("gpt-5-mini")
        .messages(messages)
        .store(false)
        .build()?;
    let response = client.chat().create(request).await?;

    Ok(())
}
