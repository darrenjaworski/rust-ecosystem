// v2/config/difficulty.rs
// Difficulty configuration for ecosystem simulation

use crate::v2::errors::{EcosystemError, EcosystemResult};

#[derive(Debug, Clone)]
pub struct DifficultyConfig {
    pub level: f32,
    pub scaling: DifficultyScaling,
}

#[derive(Debug, Clone)]
pub struct DifficultyScaling {
    pub photosynthesis_penalty: f32,
    pub respiration_increase: f32,
    pub growth_penalty: f32,
    pub death_rate_increase: f32,
    pub buffer_reduction: f32,
}

impl DifficultyConfig {
    pub fn new(level: f32) -> EcosystemResult<Self> {
        if !(0.0..=1.0).contains(&level) {
            return Err(EcosystemError::ConfigurationError {
                message: format!("Difficulty level {} must be between 0.0 and 1.0", level),
            });
        }

        let scaling = DifficultyScaling {
            photosynthesis_penalty: 0.7 * level,
            respiration_increase: 2.0 * level,
            growth_penalty: 0.7 * level,
            death_rate_increase: 1.5 * level,
            buffer_reduction: 0.8 * level,
        };

        Ok(Self { level, scaling })
    }

    pub fn validate(&self) -> EcosystemResult<()> {
        if !(0.0..=1.0).contains(&self.level) {
            return Err(EcosystemError::ConfigurationError {
                message: format!("Difficulty level {} must be between 0.0 and 1.0", self.level),
            });
        }
        Ok(())
    }

    pub fn easy() -> Self {
        Self::new(0.2).unwrap()
    }

    pub fn medium() -> Self {
        Self::new(0.5).unwrap()
    }

    pub fn hard() -> Self {
        Self::new(0.8).unwrap()
    }

    pub fn extreme() -> Self {
        Self::new(1.0).unwrap()
    }
}

impl Default for DifficultyConfig {
    fn default() -> Self {
        Self::new(0.5).unwrap() // Medium difficulty
    }
}