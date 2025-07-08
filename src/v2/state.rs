// v2/state.rs
// State struct for v2 ecosystem simulation

use crate::v2::types::*;
use crate::v2::traits::*;
use crate::v2::errors::{EcosystemResult, CollapseReason};
use crate::v2::config::environment::EnvironmentConfig;

pub struct EcosystemStateV2 {
    pub plant_biomass: Biomass,
    pub microbe_pop: Population,
    pub worm_pop: Population,
    pub shrimp_pop: Population,
    pub soil_nitrogen: Nitrogen,
    pub soil_ph: Ph,
    pub soil_moisture: Moisture,
    pub soil_aeration: Aeration,
    pub detritus: Detritus,
    pub water_liters: WaterVolume,
    pub water_o2: Oxygen,
    pub air_n2: Nitrogen,
    pub air_o2: Oxygen,
    pub air_co2: CarbonDioxide,
    pub temperature: Temperature,
    pub humidity: Humidity,
    pub rocks: usize,
}

impl EcosystemStateV2 {
    #[allow(dead_code)]
    pub fn new(config: &crate::v2::config::V2Config) -> EcosystemResult<Self> {
        Ok(Self {
            plant_biomass: Biomass::new(config.organisms.plants.initial_biomass)?,
            microbe_pop: Population::new(config.organisms.microbes.initial_count as f32)?,
            worm_pop: Population::new(config.organisms.worms.initial_count as f32)?,
            shrimp_pop: Population::new(config.organisms.shrimp.initial_count as f32)?,
            soil_nitrogen: Nitrogen::new(1.0)?,
            soil_ph: Ph::new(7.0)?,
            soil_moisture: Moisture::new(config.environment.water_volume.value())?,
            soil_aeration: Aeration::new(1.0)?,
            detritus: Detritus::new(0.5)?,
            water_liters: config.environment.water_volume,
            water_o2: Oxygen::new(8.0)?,
            air_n2: Nitrogen::new(78.0)?,
            air_o2: Oxygen::new(21.0)?,
            air_co2: CarbonDioxide::new(0.04)?,
            temperature: config.environment.initial_temperature,
            humidity: config.environment.initial_humidity,
            rocks: config.environment.rocks,
        })
    }
    
    pub fn new_with_seed(config: &crate::v2::config::V2Config, seed: u64) -> EcosystemResult<Self> {
        use rand::{Rng, SeedableRng};
        use rand::rngs::StdRng;
        let mut rng = StdRng::seed_from_u64(seed);
        Ok(Self {
            plant_biomass: Biomass::new(config.organisms.plants.initial_biomass)?,
            microbe_pop: Population::new(rng.gen_range(500.0..=2000.0))?,
            worm_pop: Population::new(rng.gen_range(1.0..=10.0))?,
            shrimp_pop: Population::new(rng.gen_range(1.0..=5.0))?,
            soil_nitrogen: Nitrogen::new(rng.gen_range(0.5..=2.0))?,
            soil_ph: Ph::new(rng.gen_range(5.5..=8.5))?,
            soil_moisture: Moisture::new(rng.gen_range(0.2..=config.environment.water_volume.value()))?,
            soil_aeration: Aeration::new(rng.gen_range(0.5..=2.0))?,
            detritus: Detritus::new(rng.gen_range(0.1..=2.0))?,
            water_liters: config.environment.water_volume,
            water_o2: Oxygen::new(rng.gen_range(6.0..=10.0))?,
            air_n2: Nitrogen::new(78.0)?,
            air_o2: Oxygen::new(21.0)?,
            air_co2: CarbonDioxide::new(0.04)?,
            temperature: Temperature::new(rng.gen_range(18.0..=28.0))?,
            humidity: Humidity::new(rng.gen_range(40.0..=80.0))?,
            rocks: config.environment.rocks,
        })
    }
    
    pub fn light_level(&self) -> f32 {
        // This should be calculated based on window proximity from config
        // For now, default to a medium light level
        4.0
    }
    
