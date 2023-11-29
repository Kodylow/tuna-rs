use clap::Parser;
use csv::Reader;
use format::{process_records, validate_file_path, validate_headers, write_output};

pub mod format;

#[derive(Parser)]
#[clap(version = "1.0", author = "Kody Low <kodylow7@gmail.com>")]
struct Cli {
    /// Sets the input CSV file
    #[clap(short, long)]
    input: String,
}

fn main() {
    let args = Cli::parse();

    let csv_file_path = &args.input;
    if !validate_file_path(csv_file_path) {
        return;
    }

    let mut reader = Reader::from_path(csv_file_path).unwrap();
    let headers = reader.headers().unwrap().clone();

    if !validate_headers(&headers) {
        return;
    }

    let output = process_records(&mut reader, &headers);

    let output_file_path = format!("{}_output.jsonl", csv_file_path.trim_end_matches(".csv"));
    write_output(output, output_file_path);
}
