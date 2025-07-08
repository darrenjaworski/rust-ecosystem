// v2/config/organisms.rs
// Organism configuration for ecosystem simulation

use crate::v2::errors::{EcosystemError, EcosystemResult};

#[derive(Debug, Clone)]
pub struct OrganismConfig {
    pub microbes: MicrobeConfig,
    pub worms: WormConfig,
    pub shrimp: ShrimpConfig,
    pub plants: PlantConfig,
}

#[derive(Debug, Clone)]
pub struct MicrobeConfig {
    pub initial_count: usize,
}

#[derive(Debug, Clone)]
pub struct WormConfig {
    pub initial_count: usize,
}

#[derive(Debug, Clone)]
pub struct ShrimpConfig {
    pub initial_count: usize,
}

#[derive(Debug, Clone)]
pub struct PlantConfig {
    pub initial_biomass: f32,
}

impl OrganismConfig {
    pub fn new(
        microbe_count: usize,
        worm_count: usize,
        shrimp_count: usize,
        plant_biomass: f32,
    ) -> EcosystemResult<Self> {
        let config = Self {
            microbes: MicrobeConfig { initial_count: microbe_count },
            worms: WormConfig { initial_count: worm_count },
            shrimp: ShrimpConfig { initial_count: shrimp_count },
            plants: PlantConfig { initial_biomass: plant_biomass },
        };
        
        config.validate()?;
        Ok(config)
    }

    pub fn validate(&self) -> EcosystemResult<()> {
        // Validate microbes
        if self.microbes.initial_count == 0 {
            return Err(EcosystemError::ConfigurationError {
                message: "Microbe count cannot be zero".to_string(),
            });
        }
        if self.microbes.initial_count > 100_000 {
            return Err(EcosystemError::ConfigurationError {
                message: format!("Too many microbes: {} (max 100,000)", self.microbes.initial_count),
            });
        }

        // Validate worms
        if self.worms.initial_count > 50 {
            return Err(EcosystemError::ConfigurationError {
                message: format!("Too many worms: {} (max 50)", self.worms.initial_count),
            });
        }

        // Validate shrimp
        if self.shrimp.initial_count > 20 {
            return Err(EcosystemError::ConfigurationError {
                message: format!("Too many shrimp: {} (max 20)", self.shrimp.initial_count),
            });
        }

        // Validate plants
        if self.plants.initial_biomass <= 0.0 {
            return Err(EcosystemError::ConfigurationError {
                message: "Plant biomass must be positive".to_string(),
            });
        }
        if self.plants.initial_biomass > 100.0 {
            return Err(EcosystemError::ConfigurationError {
                message: format!("Too much initial plant biomass: {} (max 100)", self.plants.initial_biomass),
            });
        }

        Ok(())
    }

    pub fn total_organisms(&self) -> usize {
        self.microbes.initial_count + self.worms.initial_count + self.shrimp.initial_count
    }

    pub fn is_minimal(&self) -> bool {
        self.microbes.initial_count <= 100 && 
        self.worms.initial_count <= 1 && 
        self.shrimp.initial_count <= 1
    }

    pub fn is_complex(&self) -> bool {
        self.microbes.initial_count >= 5000 && 
        self.worms.initial_count >= 10 && 
        self.shrimp.initial_count >= 5
    }
}

impl Default for OrganismConfig {
    fn default() -> Self {
        Self {
            microbes: MicrobeConfig { initial_count: 1000 },
            worms: WormConfig { initial_count: 5 },
            shrimp: ShrimpConfig { initial_count: 2 },
            plants: PlantConfig { initial_biomass: 1.0 },
        }
    }
}

// Preset configurations
impl OrganismConfig {
    pub fn minimal() -> Self {
        Self {
            microbes: MicrobeConfig { initial_count: 100 },
            worms: WormConfig { initial_count: 1 },
            shrimp: ShrimpConfig { initial_count: 1 },
            plants: PlantConfig { initial_biomass: 0.5 },
        }
    }

    pub fn balanced() -> Self {
        Self::default()
    }

    pub fn complex() -> Self {
        Self {
            microbes: MicrobeConfig { initial_count: 5000 },
            worms: WormConfig { initial_count: 15 },
            shrimp: ShrimpConfig { initial_count: 8 },
            plants: PlantConfig { initial_biomass: 2.0 },
        }
    }

    pub fn research() -> Self {
        Self {
            microbes: MicrobeConfig { initial_count: 10000 },
            worms: WormConfig { initial_count: 25 },
            shrimp: ShrimpConfig { initial_count: 12 },
            plants: PlantConfig { initial_biomass: 3.0 },
        }
    }
}