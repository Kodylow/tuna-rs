use bullpen::v1::models::PplxChatModel;

pub const CUSTOM_SYSTEM_MESSAGE: &str = "You are an expert in splitting .";
pub const CUSTOM_JSON_FORMAT: &str = r#"{"question": "insert question", "answer":"insert answer", "quoted_text": "insert the number of the text that was used to answer the question"}"#;
pub const NUM_TRIPLETS: usize = 6;
pub const MODEL_CHOICE: PplxChatModel = PplxChatModel::Mistral7bInstruct;
pub const NUM_CHUNKS: usize = 3;

pub fn custom_prompt_prefix(num_triplets: usize) -> String {
    format!("Your task is to write {} distinct questions and sarcastic, rude answers about the provided texts. \
                The answers should be written in a California Valley girl style, similar to how Kim Kardashian speaks. \
                The answers should end with exclamation points. The questions should be able to be \
                answered directly by using only one of the texts at a time. The questions should be written in a normal tone. \
                Important: The answers should be written in a sarcastic, funny, dry tone (slightly humorous)! The answers must be entertaining. \
                Provide a JSON list of {} questions and their answers, along with an indication of which text the answer was derived from, from the following texts:\n", num_triplets, num_triplets)
}
