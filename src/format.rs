use csv::{Reader, StringRecord};
use serde_json::json;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn record_to_map(header: &csv::StringRecord, record: &StringRecord) -> HashMap<String, String> {
    header
        .iter()
        .zip(record.iter())
        .map(|(h, f)| (h.to_string(), f.to_string()))
        .collect()
}

pub fn validate_file_path(csv_file_path: &str) -> bool {
    if !Path::new(csv_file_path).exists() {
        println!("File {} does not exist!", csv_file_path);
        return false;
    }
    true
}

pub fn validate_headers(headers: &csv::StringRecord) -> bool {
    let required_columns =
        vec![
            "ChunkIDs",
            "ChunkTexts",
            "Question",
            "Answer",
            "Quoted_Text_ID",
        ];
    for column in &required_columns {
        if !headers.iter().any(|h| h == *column) {
            println!("Invalid file format! The CSV file must contain the columns: 'ChunkIDs', 'ChunkTexts', 'Question', 'Answer', 'Quoted_Text_ID'");
            return false;
        }
    }
    true
}

pub fn process_records(
    reader: &mut Reader<File>,
    headers: &csv::StringRecord,
) -> Vec<serde_json::Value> {
    let mut output = Vec::new();
    for result in reader.records() {
        let record = result.unwrap();
        let record_map = record_to_map(&headers, &record);
        let prompt = format!("You are an expert in answering questions based on provided evidence. Answer the given question using the following context. Question: {}\nContext:{}", record_map["Question"], record_map["ChunkTexts"]);
        let completion = format!(
            "Here is my answer: {}\nHere is the ID of the text I used to answer the question: {}",
            record_map["Answer"], record_map["Quoted_Text_ID"]
        );
        output.push(json!({"prompt": prompt, "completion": completion}));
    }
    output
}

pub fn write_output(output: Vec<serde_json::Value>, output_file_path: String) {
    let mut file = File::create(output_file_path.clone()).unwrap();
    for item in output {
        writeln!(file, "{}", item.to_string()).unwrap();
    }
    println!("JSONL file saved to {}", output_file_path);
}
