/*!
# cuda-weather

Environmental sensing and terrain analysis.

Before an agent can navigate, it needs to understand its environment.
Is it hot? Cold? Dark? Noisy? This crate turns raw environmental
readings into actionable situation awareness.

- Temperature monitoring with thermal zones
- Humidity and moisture tracking
- Light level analysis (day/night/twilight)
- Noise level monitoring
- Terrain classification
- Weather condition inference
- Environmental comfort index
*/

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Environmental reading
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EnvReading {
    pub temperature_c: f64,
    pub humidity_pct: f64,
    pub light_lux: f64,
    pub noise_db: f64,
    pub pressure_hpa: f64,
    pub timestamp: u64,
}

/// Thermal zone classification
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThermalZone { Freezing, Cold, Cool, Comfortable, Warm, Hot, Extreme }

impl ThermalZone {
    pub fn from_celsius(c: f64) -> Self {
        if c < 0.0 { ThermalZone::Freezing }
        else if c < 10.0 { ThermalZone::Cold }
        else if c < 18.0 { ThermalZone::Cool }
        else if c < 26.0 { ThermalZone::Comfortable }
        else if c < 35.0 { ThermalZone::Warm }
        else if c < 45.0 { ThermalZone::Hot }
        else { ThermalZone::Extreme }
    }
}

/// Light condition
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum LightCondition { Dark, Dim, Twilight, Bright, Glare }

impl LightCondition {
    pub fn from_lux(lux: f64) -> Self {
        if lux < 10.0 { LightCondition::Dark }
        else if lux < 100.0 { LightCondition::Dim }
        else if lux < 1000.0 { LightCondition::Twilight }
        else if lux < 10000.0 { LightCondition::Bright }
        else { LightCondition::Glare }
    }
}

/// Noise level
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum NoiseLevel { Silent, Quiet, Moderate, Loud, Deafening }

impl NoiseLevel {
    pub fn from_db(db: f64) -> Self {
        if db < 20.0 { NoiseLevel::Silent }
        else if db < 40.0 { NoiseLevel::Quiet }
        else if db < 70.0 { NoiseLevel::Moderate }
        else if db < 100.0 { NoiseLevel::Loud }
        else { NoiseLevel::Deafening }
    }
}

/// Terrain type
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TerrainType { Flat, Sloped, Rough, Obstacle, Water, Unknown }

/// Weather condition
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum WeatherCondition { Clear, Cloudy, Rain, Storm, Fog, Snow, Windy }

/// Environmental assessment
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EnvAssessment {
    pub thermal: ThermalZone,
    pub light: LightCondition,
    pub noise: NoiseLevel,
    pub weather: WeatherCondition,
    pub comfort_index: f64,    // 0-1, how comfortable
    pub hazard_flags: Vec<String>,
    pub terrain: TerrainType,
}

/// Environmental history buffer
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EnvHistory {
    pub readings: Vec<EnvReading>,
    pub max_size: usize,
}

impl EnvHistory {
    pub fn new(max_size: usize) -> Self { EnvHistory { readings: Vec::with_capacity(max_size), max_size } }

    pub fn record(&mut self, reading: EnvReading) {
        if self.readings.len() >= self.max_size { self.readings.remove(0); }
        self.readings.push(reading);
    }

    pub fn avg_temperature(&self) -> f64 {
        if self.readings.is_empty() { return 20.0; }
        self.readings.iter().map(|r| r.temperature_c).sum::<f64>() / self.readings.len() as f64
    }

    pub fn trend_temperature(&self) -> f64 {
        if self.readings.len() < 5 { return 0.0; }
        let first_half: f64 = self.readings[..self.readings.len()/2].iter().map(|r| r.temperature_c).sum::<f64>() / (self.readings.len()/2) as f64;
        let second_half: f64 = self.readings[self.readings.len()/2..].iter().map(|r| r.temperature_c).sum::<f64>() / (self.readings.len() - self.readings.len()/2) as f64;
        second_half - first_half
    }

