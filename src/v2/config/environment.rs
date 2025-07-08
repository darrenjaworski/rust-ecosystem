// v2/config/environment.rs
// Environmental configuration for ecosystem simulation

use crate::v2::errors::{EcosystemError, EcosystemResult};
use crate::v2::types::*;

#[derive(Debug, Clone)]
pub struct EnvironmentConfig {
    pub water_volume: WaterVolume,
    pub rocks: usize,
    pub window_proximity: u8,
    pub initial_temperature: Temperature,
    pub initial_humidity: Humidity,
    pub soil_type: SoilType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SoilType {
    Porous,
    NonPorous,
    Balanced,
}

impl SoilType {
    pub fn aeration_modifier(&self) -> f32 {
        match self {
            SoilType::Porous => 1.5,
            SoilType::NonPorous => 0.7,
            SoilType::Balanced => 1.0,
        }
    }

    pub fn water_retention_modifier(&self) -> f32 {
        match self {
            SoilType::Porous => 0.8,
            SoilType::NonPorous => 1.3,
            SoilType::Balanced => 1.0,
        }
    }

    pub fn microbial_growth_modifier(&self) -> f32 {
        match self {
            SoilType::Porous => 1.2,
            SoilType::NonPorous => 0.9,
            SoilType::Balanced => 1.0,
        }
    }
}

impl EnvironmentConfig {
    pub fn new(
        water_volume: f32,
        rocks: usize,
        window_proximity: u8,
        initial_temperature: f32,
        initial_humidity: f32,
        soil_type: SoilType,
    ) -> EcosystemResult<Self> {
        Ok(Self {
            water_volume: WaterVolume::new(water_volume)?,
            rocks,
            window_proximity,
            initial_temperature: Temperature::new(initial_temperature)?,
            initial_humidity: Humidity::new(initial_humidity)?,
            soil_type,
        })
    }

    pub fn validate(&self) -> EcosystemResult<()> {
        if self.window_proximity > 6 {
            return Err(EcosystemError::ConfigurationError {
                message: format!("Window proximity {} must be 0-6", self.window_proximity),
            });
        }

        if self.rocks > 10 {
            return Err(EcosystemError::ConfigurationError {
                message: format!("Too many rocks: {} (max 10)", self.rocks),
            });
        }

        Ok(())
    }

    pub fn light_level(&self) -> f32 {
        (6 - self.window_proximity) as f32
    }

    pub fn is_near_window(&self) -> bool {
        self.window_proximity <= 2
    }

    pub fn has_good_light(&self) -> bool {
        self.light_level() >= 4.0
    }
}

impl Default for EnvironmentConfig {
    fn default() -> Self {
        Self {
            water_volume: WaterVolume::new(0.5).unwrap(),
            rocks: 2,
            window_proximity: 3,
            initial_temperature: Temperature::new(22.0).unwrap(),
            initial_humidity: Humidity::new(60.0).unwrap(),
            soil_type: SoilType::Balanced,
        }
    }
}