    pub fn clamp_values(&mut self) -> EcosystemResult<()> {
        // Ensure all values are within valid ranges
        // Most clamping is handled by the type system now
        // Just update air composition
        let total_air = self.air_o2.percentage() + self.air_co2.value();
        let remaining_n2 = (100.0 - total_air).max(0.0);
        self.air_n2 = Nitrogen::new(remaining_n2)?;
        Ok(())
    }
}

// Implement traits for EcosystemStateV2
impl EcosystemDisplay for EcosystemStateV2 {
    fn display_status(&self) -> String {
        format!(
            "Plants: {:.1} | Microbes: {:.0} | Worms: {:.0} | Shrimp: {:.0} | pH: {:.1} | O2: {:.1}% | Temp: {:.1}°C",
            self.plant_biomass.value(),
            self.microbe_pop.value(),
            self.worm_pop.value(),
            self.shrimp_pop.value(),
            self.soil_ph.value(),
            self.air_o2.percentage(),
            self.temperature.celsius()
        )
    }
    
    fn display_summary(&self) -> String {
        format!(
            "Ecosystem Health: {}\n{}",
            if self.is_healthy() { "Stable" } else { "Stressed" },
            self.display_status()
        )
    }
    
    fn display_detailed(&self) -> String {
        format!(
            "=== Ecosystem State ===\n\
             Plants: {:.2} kg biomass\n\
             Microbes: {:.0} population\n\
             Worms: {:.0} population\n\
             Shrimp: {:.0} population\n\
             \n\
             Soil:\n\
             - Nitrogen: {:.2}\n\
             - pH: {:.2}\n\
             - Moisture: {:.2}\n\
             - Aeration: {:.2}\n\
             - Detritus: {:.2}\n\
             \n\
             Water:\n\
             - Volume: {:.2} L\n\
             - Oxygen: {:.1}%\n\
             \n\
             Air:\n\
             - Nitrogen: {:.1}%\n\
             - Oxygen: {:.1}%\n\
             - CO2: {:.3}%\n\
             \n\
             Environment:\n\
             - Temperature: {:.1}°C\n\
             - Humidity: {:.1}%\n\
             - Rocks: {}\n",
            self.plant_biomass.value(),
            self.microbe_pop.value(),
            self.worm_pop.value(),
            self.shrimp_pop.value(),
            self.soil_nitrogen.value(),
            self.soil_ph.value(),
            self.soil_moisture.value(),
            self.soil_aeration.value(),
            self.detritus.value(),
            self.water_liters.value(),
            self.water_o2.percentage(),
            self.air_n2.value(),
            self.air_o2.percentage(),
            self.air_co2.value(),
            self.temperature.celsius(),
            self.humidity.percentage(),
            self.rocks
        )
    }
}

impl EcosystemValidation for EcosystemStateV2 {
    fn validate(&self) -> EcosystemResult<()> {
        // All individual types handle their own validation
        // This just checks for ecosystem-level consistency
        Ok(())
    }
    
    fn is_healthy(&self) -> bool {
        !self.is_collapsed() && 
        self.collapse_risk() < 0.7 &&
        self.air_o2.percentage() > 15.0 &&
        (6.0..=8.0).contains(&self.soil_ph.value())
    }
    
    fn health_warnings(&self) -> Vec<String> {
        let mut warnings = Vec::new();
        
        if self.air_o2.is_dangerously_low() {
            warnings.push("Oxygen levels critically low".to_string());
        }
        
        if self.soil_ph.value() < 6.0 || self.soil_ph.value() > 8.5 {
            warnings.push("Soil pH outside optimal range".to_string());
        }
        
        if self.plant_biomass.value() < 0.1 {
            warnings.push("Plant biomass very low".to_string());
        }
        
        if self.microbe_pop.value() < 100.0 {
            warnings.push("Microbe population critically low".to_string());
        }
        
        warnings
    }
}

impl CollapseDetection for EcosystemStateV2 {
    fn is_collapsed(&self) -> bool {
        self.plant_biomass.is_collapsed() ||
        self.microbe_pop.is_collapsed() ||
        self.worm_pop.is_collapsed() ||
        self.shrimp_pop.is_collapsed()
    }
    
