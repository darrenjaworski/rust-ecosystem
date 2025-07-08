// v2/organisms/shrimp.rs
// Shrimp simulation logic

use crate::v2::config::parameters::SimulationParameters;
use crate::v2::state::EcosystemStateV2;
use crate::v2::environmental::*;
use crate::v2::errors::EcosystemResult;
use crate::v2::organisms::microbes::PopulationOps;

/// Update shrimp population and associated processes
pub fn update_shrimp(
    state: &mut EcosystemStateV2,
    params: &SimulationParameters,
    dt: f32,
) -> EcosystemResult<()> {
    // Detritus consumption by shrimp
    detritus_consumption(state, params, dt)?;
    
    // Waste production by shrimp
    waste_production(state, params, dt)?;
    
    // Shrimp population dynamics
    shrimp_population_dynamics(state, params, dt)?;
    
    Ok(())
}

/// Calculate detritus consumption by shrimp
fn detritus_consumption(
    state: &mut EcosystemStateV2,
    params: &SimulationParameters,
    dt: f32,
) -> EcosystemResult<()> {
    let consumption_rate = params.shrimp.detritus_consumption_rate * state.shrimp_pop.value();
    let detritus_consumed = consumption_rate * dt;
    
    // Consume detritus
    let new_detritus = (state.detritus.value() - detritus_consumed).max(0.0);
    state.detritus = crate::v2::types::Detritus::new(new_detritus)?;
    
    Ok(())
}

/// Calculate waste production by shrimp
fn waste_production(
    state: &mut EcosystemStateV2,
    params: &SimulationParameters,
    dt: f32,
) -> EcosystemResult<()> {
    let waste_rate = params.shrimp.waste_production_rate * state.shrimp_pop.value();
    let waste_produced = waste_rate * dt;
    
    // Add waste as soil nitrogen (shrimp waste is nutrient-rich)
    let new_nitrogen = state.soil_nitrogen.value() + waste_produced;
    state.soil_nitrogen = crate::v2::types::Nitrogen::new(new_nitrogen)?;
    
    Ok(())
}

/// Calculate shrimp population growth and death
fn shrimp_population_dynamics(
    state: &mut EcosystemStateV2,
    params: &SimulationParameters,
    dt: f32,
) -> EcosystemResult<()> {
    let detritus_factor = detritus_availability(state.detritus);
    let water_oxygen_factor = water_oxygen_efficiency(state.water_o2);
    let temperature_factor = temperature_efficiency(state.temperature);
    
    // Growth
    let growth_rate = params.shrimp.growth_rate
        * state.shrimp_pop.value()
        * detritus_factor
        * water_oxygen_factor
        * temperature_factor;
    
    // Death (including toxicity effects when implemented)
    let toxicity_factor = toxicity_factor(0.0); // Placeholder
    let death_rate = params.shrimp.death_rate
        * state.shrimp_pop.value()
        * (1.0 + toxicity_factor);
    
    let net_growth = (growth_rate - death_rate) * dt;
    let new_population = (state.shrimp_pop.value() + net_growth).max(0.0);
    
    state.shrimp_pop = crate::v2::types::Population::new(new_population)?;
    
    Ok(())
}

/// Check if shrimp have collapsed
pub fn are_shrimp_collapsed(state: &EcosystemStateV2) -> bool {
    state.shrimp_pop.is_collapsed()
}

/// Get current shrimp health status
pub fn shrimp_health_status(state: &EcosystemStateV2) -> ShrimpHealthStatus {
    let population = state.shrimp_pop.value();
    let detritus_factor = detritus_availability(state.detritus);
    let water_oxygen_factor = water_oxygen_efficiency(state.water_o2);
    let temperature_factor = temperature_efficiency(state.temperature);
    
    ShrimpHealthStatus {
        population,
        detritus_adequacy: detritus_factor,
        water_oxygen_adequacy: water_oxygen_factor,
        temperature_adequacy: temperature_factor,
        is_growing: population > 0.01 && detritus_factor > 0.1 && water_oxygen_factor > 0.3,
        is_stressed: water_oxygen_factor < 0.3 || temperature_factor < 0.3,
        detritus_consumption_rate: calculate_detritus_consumption_rate(state),
        waste_production_rate: calculate_waste_production_rate(state),
    }
}

fn calculate_detritus_consumption_rate(state: &EcosystemStateV2) -> f32 {
    0.01 * state.shrimp_pop.value()
}

fn calculate_waste_production_rate(state: &EcosystemStateV2) -> f32 {
    0.005 * state.shrimp_pop.value()
}

#[derive(Debug, Clone)]
pub struct ShrimpHealthStatus {
    pub population: f32,
    pub detritus_adequacy: f32,
    pub water_oxygen_adequacy: f32,
    pub temperature_adequacy: f32,
    pub is_growing: bool,
    pub is_stressed: bool,
    pub detritus_consumption_rate: f32,
    pub waste_production_rate: f32,
}

/// Calculate the benefit shrimp provide to the ecosystem
pub fn shrimp_ecosystem_benefits(state: &EcosystemStateV2) -> ShrimpEcosystemBenefits {
    let detritus_cleanup = calculate_detritus_consumption_rate(state);
    let nutrient_contribution = calculate_waste_production_rate(state);
    let water_quality_improvement = detritus_cleanup * 0.5; // Removing detritus improves water quality
    
    ShrimpEcosystemBenefits {
        detritus_cleanup,
        nutrient_contribution,
        water_quality_improvement,
        overall_aquatic_health: (detritus_cleanup + nutrient_contribution).min(2.0),
    }
}

#[derive(Debug, Clone)]
pub struct ShrimpEcosystemBenefits {
    pub detritus_cleanup: f32,
    pub nutrient_contribution: f32,
    pub water_quality_improvement: f32,
    pub overall_aquatic_health: f32,
}

/// Calculate water oxygen consumption by shrimp
pub fn shrimp_oxygen_consumption(state: &EcosystemStateV2) -> f32 {
    let base_consumption = 0.001; // Base oxygen consumption rate
    base_consumption * state.shrimp_pop.value()
}

/// Update water oxygen levels based on shrimp activity
pub fn update_water_oxygen(
    state: &mut EcosystemStateV2,
    dt: f32,
) -> EcosystemResult<()> {
    let oxygen_consumed = shrimp_oxygen_consumption(state) * dt;
    let new_water_oxygen = (state.water_o2.percentage() - oxygen_consumed).max(0.0);
    state.water_o2 = crate::v2::types::Oxygen::new(new_water_oxygen)?;
    
    // Water can also gain oxygen from air through surface exchange
    let surface_exchange_rate = 0.01; // Slow oxygen exchange with air
    let oxygen_gained = surface_exchange_rate * (state.air_o2.percentage() - state.water_o2.percentage()).max(0.0) * dt;
    let final_water_oxygen = state.water_o2.percentage() + oxygen_gained;
    state.water_o2 = crate::v2::types::Oxygen::new(final_water_oxygen)?;
    
    Ok(())
}