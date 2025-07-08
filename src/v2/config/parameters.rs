// v2/config/parameters.rs
// Simulation parameters for ecosystem modeling

use crate::v2::config::difficulty::DifficultyConfig;
use crate::v2::errors::{EcosystemError, EcosystemResult};

#[derive(Debug, Clone)]
pub struct SimulationParameters {
    pub photosynthesis: PhotosynthesisParams,
    pub respiration: RespirationParams,
    pub microbial: MicrobialParams,
    pub worm: WormParams,
    pub shrimp: ShrimpParams,
    pub environmental: EnvironmentalParams,
}

#[derive(Debug, Clone)]
pub struct PhotosynthesisParams {
    pub base_rate: f32,
    pub co2_efficiency: f32,
    pub light_dependency: f32,
    pub humidity_dependency: f32,
}

#[derive(Debug, Clone)]
pub struct RespirationParams {
    pub base_rate: f32,
    pub co2_production: f32,
}

#[derive(Debug, Clone)]
pub struct MicrobialParams {
    pub nitrogen_fixation_rate: f32,
    pub growth_rate: f32,
    pub death_rate: f32,
    pub respiration_rate: f32,
    pub respiration_co2_ratio: f32,
}

#[derive(Debug, Clone)]
pub struct WormParams {
    pub aeration_rate: f32,
    pub decomposition_rate: f32,
    pub growth_rate: f32,
    pub death_rate: f32,
}

#[derive(Debug, Clone)]
pub struct ShrimpParams {
    pub detritus_consumption_rate: f32,
    pub waste_production_rate: f32,
    pub growth_rate: f32,
    pub death_rate: f32,
}

#[derive(Debug, Clone)]
pub struct EnvironmentalParams {
    pub ph_acidification_rate: f32,
    pub rock_buffer_rate: f32,
    pub water_buffer_rate: f32,
    pub plant_nitrogen_uptake: f32,
}

impl SimulationParameters {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn apply_difficulty(&mut self, difficulty: &DifficultyConfig) {
        let scaling = &difficulty.scaling;

        // Apply difficulty scaling to photosynthesis
        self.photosynthesis.base_rate *= 1.0 - scaling.photosynthesis_penalty;
        self.photosynthesis.co2_efficiency *= 1.0 - scaling.photosynthesis_penalty * 0.5;

        // Apply difficulty scaling to respiration
        self.respiration.base_rate *= 1.0 + scaling.respiration_increase;

        // Apply difficulty scaling to growth rates
        self.microbial.growth_rate *= 1.0 - scaling.growth_penalty;
        self.worm.growth_rate *= 1.0 - scaling.growth_penalty;
        self.shrimp.growth_rate *= 1.0 - scaling.growth_penalty;

        // Apply difficulty scaling to death rates
        self.microbial.death_rate *= 1.0 + scaling.death_rate_increase;
        self.worm.death_rate *= 1.0 + scaling.death_rate_increase;
        self.shrimp.death_rate *= 1.0 + scaling.death_rate_increase;

        // Apply difficulty scaling to environmental buffers
        self.environmental.rock_buffer_rate *= 1.0 - scaling.buffer_reduction;
        self.environmental.water_buffer_rate *= 1.0 - scaling.buffer_reduction;

        // Increase nutrient uptake with difficulty
        self.environmental.plant_nitrogen_uptake *= 1.0 + scaling.growth_penalty;
        self.environmental.ph_acidification_rate *= 1.0 + scaling.death_rate_increase;
    }

    pub fn validate(&self) -> EcosystemResult<()> {
        // Validate that all rates are positive
        if self.photosynthesis.base_rate <= 0.0 {
            return Err(EcosystemError::ConfigurationError {
                message: "Photosynthesis base rate must be positive".to_string(),
            });
        }

        if self.respiration.base_rate <= 0.0 {
            return Err(EcosystemError::ConfigurationError {
                message: "Respiration base rate must be positive".to_string(),
            });
        }

        if self.microbial.growth_rate <= 0.0 {
            return Err(EcosystemError::ConfigurationError {
                message: "Microbial growth rate must be positive".to_string(),
            });
        }

        // Validate ranges
        if self.photosynthesis.co2_efficiency > 5.0 {
            return Err(EcosystemError::ConfigurationError {
                message: "CO2 efficiency too high".to_string(),
            });
        }

        Ok(())
    }

    pub fn balanced() -> Self {
        Self::default()
    }

    pub fn realistic() -> Self {
        Self {
            photosynthesis: PhotosynthesisParams {
                base_rate: 0.08,
                co2_efficiency: 1.2,
                light_dependency: 0.9,
                humidity_dependency: 0.8,
            },
            respiration: RespirationParams {
                base_rate: 0.001,
                co2_production: 0.9,
            },
            microbial: MicrobialParams {
                nitrogen_fixation_rate: 0.006,
                growth_rate: 0.008,
                death_rate: 0.003,
                respiration_rate: 0.0008,
                respiration_co2_ratio: 0.9,
            },
            worm: WormParams {
                aeration_rate: 0.008,
                decomposition_rate: 0.008,
                growth_rate: 0.008,
                death_rate: 0.003,
            },
            shrimp: ShrimpParams {
                detritus_consumption_rate: 0.008,
                waste_production_rate: 0.004,
                growth_rate: 0.008,
                death_rate: 0.003,
            },
            environmental: EnvironmentalParams {
                ph_acidification_rate: 0.0008,
                rock_buffer_rate: 0.0015,
                water_buffer_rate: 0.0008,
                plant_nitrogen_uptake: 0.0015,
            },
        }
    }
}

impl Default for SimulationParameters {
    fn default() -> Self {
        Self {
            photosynthesis: PhotosynthesisParams {
                base_rate: 0.10,
                co2_efficiency: 1.5,
                light_dependency: 1.0,
                humidity_dependency: 1.0,
            },
            respiration: RespirationParams {
                base_rate: 0.002,
                co2_production: 1.0,
            },
            microbial: MicrobialParams {
                nitrogen_fixation_rate: 0.008,
                growth_rate: 0.01,
                death_rate: 0.005,
                respiration_rate: 0.001,
                respiration_co2_ratio: 1.0,
            },
            worm: WormParams {
                aeration_rate: 0.01,
                decomposition_rate: 0.01,
                growth_rate: 0.01,
                death_rate: 0.005,
            },
            shrimp: ShrimpParams {
                detritus_consumption_rate: 0.01,
                waste_production_rate: 0.005,
                growth_rate: 0.01,
                death_rate: 0.005,
            },
            environmental: EnvironmentalParams {
                ph_acidification_rate: 0.001,
                rock_buffer_rate: 0.002,
                water_buffer_rate: 0.001,
                plant_nitrogen_uptake: 0.002,
            },
        }
    }
}