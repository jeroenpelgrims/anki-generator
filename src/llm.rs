use async_openai_compat::{
    Client,
    config::AzureConfig,
    types::{
        ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs,
        CreateChatCompletionResponse,
    },
};

pub async fn send_message() -> Result<CreateChatCompletionResponse, Box<dyn std::error::Error>> {
    let api_key =
        std::env::var("OPENAI_API_KEY").expect("Failed to load OPENAI_API_KEY from environment");
    let endpoint = std::env::var("MODEL_URI").expect("Failed to load MODEL_URI from environment");
    let config = AzureConfig::new()
        .with_api_base(endpoint)
        .with_api_key(api_key)
        .with_api_version("2024-12-01-preview")
        .with_deployment_id("gpt-5-mini");
    let client = Client::with_config(config);

    let request = CreateChatCompletionRequestArgs::default()
        .model("gpt-5-mini")
        .messages([ChatCompletionRequestUserMessageArgs::default()
            .content("Your prompt here")
            .build()?
            .into()])
        .build()?;

    let response = client.chat().create(request).await?;
    Ok(response)
}
