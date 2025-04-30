// This module defines data structures, structs, enums, and types utilized across different modules
use serde::Deserialize;
use std::fmt;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]

//This struct represents each record in the dataset with all the characteristics
pub struct RawRecord {
    #[serde(rename = "Date")]
    pub date: String,
    #[serde(rename = "Rented Bike Count")]
    pub rented_bike_count: i32,
    #[serde(rename = "Hour")]
    pub hour: u32,
    #[serde(rename = "Temperature (°C)")]
    pub temperature: f64,
    #[serde(rename = "Humidity (%)")]
    pub humidity: f64,
    #[serde(rename = "Wind speed (m/s)")]
    pub wind_speed: f64,
    #[serde(rename = "Visibility (10m)")]
    pub visibility: f64,
    #[serde(rename = "Dew point temperature(°C)")]
    pub dew_point_temp: f64,
    #[serde(rename = "Solar Radiation (MJ/m2)")]
    pub solar_radiation: f64,
    #[serde(rename = "Rainfall(mm)")]
    pub rainfall: f64,
    #[serde(rename = "Snowfall (cm)")]
    pub snowfall: f64,
    #[serde(rename = "Seasons")]
    pub seasons: String,
    #[serde(rename = "Holiday")]
    pub holiday: String,
    #[serde(rename = "Functioning Day")]
    pub functioning_day: String,

    #[serde(skip)]
    pub month: u32,
}


#[derive(Debug, Clone, PartialEq)]
//This enum Weather initializes the four possible weather labels
pub enum Weather {
    Sunny,
    Rainy,
    Snowy,
    Foggy,
}

impl fmt::Display for Weather {
    //matches enum and then writes its string representation
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Weather::Sunny => write!(f, "Sunny"),
            Weather::Rainy => write!(f, "Rainy"),
            Weather::Snowy => write!(f, "Snowy"),
            Weather::Foggy => write!(f, "Foggy"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
//This enum TemperatureCategory intialiazes different temperature groups for the raw records
pub enum TemperatureCategory {
    Hot,
    Moderate,
    Cold,
}

impl fmt::Display for TemperatureCategory {
    //matches the temperature category and then writes its string representation
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TemperatureCategory::Hot => write!(f, "Hot"),
            TemperatureCategory::Moderate => write!(f, "Moderate"),
            TemperatureCategory::Cold => write!(f, "Cold"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
//The enum pricipitation intensity initializes three possible precipitation groups
pub enum PrecipitationIntensity {
    Light,
    Heavy,
    None,
}

impl fmt::Display for PrecipitationIntensity {
    //This function matches its precipation intensity to the group and then writes a string representation
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PrecipitationIntensity::Light => write!(f, "Light"),
            PrecipitationIntensity::Heavy => write!(f, "Heavy"),
            PrecipitationIntensity::None => write!(f, "None"),
        }
    }
}

#[derive(Debug, Clone)]
//This struct lists out traits that will be inputed by the user
pub struct Conditions {
    pub month: u32,
    pub weather: Weather,
    pub temperature: TemperatureCategory,
    pub precipitation: PrecipitationIntensity,
}

#[derive(Debug, Clone)]
//This struct represents an aggregated data for a single calendar day
pub struct DaySummary {
    pub date: String,
    pub month: u32,
    pub total_rentals: i32,
    pub weather: Weather,
    pub temperature_category: TemperatureCategory,
    pub precipitation: PrecipitationIntensity,
}
