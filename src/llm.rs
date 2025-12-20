use openai_api_rust::chat::*;
use openai_api_rust::*;

pub fn send_message(messages: Vec<Message>) -> Message {
    let auth = Auth::from_env().expect("Failed to load auth from environment");
    let endpoint = std::env::var("MODEL_URI").expect("Failed to load MODEL_URI from environment");
    let openai = OpenAI::new(auth, &endpoint);
    let body = ChatBody {
        model: "gpt-5-mini".to_string(),
        n: Some(2),
        stream: Some(false),
        messages,
        frequency_penalty: None,
        presence_penalty: None,
        max_tokens: None,
        temperature: None,
        top_p: None,
        stop: None,
        user: None,
        logit_bias: None,
    };
    let rs = openai.chat_completion_create(&body);
    let choice = rs.unwrap().choices;
    choice[0].message.to_owned().unwrap()
}