    fn collapse_risk(&self) -> f32 {
        let mut risk_factors = Vec::new();
        
        // Population risks
        if self.plant_biomass.value() < 0.1 { risk_factors.push(0.8); }
        if self.microbe_pop.value() < 100.0 { risk_factors.push(0.9); }
        if self.worm_pop.value() < 1.0 { risk_factors.push(0.6); }
        if self.shrimp_pop.value() < 1.0 { risk_factors.push(0.5); }
        
        // Environmental risks
        if self.air_o2.is_dangerously_low() { risk_factors.push(0.9); }
        if self.soil_ph.value() < 5.5 || self.soil_ph.value() > 9.0 { risk_factors.push(0.8); }
        if !self.temperature.is_optimal() { risk_factors.push(0.3); }
        
        // Calculate overall risk (max of individual risks, but capped)
        risk_factors.iter().fold(0.0, |acc, &risk| acc.max(risk))
    }
    
    fn collapse_reasons(&self) -> Vec<CollapseReason> {
        let mut reasons = Vec::new();
        
        if self.plant_biomass.is_collapsed() {
            reasons.push(CollapseReason::PlantsDied);
        }
        if self.microbe_pop.is_collapsed() {
            reasons.push(CollapseReason::MicrobesDied);
        }
        if self.worm_pop.is_collapsed() {
            reasons.push(CollapseReason::WormsDied);
        }
        if self.shrimp_pop.is_collapsed() {
            reasons.push(CollapseReason::ShrimpDied);
        }
        if self.air_o2.is_dangerously_low() {
            reasons.push(CollapseReason::OxygenDepletion);
        }
        if self.soil_ph.value() < 4.0 || self.soil_ph.value() > 10.0 {
            reasons.push(CollapseReason::PhImbalance);
        }
        
        reasons
    }
}

impl EcosystemMonitoring for EcosystemStateV2 {
    fn key_metrics(&self) -> Vec<(String, f32)> {
        vec![
            ("Plant Biomass".to_string(), self.plant_biomass.value()),
            ("Microbe Population".to_string(), self.microbe_pop.value()),
            ("Worm Population".to_string(), self.worm_pop.value()),
            ("Shrimp Population".to_string(), self.shrimp_pop.value()),
            ("Soil pH".to_string(), self.soil_ph.value()),
            ("Air Oxygen".to_string(), self.air_o2.percentage()),
            ("Temperature".to_string(), self.temperature.celsius()),
            ("Humidity".to_string(), self.humidity.percentage()),
            ("Soil Nitrogen".to_string(), self.soil_nitrogen.value()),
            ("Water Oxygen".to_string(), self.water_o2.percentage()),
        ]
    }
    
    fn trend_indicators(&self) -> Vec<TrendIndicator> {
        // This would need historical data to implement properly
        // For now, return empty vec
        Vec::new()
    }
    
    fn alert_conditions(&self) -> Vec<AlertCondition> {
        let mut alerts = Vec::new();
        
        if self.air_o2.is_dangerously_low() {
            alerts.push(AlertCondition {
                severity: AlertSeverity::Emergency,
                message: "Oxygen levels critically low".to_string(),
                parameter: "air_oxygen".to_string(),
                current_value: self.air_o2.percentage(),
                threshold: 5.0,
            });
        }
        
        if self.soil_ph.value() < 6.0 {
            alerts.push(AlertCondition {
                severity: AlertSeverity::Warning,
                message: "Soil becoming acidic".to_string(),
                parameter: "soil_ph".to_string(),
                current_value: self.soil_ph.value(),
                threshold: 6.0,
            });
        }
        
        if self.plant_biomass.value() < 0.1 {
            alerts.push(AlertCondition {
                severity: AlertSeverity::Critical,
                message: "Plant biomass very low".to_string(),
                parameter: "plant_biomass".to_string(),
                current_value: self.plant_biomass.value(),
                threshold: 0.1,
            });
        }
        
        alerts
    }
}
