//This module initializes the dataframe by loading the csv file and turning it into the vector of raw records
use crate::common::RawRecord;
use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;

//This function loads in the csv file by taking in the path and outputting the vector of raw records
pub fn load_data(path: &str) -> Result<Vec<RawRecord>, Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new()
        .delimiter(b',')
        .has_headers(true)
        .trim(csv::Trim::All)
        .flexible(true)
        .from_reader(File::open(path)?);

    let mut dataset = Vec::new();

    //The for loop iterates through the raw records(rows) and then for each row, splits the date apart to specify the date/month/year structure
    //Then once it is checked that the raw record represents a functioning day, it is added to the dataset
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
    Ok(dataset)
}

//The test creates an arbitrary csv file and loads the data
//Looking at the first record, it checks through assert_eq! that each variable matches the expected
#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_load_data_success() {

        let test_csv = "\
    Date,Rented Bike Count,Hour,Temperature (°C),Humidity (%),Wind speed (m/s),Visibility (10m),Dew point temperature(°C),Solar Radiation (MJ/m2),Rainfall(mm),Snowfall (cm),Seasons,Holiday,Functioning Day
    01/12/2017,254,0,2.1,81.0,1.5,2000,0.9,0.0,0.0,0.0,Winter,No Holiday,Yes
    02/12/2017,204,1,1.8,80.0,2.0,1800,0.7,0.0,0.0,0.0,Winter,No Holiday,Yes";

        let path = "test_data_full.csv";
        let mut file = File::create(path).unwrap();
        writeln!(file, "{}", test_csv).unwrap();

        let result = crate::data::load_data(path);
        assert!(result.is_ok());
        let records = result.unwrap();
        assert_eq!(records.len(), 2);

        let rec = &records[0];
        assert_eq!(rec.date, "01/12/2017");
        assert_eq!(rec.rented_bike_count, 254);
        assert_eq!(rec.hour, 0);
        assert_eq!(rec.temperature, 2.1);
        assert_eq!(rec.humidity, 81.0);
        assert_eq!(rec.wind_speed, 1.5);
        assert_eq!(rec.visibility, 2000.0);
        assert_eq!(rec.dew_point_temp, 0.9);
        assert_eq!(rec.solar_radiation, 0.0);
        assert_eq!(rec.rainfall, 0.0);
        assert_eq!(rec.snowfall, 0.0);
        assert_eq!(rec.seasons, "Winter");
        assert_eq!(rec.holiday, "No Holiday");
        assert_eq!(rec.functioning_day, "Yes");

        std::fs::remove_file(path).unwrap();
    }
}