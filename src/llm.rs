use async_openai_compat::{
    Client,
    config::AzureConfig,
    types::{
        ChatCompletionRequestMessage, ChatCompletionRequestUserMessage,
        ChatCompletionRequestUserMessageContent, CreateChatCompletionRequestArgs, ResponseFormat,
        ResponseFormatJsonSchema,
    },
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(deny_unknown_fields)]
pub struct TranslationResponseItem {
    pub original_word: String,
    pub article: String,
    pub translated_word: String,
}
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(deny_unknown_fields)]
struct TranslationResponse {
    translations: Vec<TranslationResponseItem>,
}

pub async fn translate(
    input: Vec<String>,
    target_language: String,
) -> Result<Vec<TranslationResponseItem>, anyhow::Error> {
    let schema = schemars::schema_for!(TranslationResponse);
    let response_format = ResponseFormat::JsonSchema {
        json_schema: ResponseFormatJsonSchema {
            schema: Some(serde_json::to_value(schema)?),
            strict: Some(true),
            name: "translation_response".to_string(),
            description: Some("Translation response structure".to_string()),
        },
    };

    let client = get_client();
    let messages: Vec<ChatCompletionRequestMessage> = vec![ChatCompletionRequestMessage::User(
        ChatCompletionRequestUserMessage {
            content: ChatCompletionRequestUserMessageContent::Text(format!(
                "Translate the following words to the language of this language code: {}. Put the article in a separate field. The translated word should ONLY contain the word, not the article.\n\n{}",
                target_language,
                input.join("\n")
            )),
            name: None,
        },
    )];
    let request = CreateChatCompletionRequestArgs::default()
        .model("gpt-5-mini")
        .messages(messages)
        .response_format(response_format)
        .store(false)
        .build()?;
    let response = client.chat().create(request).await?;
    let content = response.choices[0].message.content.as_ref().unwrap();
    let translation = serde_json::from_str::<TranslationResponse>(content)?;

    println!("Translation response: {:?}", translation);

    Ok(translation.translations)
}
