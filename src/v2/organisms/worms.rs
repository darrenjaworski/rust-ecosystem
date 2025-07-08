// v2/organisms/worms.rs
// Worm simulation logic

use crate::v2::config::parameters::SimulationParameters;
use crate::v2::state::EcosystemStateV2;
use crate::v2::environmental::*;
use crate::v2::errors::EcosystemResult;
use crate::v2::organisms::microbes::PopulationOps;

/// Update worm population and associated processes
pub fn update_worms(
    state: &mut EcosystemStateV2,
    params: &SimulationParameters,
    dt: f32,
) -> EcosystemResult<()> {
    // Soil aeration by worms
    soil_aeration(state, params, dt)?;
    
    // Decomposition of organic matter
    decomposition(state, params, dt)?;
    
    // Worm population dynamics
    worm_population_dynamics(state, params, dt)?;
    
    Ok(())
}

/// Calculate soil aeration by worms
fn soil_aeration(
    state: &mut EcosystemStateV2,
    params: &SimulationParameters,
    dt: f32,
) -> EcosystemResult<()> {
    let aeration_increase = params.worm.aeration_rate * state.worm_pop.value() * dt;
    
    let new_aeration = state.soil_aeration.value() + aeration_increase;
    state.soil_aeration = crate::v2::types::Aeration::new(new_aeration)?;
    
    Ok(())
}

/// Calculate decomposition of organic matter by worms
fn decomposition(
    state: &mut EcosystemStateV2,
    params: &SimulationParameters,
    dt: f32,
) -> EcosystemResult<()> {
    let decomposition_rate = params.worm.decomposition_rate * state.worm_pop.value();
    let detritus_consumed = decomposition_rate * dt;
    
    // Consume detritus
    let new_detritus = (state.detritus.value() - detritus_consumed).max(0.0);
    state.detritus = crate::v2::types::Detritus::new(new_detritus)?;
    
    // Convert some detritus back to soil nutrients
    let nutrients_released = detritus_consumed * 0.3; // 30% conversion efficiency
    let new_nitrogen = state.soil_nitrogen.value() + nutrients_released;
    state.soil_nitrogen = crate::v2::types::Nitrogen::new(new_nitrogen)?;
    
    Ok(())
}

/// Calculate worm population growth and death
fn worm_population_dynamics(
    state: &mut EcosystemStateV2,
    params: &SimulationParameters,
    dt: f32,
) -> EcosystemResult<()> {
    let detritus_factor = detritus_availability(state.detritus);
    let moisture_factor = moisture_efficiency(state.soil_moisture);
    let temperature_factor = temperature_efficiency(state.temperature);
    
    // Growth
    let growth_rate = params.worm.growth_rate
        * state.worm_pop.value()
        * detritus_factor
        * moisture_factor
        * temperature_factor;
    
    // Death (including toxicity effects when implemented)
    let toxicity_factor = toxicity_factor(0.0); // Placeholder
    let death_rate = params.worm.death_rate
        * state.worm_pop.value()
        * (1.0 + toxicity_factor);
    
    let net_growth = (growth_rate - death_rate) * dt;
    let new_population = (state.worm_pop.value() + net_growth).max(0.0);
    
    state.worm_pop = crate::v2::types::Population::new(new_population)?;
    
    Ok(())
}

/// Check if worms have collapsed
pub fn are_worms_collapsed(state: &EcosystemStateV2) -> bool {
    state.worm_pop.is_collapsed()
}

/// Get current worm health status
pub fn worm_health_status(state: &EcosystemStateV2) -> WormHealthStatus {
    let population = state.worm_pop.value();
    let detritus_factor = detritus_availability(state.detritus);
    let moisture_factor = moisture_efficiency(state.soil_moisture);
    let temperature_factor = temperature_efficiency(state.temperature);
    
    WormHealthStatus {
        population,
        detritus_adequacy: detritus_factor,
        moisture_adequacy: moisture_factor,
        temperature_adequacy: temperature_factor,
        is_growing: population > 0.01 && detritus_factor > 0.1 && moisture_factor > 0.2,
        is_stressed: moisture_factor < 0.3 || temperature_factor < 0.3,
        soil_aeration_contribution: calculate_aeration_contribution(state),
        decomposition_rate: calculate_decomposition_rate(state),
    }
}

fn calculate_aeration_contribution(state: &EcosystemStateV2) -> f32 {
    0.01 * state.worm_pop.value()
}

fn calculate_decomposition_rate(state: &EcosystemStateV2) -> f32 {
    0.01 * state.worm_pop.value()
}

#[derive(Debug, Clone)]
pub struct WormHealthStatus {
    pub population: f32,
    pub detritus_adequacy: f32,
    pub moisture_adequacy: f32,
    pub temperature_adequacy: f32,
    pub is_growing: bool,
    pub is_stressed: bool,
    pub soil_aeration_contribution: f32,
    pub decomposition_rate: f32,
}

/// Calculate the benefit worms provide to the ecosystem
pub fn worm_ecosystem_benefits(state: &EcosystemStateV2) -> WormEcosystemBenefits {
    let aeration_benefit = state.soil_aeration.value();
    let decomposition_benefit = calculate_decomposition_rate(state);
    let nutrient_cycling_benefit = decomposition_benefit * 0.3; // Nutrient release rate
    
    WormEcosystemBenefits {
        soil_aeration: aeration_benefit,
        organic_matter_recycling: decomposition_benefit,
        nutrient_cycling: nutrient_cycling_benefit,
        overall_soil_health: (aeration_benefit + nutrient_cycling_benefit).min(2.0),
    }
}

#[derive(Debug, Clone)]
pub struct WormEcosystemBenefits {
    pub soil_aeration: f32,
    pub organic_matter_recycling: f32,
    pub nutrient_cycling: f32,
    pub overall_soil_health: f32,
}