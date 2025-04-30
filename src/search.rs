use crate::common::{DaySummary, Conditions};

pub fn build_graph(daily_data: &[DaySummary]) -> Vec<DaySummary> {
    daily_data.to_vec()
}

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

pub fn bfs_closest(query: &Conditions, graph: &[DaySummary]) -> Option<DaySummary> {
    graph.iter()
        .min_by_key(|day| similarity_score(day, query))
        .cloned()
}

pub fn dfs_closest(query: &Conditions, graph: &[DaySummary]) -> Option<DaySummary> {
    graph.iter()
        .rev()
        .min_by_key(|day| similarity_score(day, query))
        .cloned()
}

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
