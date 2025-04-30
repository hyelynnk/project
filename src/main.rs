mod data;
mod model;
mod search;
mod common;

use std::io::{self, Write};
use std::process;
use std::collections::HashMap;

use common::{Weather, TemperatureCategory, PrecipitationIntensity,Conditions};
use model::summarize_daily_totals;
use search::{build_graph, bfs_closest, dfs_closest};

fn get_user_input() -> Conditions {
    let mut input = String::new();

    print!("Enter month (ex. 1): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let month: u32 = match input.trim().parse() {
        Ok(m) if m >= 1 && m <= 12 => m,
        _ => {
            println!("Invalid");
            0
        }
    };

    input.clear();
    print!("Enter weather (sunny, rainy, snowy, foggy): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let weather = match input.trim().to_lowercase().as_str() {
        "sunny" => Weather::Sunny,
        "rainy" => Weather::Rainy,
        "snowy" => Weather::Snowy,
        "foggy" => Weather::Foggy,
        _ => {
            println!("Invalid weather");
            Weather::Sunny
        }
    };

    let mut precipitation = PrecipitationIntensity::None;
    if matches!(weather, Weather::Rainy | Weather::Snowy) {
        input.clear();
        print!("Was it light or heavy {}? (light/heavy): ", if weather == Weather::Rainy { "rain" } else { "snow" });
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        precipitation = match input.trim().to_lowercase().as_str() {
            "light" => PrecipitationIntensity::Light,
            "heavy" => PrecipitationIntensity::Heavy,
            _ => {
                println!("Invalid intensity");
                PrecipitationIntensity::Light
            }
        };
    }

    input.clear();
    print!("Enter temperature (hot, moderate, cold): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let temperature = match input.trim().to_lowercase().as_str() {
        "hot" => TemperatureCategory::Hot,
        "moderate" => TemperatureCategory::Moderate,
        "cold" => TemperatureCategory::Cold,
        _ => {
            println!("Invalid temperature");
            TemperatureCategory::Moderate
        }
    };

    Conditions {
        month,
        weather,
        temperature,
        precipitation,
    }
}

fn main() {
    let raw_data = match data::load_data("SeoulBikeData 4.csv") {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Failed to load CSV: {}", e);
            process::exit(1);
        }
    };
    
    let daily_summaries = summarize_daily_totals(&raw_data);
    let graph = build_graph(&daily_summaries);

    let mut season_totals: HashMap<String, (i32, usize)> = HashMap::new();
    let mut weather_totals: HashMap<String, (i32, usize)> = HashMap::new();
    let mut temp_totals: HashMap<String, (i32, usize)> = HashMap::new();
    let mut rain_totals: HashMap<String, (i32, usize)> = HashMap::new();
    let mut wind_high = (0, 0);
    let mut wind_low = (0, 0);

    for day in &daily_summaries {

        let season = match day.month {
            3..=5 => "Spring",
            6..=8 => "Summer",
            9..=11 => "Fall",
            _ => "Winter",
        };

        season_totals.entry(season.to_string())
            .and_modify(|e| { e.0 += day.total_rentals; e.1 += 1 })
            .or_insert((day.total_rentals, 1));

        weather_totals.entry(format!("{}", day.weather))
            .and_modify(|e| { e.0 += day.total_rentals; e.1 += 1 })
            .or_insert((day.total_rentals, 1));

        temp_totals.entry(format!("{}", day.temperature_category))
            .and_modify(|e| { e.0 += day.total_rentals; e.1 += 1 })
            .or_insert((day.total_rentals, 1));

        if matches!(day.weather, Weather::Rainy | Weather::Snowy) {
            rain_totals.entry(format!("{}", day.precipitation))
                .and_modify(|e| { e.0 += day.total_rentals; e.1 += 1 })
                .or_insert((day.total_rentals, 1));
        }

        if day.weather == Weather::Foggy {
            wind_high.0 += day.total_rentals;
            wind_high.1 += 1;
        } else {
            wind_low.0 += day.total_rentals;
            wind_low.1 += 1;
        }
    }

    println!("[Seasonal average bike demand]");
    let season_order = ["Spring", "Summer", "Fall", "Winter"];
    for season in &season_order {
        if let Some((total, count)) = season_totals.get(*season) {
            let avg = *total as f32 / *count as f32;
            println!("  {}: {:.0} bikes", season, avg);
        }
    }

    println!();
    println!("[Average daily demand by Weather condition]");
    for (weather, (total, count)) in &weather_totals {
        let avg = *total as f32 / *count as f32;
        println!("  {}: {:.0} bikes", weather, avg);
    }

    println!();
    println!("[Average daily demand by temperature category]");
    for (temp, (total, count)) in &temp_totals {
        let avg = *total as f32 / *count as f32;
        println!("  {}: {:.0} bikes", temp, avg);
    }

    println!();
    println!("Use conditions for prediction:");
    let query = get_user_input();

    println!();
    println!("[Finding the closest match]");
    println!("  Month: {}", query.month);
    println!("  Weather: {}", query.weather);
    println!("  Temperature: {}", query.temperature);
    println!("  Precipitation: {}", query.precipitation);

    if let Some(day) = bfs_closest(&query, &graph) {
        println!("[BFS] Closest match: {} - {} rentals", day.date, day.total_rentals);
    } else {
        println!("[BFS] No matching day found.");
    }

    if let Some(day) = dfs_closest(&query, &graph) {
        println!("[DFS] Closest match: {} - {} rentals", day.date, day.total_rentals);
    } else {
        println!("[DFS] No matching day found.");
    }
}
