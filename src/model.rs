use crate::common::{RawRecord, DaySummary, Weather, TemperatureCategory, PrecipitationIntensity};
use std::collections::HashMap;

pub fn summarize_daily_totals(data: &[RawRecord]) -> Vec<DaySummary> {
    let mut daily_map: HashMap<String, DaySummary> = HashMap::new();

    for record in data {
        let date_key = record.date.clone();

        let weather = if record.snowfall > 0.0 {
            Weather::Snowy
        } else if record.rainfall > 0.0 {
            Weather::Rainy
        } else if record.visibility < 500.0 {
            Weather::Foggy
        } else {
            Weather::Sunny
        };

        let precipitation = if record.snowfall > 0.0 {
            if record.snowfall < 1.0 {
                PrecipitationIntensity::Light
            } else {
                PrecipitationIntensity::Heavy
            }
        } else {
            PrecipitationIntensity::None
        };

        let temperature_category = if record.temperature > 25.0 {
            TemperatureCategory::Hot
        } else if record.temperature < 10.0 {
            TemperatureCategory::Cold
        } else {
            TemperatureCategory::Moderate
        };

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

    daily_map.into_values().collect()
}
