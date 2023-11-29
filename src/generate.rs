use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Chunk {
    #[serde(rename = "ChunkID")]
    pub chunk_id: String,
    #[serde(rename = "ChunkText")]
    pub chunk_text: String,
}

// Function to read the CSV file and return a Vec of Chunks
pub fn read_csv(file_path: &str) -> Result<Vec<Chunk>> {
    let mut rdr = csv::Reader::from_path(file_path)?;
    let mut chunks = Vec::new();

    for result in rdr.deserialize() {
        let record: Chunk = result?;
        chunks.push(record);
    }

    Ok(chunks)
}