    pub fn is_stable(&self, threshold: f64) -> bool {
        if self.readings.is_empty() { return true; }
        let avg = self.avg_temperature();
        self.readings.iter().all(|r| (r.temperature_c - avg).abs() < threshold)
    }
}

/// The weather/environment system
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EnvironmentMonitor {
    pub history: EnvHistory,
    pub current: Option<EnvReading>,
    pub terrain_map: HashMap<(i32, i32), TerrainType>,
    pub alert_thresholds: AlertThresholds,
    pub alerts: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AlertThresholds {
    pub max_temp_c: f64,
    pub min_temp_c: f64,
    pub max_noise_db: f64,
    pub min_light_lux: f64,
}

impl Default for AlertThresholds {
    fn default() -> Self { AlertThresholds { max_temp_c: 50.0, min_temp_c: -10.0, max_noise_db: 90.0, min_light_lux: 5.0 } }
}

impl EnvironmentMonitor {
    pub fn new() -> Self { EnvironmentMonitor { history: EnvHistory::new(100), current: None, terrain_map: HashMap::new(), alert_thresholds: AlertThresholds::default(), alerts: vec![] } }

    /// Record a reading and assess
    pub fn sense(&mut self, reading: EnvReading) -> EnvAssessment {
        self.history.record(reading.clone());
        self.current = Some(reading.clone());

        let thermal = ThermalZone::from_celsius(reading.temperature_c);
        let light = LightCondition::from_lux(reading.light_lux);
        let noise = NoiseLevel::from_db(reading.noise_db);
        let weather = Self::infer_weather(&reading);
        let mut hazard_flags = vec![];

        // Comfort index
        let temp_comfort = 1.0 - ((reading.temperature_c - 22.0).abs() / 30.0).min(1.0);
        let noise_comfort = 1.0 - (reading.noise_db / 100.0).min(1.0);
        let comfort = temp_comfort * 0.6 + noise_comfort * 0.4;

        // Alerts
        if reading.temperature_c > self.alert_thresholds.max_temp_c { hazard_flags.push("overheating".into()); }
        if reading.temperature_c < self.alert_thresholds.min_temp_c { hazard_flags.push("freezing".into()); }
        if reading.noise_db > self.alert_thresholds.max_noise_db { hazard_flags.push("loud_noise".into()); }
        if reading.light_lux < self.alert_thresholds.min_light_lux { hazard_flags.push("too_dark".into()); }

        EnvAssessment { thermal, light, noise, weather, comfort_index: comfort, hazard_flags, terrain: TerrainType::Unknown }
    }

    /// Infer weather from readings
    fn infer_weather(reading: &EnvReading) -> WeatherCondition {
        if reading.humidity_pct > 90.0 && reading.light_lux < 500.0 { WeatherCondition::Storm }
        else if reading.humidity_pct > 70.0 { WeatherCondition::Rain }
        else if reading.humidity_pct > 50.0 && reading.light_lux < 2000.0 { WeatherCondition::Fog }
        else if reading.light_lux > 10000.0 && reading.temperature_c > 30.0 { WeatherCondition::Clear }
        else if reading.pressure_hpa < 1000.0 { WeatherCondition::Windy }
        else { WeatherCondition::Cloudy }
    }

    /// Set terrain at grid cell
    pub fn set_terrain(&mut self, x: i32, y: i32, terrain: TerrainType) {
        self.terrain_map.insert((x, y), terrain);
    }

    /// Get terrain at grid cell
    pub fn get_terrain(&self, x: i32, y: i32) -> TerrainType {
        self.terrain_map.get(&(x, y)).copied().unwrap_or(TerrainType::Unknown)
    }

    /// Is position traversable?
    pub fn is_traversable(&self, x: i32, y: i32) -> bool {
        let t = self.get_terrain(x, y);
        t != TerrainType::Obstacle && t != TerrainType::Water
    }

    /// Summary
    pub fn summary(&self) -> String {
        let temp = self.current.map(|r| format!("{:.1}C", r.temperature_c)).unwrap_or_else(|| "N/A".into());
        format!("Environment: temp={}, history={} readings, terrain_cells={}, alerts={}", temp, self.history.readings.len(), self.terrain_map.len(), self.alerts.len())
    }
}

fn now() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_millis() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_reading() -> EnvReading { EnvReading { temperature_c: 22.0, humidity_pct: 45.0, light_lux: 500.0, noise_db: 35.0, pressure_hpa: 1013.0, timestamp: now() } }

    #[test]
    fn test_thermal_zones() {
        assert_eq!(ThermalZone::from_celsius(-5.0), ThermalZone::Freezing);
        assert_eq!(ThermalZone::from_celsius(5.0), ThermalZone::Cold);
        assert_eq!(ThermalZone::from_celsius(20.0), ThermalZone::Comfortable);
        assert_eq!(ThermalZone::from_celsius(50.0), ThermalZone::Hot);
    }

    #[test]
    fn test_light_conditions() {
        assert_eq!(LightCondition::from_lux(1.0), LightCondition::Dark);
        assert_eq!(LightCondition::from_lux(500.0), LightCondition::Twilight);
        assert_eq!(LightCondition::from_lux(5000.0), LightCondition::Bright);
    }

    #[test]
    fn test_noise_levels() {
        assert_eq!(NoiseLevel::from_db(10.0), NoiseLevel::Silent);
        assert_eq!(NoiseLevel::from_db(50.0), NoiseLevel::Moderate);
        assert_eq!(NoiseLevel::from_db(110.0), NoiseLevel::Deafening);
    }

    #[test]
    fn test_sense_and_assess() {
        let mut mon = EnvironmentMonitor::new();
        let assessment = mon.sense(sample_reading());
        assert_eq!(assessment.thermal, ThermalZone::Comfortable);
        assert!(assessment.comfort_index > 0.5);
        assert!(assessment.hazard_flags.is_empty());
    }

    #[test]
    fn test_hot_alert() {
        let mut mon = EnvironmentMonitor::new();
        let mut reading = sample_reading();
        reading.temperature_c = 60.0;
        let assessment = mon.sense(reading);
        assert!(assessment.hazard_flags.contains(&"overheating".to_string()));
    }

    #[test]
    fn test_dark_alert() {
        let mut mon = EnvironmentMonitor::new();
        let mut reading = sample_reading();
        reading.light_lux = 1.0;
        let assessment = mon.sense(reading);
        assert!(assessment.hazard_flags.contains(&"too_dark".to_string()));
    }

    #[test]
    fn test_weather_inference() {
        let mut reading = sample_reading();
        reading.humidity_pct = 95.0;
        reading.light_lux = 100.0;
        let weather = EnvironmentMonitor::infer_weather(&reading);
        assert_eq!(weather, WeatherCondition::Storm);
    }

    #[test]
    fn test_terrain_map() {
        let mut mon = EnvironmentMonitor::new();
        mon.set_terrain(0, 0, TerrainType::Flat);
        mon.set_terrain(1, 0, TerrainType::Obstacle);
        mon.set_terrain(2, 0, TerrainType::Water);
        assert!(mon.is_traversable(0, 0));
        assert!(!mon.is_traversable(1, 0));
        assert!(!mon.is_traversable(2, 0));
    }

    #[test]
    fn test_history_avg() {
        let mut mon = EnvironmentMonitor::new();
        for i in 0..10 {
            let mut r = sample_reading();
            r.temperature_c = 20.0 + i as f64;
            mon.sense(r);
        }
        assert!((mon.history.avg_temperature() - 24.5).abs() < 0.01);
    }

    #[test]
    fn test_history_trend() {
        let mut mon = EnvironmentMonitor::new();
        for i in 0..10 {
            let mut r = sample_reading();
            r.temperature_c = 10.0 + i as f64;
            mon.sense(r);
        }
        assert!(mon.history.trend_temperature() > 0.0);
    }

    #[test]
    fn test_summary() {
        let mon = EnvironmentMonitor::new();
        let s = mon.summary();
        assert!(s.contains("N/A"));
    }
}
