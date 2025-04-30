//This module implements algorithms to predict bike rental count given weather conditions
use crate::common::{DaySummary, Conditions};
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug)]
pub struct Graph {
    pub adj_list: HashMap<usize, Vec<usize>>,
}

// This functions checks if two days are similar enough to connect in the graph
fn are_similar(a: &DaySummary, b: &DaySummary) -> bool {
    a.weather == b.weather &&
    a.temperature_category == b.temperature_category &&
    a.precipitation == b.precipitation
}

// This function builds the graph by connecting similar nodes
pub fn build_graph(days: &[DaySummary]) -> Graph {
    let mut adj_list: HashMap<usize, Vec<usize>> = HashMap::new();
    for (i, day_i) in days.iter().enumerate() {
        for (j, day_j) in days.iter().enumerate() {
            if i != j && are_similar(day_i, day_j) {
                adj_list.entry(i).or_default().push(j);
            }
        }
    }
    Graph { adj_list }
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

// This function utilizes bfs algorithm, and we look for the min similarity score after iteration
pub fn bfs_closest(start: usize, graph: &Graph, days: &[DaySummary], query: &Conditions) -> Option<DaySummary> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut best_match = None;
    let mut best_score = i32::MAX;

    queue.push_back(start);
    visited.insert(start);

    while let Some(node) = queue.pop_front() {
        let day = &days[node];
        let score = similarity_score(day, query);

        if score < best_score {
            best_score = score;
            best_match = Some(day.clone());
        }

        if let Some(neighbors) = graph.adj_list.get(&node) {
            for &neighbor in neighbors {
                if visited.insert(neighbor) {
                    queue.push_back(neighbor);
                }
            }
        }
    }

    best_match
}

// Same thing with finding the min by key is applied for dfs algorithm, but in the reverse order
pub fn dfs_closest(start: usize, graph: &Graph, days: &[DaySummary], query: &Conditions) -> Option<DaySummary> {
    let mut visited = HashSet::new();
    let mut stack = Vec::new();
    let mut best_match = None;
    let mut best_score = i32::MAX;

    stack.push(start);
    visited.insert(start);

    while let Some(node) = stack.pop() {
        let day = &days[node];
        let score = similarity_score(day, query);

        if score < best_score {
            best_score = score;
            best_match = Some(day.clone());
        }

        if let Some(neighbors) = graph.adj_list.get(&node) {
            for &neighbor in neighbors {
                if visited.insert(neighbor) {
                    stack.push(neighbor);
                }
            }
        }
    }

    best_match
}


#[test]
//The test aims to see if given two day summarys, the query correctly identifies the one that is closer
fn test_bfs_finds_matching_day() {
    let graph_data = vec![
        DaySummary {
            date: "01/01/2018".to_string(),
            total_rentals: 400,
            month: 1,
            weather: crate::common::Weather::Sunny,
            temperature_category: crate::common::TemperatureCategory::Cold,
            precipitation: crate::common::PrecipitationIntensity::None,
        },
        DaySummary {
            date: "02/01/2018".to_string(),
            total_rentals: 500,
            month: 1,
            weather: crate::common::Weather::Sunny,
            temperature_category: crate::common::TemperatureCategory::Cold,
            precipitation: crate::common::PrecipitationIntensity::None,
        }
    ];

    let query = Conditions {
        month: 1,
        weather: crate::common::Weather::Sunny,
        temperature: crate::common::TemperatureCategory::Cold,
        precipitation: crate::common::PrecipitationIntensity::None,
    };

    let graph = build_graph(&graph_data);
    let result = bfs_closest(0, &graph, &graph_data, &query);
    assert!(result.is_some());
    assert_eq!(result.unwrap().total_rentals, 400);
}
