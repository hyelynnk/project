use crate::common::RawRecord;
use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;

pub fn load_data(path: &str) -> Result<Vec<RawRecord>, Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new()
        .delimiter(b',')
        .has_headers(true)
        .trim(csv::Trim::All)
        .flexible(true)
        .from_reader(File::open(path)?);

    let mut dataset = Vec::new();

    for result in rdr.deserialize::<RawRecord>() {
        match result {
            Ok(mut record) => {
                if let Some(month_str) = record.date.split('/').nth(1) {
                    record.month = month_str.parse().unwrap_or(1);
                }                
                if record.functioning_day == "Yes" {
                    dataset.push(record);
                }
            },
            Err(e) => {
                eprintln!("Error Input: {}", e);
                continue;
            }
        }
    }

    if dataset.is_empty() {
        return Err("No valid records.".into());
    }

    Ok(dataset)
}
