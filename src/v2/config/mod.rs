// v2/config/mod.rs
// Configuration management for v2 ecosystem simulation

pub mod difficulty;
pub mod environment;
pub mod organisms;
pub mod parameters;

use crate::v2::errors::{EcosystemError, EcosystemResult};
use crate::v2::types::*;

#[derive(Debug, Clone)]
pub struct V2Config {
    pub organisms: organisms::OrganismConfig,
    pub environment: environment::EnvironmentConfig,
    pub parameters: parameters::SimulationParameters,
    pub difficulty: difficulty::DifficultyConfig,
}

impl V2Config {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_difficulty(difficulty: f32) -> EcosystemResult<Self> {
        let mut config = Self::default();
        config.difficulty = difficulty::DifficultyConfig::new(difficulty)?;
        config.parameters.apply_difficulty(&config.difficulty);
        Ok(config)
    }

    pub fn validate(&self) -> EcosystemResult<()> {
        self.organisms.validate()?;
        self.environment.validate()?;
        self.parameters.validate()?;
        self.difficulty.validate()?;
        Ok(())
    }

    pub fn window_proximity(&self) -> u8 {
        self.environment.window_proximity
    }

    pub fn water_liters(&self) -> f32 {
        self.environment.water_volume.value()
    }

    pub fn initial_temp(&self) -> f32 {
        self.environment.initial_temperature.celsius()
    }

    pub fn initial_humidity(&self) -> f32 {
        self.environment.initial_humidity.percentage()
    }

    pub fn rocks(&self) -> usize {
        self.environment.rocks
    }

    pub fn num_microbes(&self) -> usize {
        self.organisms.microbes.initial_count
    }

    pub fn num_worms(&self) -> usize {
        self.organisms.worms.initial_count
    }

    pub fn num_shrimp(&self) -> usize {
        self.organisms.shrimp.initial_count
    }
}

impl Default for V2Config {
    fn default() -> Self {
        Self {
            organisms: organisms::OrganismConfig::default(),
            environment: environment::EnvironmentConfig::default(),
            parameters: parameters::SimulationParameters::default(),
            difficulty: difficulty::DifficultyConfig::default(),
        }
    }
}

// Legacy compatibility - keep the old struct for backwards compatibility
#[deprecated(note = "Use V2Config instead")]
pub struct LegacyV2Config {
    pub num_microbes: usize,
    pub num_worms: usize,
    pub num_shrimp: usize,
    pub water_liters: f32,
    pub rocks: usize,
    pub window_proximity: u8,
    pub initial_temp: f32,
    pub initial_humidity: f32,
}

impl From<V2Config> for LegacyV2Config {
    fn from(config: V2Config) -> Self {
        Self {
            num_microbes: config.organisms.microbes.initial_count,
            num_worms: config.organisms.worms.initial_count,
            num_shrimp: config.organisms.shrimp.initial_count,
            water_liters: config.environment.water_volume.value(),
            rocks: config.environment.rocks,
            window_proximity: config.environment.window_proximity,
            initial_temp: config.environment.initial_temperature.celsius(),
            initial_humidity: config.environment.initial_humidity.percentage(),
        }
    }
}

impl From<LegacyV2Config> for V2Config {
    fn from(legacy: LegacyV2Config) -> Self {
        Self {
            organisms: organisms::OrganismConfig {
                microbes: organisms::MicrobeConfig { initial_count: legacy.num_microbes },
                worms: organisms::WormConfig { initial_count: legacy.num_worms },
                shrimp: organisms::ShrimpConfig { initial_count: legacy.num_shrimp },
                plants: organisms::PlantConfig { initial_biomass: 1.0 },
            },
            environment: environment::EnvironmentConfig {
                water_volume: WaterVolume::new(legacy.water_liters).unwrap_or_else(|_| WaterVolume::new(0.5).unwrap()),
                rocks: legacy.rocks,
                window_proximity: legacy.window_proximity,
                initial_temperature: Temperature::new(legacy.initial_temp).unwrap_or_else(|_| Temperature::new(22.0).unwrap()),
                initial_humidity: Humidity::new(legacy.initial_humidity).unwrap_or_else(|_| Humidity::new(60.0).unwrap()),
                soil_type: environment::SoilType::Balanced,
            },
            parameters: parameters::SimulationParameters::default(),
            difficulty: difficulty::DifficultyConfig::default(),
        }
    }
}