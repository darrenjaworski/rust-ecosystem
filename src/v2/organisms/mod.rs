// v2/organisms/mod.rs
// Organism-specific simulation logic

pub mod plants;
pub mod microbes;
pub mod worms;
pub mod shrimp;

use crate::v2::config::parameters::SimulationParameters;
use crate::v2::state::EcosystemStateV2;
use crate::v2::environmental::*;
use crate::v2::errors::EcosystemResult;

/// Trait for organisms that can be simulated
pub trait Organism {
    fn update(&mut self, state: &EcosystemStateV2, params: &SimulationParameters, is_day: bool, dt: f32) -> EcosystemResult<()>;
    fn is_collapsed(&self) -> bool;
    fn population_size(&self) -> f32;
}

/// Update all organisms in the ecosystem
pub fn update_all_organisms(
    state: &mut EcosystemStateV2,
    params: &SimulationParameters,
    is_day: bool,
    dt: f32,
) -> EcosystemResult<()> {
    // Update plants
    plants::update_plants(state, params, is_day, dt)?;
    
    // Update microbes
    microbes::update_microbes(state, params, dt)?;
    
    // Update worms
    worms::update_worms(state, params, dt)?;
    
    // Update shrimp
    shrimp::update_shrimp(state, params, dt)?;
    
    Ok(())
}

/// Calculate environmental penalties and apply them to populations
pub fn apply_environmental_penalties(state: &mut EcosystemStateV2) -> EcosystemResult<()> {
    use crate::v2::types::*;
    
    // pH penalties
    let ph_penalty = ph_penalty_factor(state.soil_ph);
    if ph_penalty > 0.0 {
        let new_plant_biomass = (state.plant_biomass.value() * (1.0 - 0.10 * ph_penalty)).max(0.0);
        state.plant_biomass = Biomass::new(new_plant_biomass)?;
        
        let new_microbe_pop = (state.microbe_pop.value() * (1.0 - 0.15 * ph_penalty)).max(0.01);
        state.microbe_pop = Population::new(new_microbe_pop)?;
        
        let new_shrimp_pop = (state.shrimp_pop.value() * (1.0 - 0.20 * ph_penalty)).max(0.01);
        state.shrimp_pop = Population::new(new_shrimp_pop)?;
    }

    // Air oxygen penalties
    let oxygen_penalty = oxygen_penalty_factor(state.air_o2);
    if oxygen_penalty > 0.0 {
        let new_plant_biomass = (state.plant_biomass.value() * (1.0 - 0.10 * oxygen_penalty)).max(0.0);
        state.plant_biomass = Biomass::new(new_plant_biomass)?;
        
        let new_microbe_pop = (state.microbe_pop.value() * (1.0 - 0.15 * oxygen_penalty)).max(0.01);
        state.microbe_pop = Population::new(new_microbe_pop)?;
        
        let new_worm_pop = (state.worm_pop.value() * (1.0 - 0.20 * oxygen_penalty)).max(0.01);
        state.worm_pop = Population::new(new_worm_pop)?;
    }

    // Water oxygen penalties for shrimp
    let water_oxygen_penalty = water_oxygen_penalty_factor(state.water_o2);
    if water_oxygen_penalty > 0.0 {
        let new_shrimp_pop = (state.shrimp_pop.value() * (1.0 - 0.20 * water_oxygen_penalty)).max(0.01);
        state.shrimp_pop = Population::new(new_shrimp_pop)?;
    }

    Ok(())
}