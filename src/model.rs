//This module specifies logic used to classify the weather conditions into different groups, which will be used for search algorithms
use crate::common::{RawRecord, DaySummary, Weather, TemperatureCategory, PrecipitationIntensity};
use std::collections::HashMap;

//This function takes in data (raw record) as the input and then returns the vector after summarizing daily totals
//This step is crucial because the original data has separate rows for each hour, but by getting daily totals, we look at daily bike demands
pub fn summarize_daily_totals(data: &[RawRecord]) -> Vec<DaySummary> {
    let mut daily_map: HashMap<String, DaySummary> = HashMap::new();

    //The for loop, for every "data", does four main jobs.
    for record in data {
        let date_key = record.date.clone();

        //First is to look at snowfall, rainfall, and visibility statistics to identify the appropriate the weather label
        let weather = if record.snowfall > 0.0 {
            Weather::Snowy
        } else if record.rainfall > 0.0 {
            Weather::Rainy
        } else if record.visibility < 500.0 {
            Weather::Foggy
        } else {
            Weather::Sunny
        };

        //Second is to look at precipitation intensity to either label as light or heavy
        let precipitation = if record.snowfall > 0.0 {
            if record.snowfall < 1.0 {
                PrecipitationIntensity::Light
            } else {
                PrecipitationIntensity::Heavy
            }
        } else {
            PrecipitationIntensity::None
        };

        //Third is to look at the temperature and appropriately categorize it using 10 and 25 degrees benchmarks
        let temperature_category = if record.temperature > 25.0 {
            TemperatureCategory::Hot
        } else if record.temperature < 10.0 {
            TemperatureCategory::Cold
        } else {
            TemperatureCategory::Moderate
        };

        //The code below attempts to concatenate each hourly rented bike count for each respective day to get daily totals
        daily_map.entry(date_key.clone())
            .and_modify(|day| day.total_rentals += record.rented_bike_count)
            .or_insert(DaySummary {
                date: date_key,
                month: record.month,
                total_rentals: record.rented_bike_count,
                weather,
                temperature_category,
                precipitation,
            });
    }
    //These are now added to the output vector
    daily_map.into_values().collect()
}
