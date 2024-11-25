use std::{collections::HashMap, error::Error, fs::File, io::BufReader, path::Path};

use csv::ReaderBuilder; // csv = "1.3.0"
// use serde::Deserialize; // serde = { version = "1", features = ["derive"] }

// #[derive(Deserialize)]
// struct Row {
//     a: String,
//     c: String,
// }

pub fn read_tsv(file_path: &Path) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let reader = BufReader::new(File::open(file_path)?);
    let mut rdr = ReaderBuilder::new()
        .delimiter(b'\t')
        .comment(Some(b'#'))
        .from_reader(reader);
    // let mut labels: HashMap<String, String> = HashMap::new();
    // for result in rdr.records() {
    //     let record = result?;
    //     if record.len() >= 2 {
    //         labels.insert(record[0].to_string(), record[1].to_string());
    //     } else {
    //         // Handle cases with less than 2 fields as needed
    //         eprintln!("Warning: Skipping invalid record: {:?}", record);
    //     }
    // }
    let labels: HashMap<String, String> = rdr.records()
    .filter_map(|result| {
        match result {
            Ok(record) if record.len() >= 2 => Some((record[0].to_string(), record[1].to_string())),
            _ => None, // Handle invalid records as needed
        }
    })
    .collect();
    Ok(labels)
}
