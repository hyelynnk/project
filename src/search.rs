//This module implements algorithms to predict bike rental count given weather conditions
use crate::common::{DaySummary, Conditions};

//The daily data is turned into a vector
pub fn make_graph(daily_data: &[DaySummary]) -> Vec<DaySummary> {
    daily_data.to_vec()
}

//The function for calculating similarity score is outlined. If the query and the day have essentially different values, the score goes up
//The higher the score is, the more that value is dissimilar from what we're interested in
fn similarity_score(day: &DaySummary, query: &Conditions) -> i32 {
    let mut score = 0;
    if day.month != query.month {
        score += 10;
    }
    if day.weather != query.weather {
        score += 30;
    }
    if day.temperature_category != query.temperature {
        score += 20;
    }
    if day.precipitation != query.precipitation {
        score += 15;
    }
    score
}

//Therefore, in order to find the closest in the bfs algorithm, we look for the min similarity score after iteration.
pub fn bfs_closest(query: &Conditions, graph: &[DaySummary]) -> Option<DaySummary> {
    graph.iter()
        .min_by_key(|day| similarity_score(day, query))
        .cloned()
}

//Same thing with finding the min by key is applied for dfs algorithm, but in the reverse order
pub fn dfs_closest(query: &Conditions, graph: &[DaySummary]) -> Option<DaySummary> {
    graph.iter()
        .rev()
        .min_by_key(|day| similarity_score(day, query))
        .cloned()
}

//The test aims to see if given two day summarys, the query correctly identifies the one that is closer
#[test]
fn test_bfs_finds_matching_day() {
    use crate::common::{Weather, TemperatureCategory, PrecipitationIntensity};
    let graph = vec![
        DaySummary {
            date: "01/01/2018".to_string(),
            total_rentals: 400,
            month: 1,
            weather: Weather::Sunny,
            temperature_category: TemperatureCategory::Cold,
            precipitation: PrecipitationIntensity::None,
        },
        DaySummary {
            date: "02/01/2018".to_string(),
            total_rentals: 500,
            month: 1,
            weather: Weather::Sunny,
            temperature_category: TemperatureCategory::Cold,
            precipitation: PrecipitationIntensity::None,
        }
    ];

    let query = Conditions {
        month: 1,
        weather: Weather::Sunny,
        temperature: TemperatureCategory::Cold,
        precipitation: PrecipitationIntensity::None,
    };

    let result = bfs_closest(&query, &graph);
    assert!(result.is_some());
    assert_eq!(result.unwrap().total_rentals, 400);
}
