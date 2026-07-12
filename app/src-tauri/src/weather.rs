//! Real weather (Open-Meteo — free, keyless). Independent of Inner Weather:
//! this is what the sky actually shows outside; Inner Weather is the manual
//! mood layer that changes how the brush moves. The two never mix.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherSnapshot {
    pub fetched_at: i64,
    pub temperature_c: f64,
    pub weather_code: i64,
    pub wind_speed_kmh: f64,
    pub precipitation_mm: f64,
    pub is_day: bool,
    /// True when this snapshot is an offline fallback rather than fresh.
    #[serde(default)]
    pub stale: bool,
}

#[derive(Deserialize)]
struct OpenMeteoResponse {
    current: OpenMeteoCurrent,
}

#[derive(Deserialize)]
struct OpenMeteoCurrent {
    temperature_2m: f64,
    weather_code: i64,
    wind_speed_10m: f64,
    precipitation: f64,
    is_day: i64,
}

pub async fn fetch(lat: f64, lon: f64) -> Result<WeatherSnapshot, String> {
    let url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={lat}&longitude={lon}&current=temperature_2m,weather_code,wind_speed_10m,precipitation,is_day&timezone=auto"
    );
    let resp = reqwest::get(&url)
        .await
        .map_err(|e| format!("weather request failed: {e}"))?
        .error_for_status()
        .map_err(|e| format!("weather request failed: {e}"))?
        .json::<OpenMeteoResponse>()
        .await
        .map_err(|e| format!("weather response parse failed: {e}"))?;

    Ok(WeatherSnapshot {
        fetched_at: chrono::Utc::now().timestamp(),
        temperature_c: resp.current.temperature_2m,
        weather_code: resp.current.weather_code,
        wind_speed_kmh: resp.current.wind_speed_10m,
        precipitation_mm: resp.current.precipitation,
        is_day: resp.current.is_day != 0,
        stale: false,
    })
}
