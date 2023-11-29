use crate::config::{
    custom_prompt_prefix, CUSTOM_JSON_FORMAT, CUSTOM_SYSTEM_MESSAGE, MODEL_CHOICE, NUM_CHUNKS,
    NUM_TRIPLETS,
};
use anyhow::Result;
use bullpen::v1::resources::pplx::chat_completion::{ChatMessage, PplxChatCompletionRequest};
use clap::{Parser, Subcommand};
use csv::Reader;
use dotenv::dotenv;
use format::{process_records, validate_file_path, validate_headers, write_output};
use generate::read_csv;
use itertools::Itertools;
use std::path::PathBuf;

pub mod config;
pub mod format;
pub mod generate;

#[derive(Parser)]
#[clap(version = "1.0", author = "Kody Low <kodylow7@gmail.com>")]
struct Cli {
    /// Sets a custom config file
    #[clap(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[clap(short, long)]
    debug: bool,

    /// Sets the custom system message
    #[clap(short = 's', long)]
    custom_system_message: Option<String>,

    /// Sets the custom JSON format
    #[clap(short = 'j', long)]
    custom_json_format: Option<String>,

    /// Sets the model choice
    #[clap(short = 'm', long)]
    model_choice: Option<String>,

    /// Sets the number of chunks
    #[clap(short = 'n', long)]
    num_chunks: Option<usize>,

    /// Sets the number of triplets
    #[clap(short = 't', long)]
    num_triplets: Option<usize>,

    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Format(Format),
    Generate(Generate),
}

#[derive(Parser)]
struct Format {
    /// Sets the input CSV file
    #[clap(short, long)]
    input: String,
}

#[derive(Parser)]
struct Generate {
    /// Sets the input CSV file
    #[clap(short, long)]
    input: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Format(cmd) => {
            let csv_file_path = &cmd.input;
            if !validate_file_path(csv_file_path) {
                return Err(anyhow::anyhow!("Invalid file path"));
            }

            let mut reader = Reader::from_path(csv_file_path).unwrap();
            let headers = reader.headers().unwrap().clone();

            if !validate_headers(&headers) {
                return Err(anyhow::anyhow!("Invalid headers"));
            }

            let output = process_records(&mut reader, &headers);

            let output_file_path =
                format!("{}_output.jsonl", csv_file_path.trim_end_matches(".csv"));
            write_output(output, output_file_path);
        }
        Commands::Generate(cmd) => {
            let csv_file_path = &cmd.input;
            if !validate_file_path(csv_file_path) {
                return Err(anyhow::anyhow!("Invalid file path"));
            }

            dotenv().ok();
            let pplx_api_key = std::env::var("PPLX_API_KEY")?;
            let pplx = bullpen::v1::api::Pplx::new(pplx_api_key);

            // Read the CSV file
            let chunks = read_csv(csv_file_path)?;

            // Custom messages and formats
            let custom_system_message = cli
                .custom_system_message
                .unwrap_or_else(|| CUSTOM_SYSTEM_MESSAGE.to_string());
            let _custom_json_format = cli
                .custom_json_format
                .unwrap_or_else(|| CUSTOM_JSON_FORMAT.to_string());
            let model_choice = match cli.model_choice {
                Some(model) => model.parse::<bullpen::v1::models::PplxChatModel>()?,
                None => MODEL_CHOICE,
            };
            let num_chunks = cli.num_chunks.unwrap_or(NUM_CHUNKS);
            let num_triplets = cli.num_triplets.unwrap_or(NUM_TRIPLETS);
            let custom_prompt_prefix = custom_prompt_prefix(num_triplets);

            // Create all possible combinations of chunks
            let chunk_combinations = chunks.into_iter().combinations(num_chunks);

            for chunk_group in chunk_combinations {
                let combined_text: String = chunk_group
                    .iter()
                    .map(|chunk| format!("Text {} {{\n{}\n}}", chunk.chunk_id, chunk.chunk_text))
                    .collect::<Vec<String>>()
                    .join("\n\n");

                // Create the prompt
                let prompt = PplxChatCompletionRequest {
                    model: model_choice,
                    messages: vec![
                        ChatMessage {
                            role: bullpen::v1::resources::pplx::chat_completion::Role::System,
                            content: custom_system_message.to_string(),
                            name: None,
                        },
                        ChatMessage {
                            role: bullpen::v1::resources::pplx::chat_completion::Role::User,
                            content: custom_prompt_prefix.to_string() + &combined_text,
                            name: None,
                        },
                    ],

                    ..Default::default()
                };

                let res = pplx
                    .chat(prompt)
                    .await
                    .expect("Failed to generate chat completion");

                println!("Response: {:?}", res);

                break;
            }
        }
    }

    Ok(())
}
