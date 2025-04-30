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
