// v2/organisms/microbes.rs
// Microbe simulation logic

use crate::v2::config::parameters::SimulationParameters;
use crate::v2::state::EcosystemStateV2;
use crate::v2::environmental::*;
use crate::v2::errors::EcosystemResult;
use crate::v2::organisms::plants::{GasOps, BiomassOps};

/// Update microbe population and associated processes
pub fn update_microbes(
    state: &mut EcosystemStateV2,
    params: &SimulationParameters,
    dt: f32,
) -> EcosystemResult<()> {
    // Nitrogen fixation
    nitrogen_fixation(state, params, dt)?;
    
    // Microbe population growth and death
    microbe_population_dynamics(state, params, dt)?;
    
    // Microbe respiration
    microbe_respiration(state, params, dt)?;
    
    Ok(())
}

/// Calculate nitrogen fixation by microbes
fn nitrogen_fixation(
    state: &mut EcosystemStateV2,
    params: &SimulationParameters,
    dt: f32,
) -> EcosystemResult<()> {
    let oxygen_factor = oxygen_efficiency(state.air_o2);
    let moisture_factor = moisture_efficiency(state.soil_moisture);
    
    let fixation_rate = params.microbial.nitrogen_fixation_rate
        * state.microbe_pop.value()
        * oxygen_factor
        * moisture_factor;
    
    let nitrogen_fixed = fixation_rate * dt;
    state.soil_nitrogen = state.soil_nitrogen.add(nitrogen_fixed)?;
    
    // Consume some atmospheric nitrogen
    let n2_consumed = nitrogen_fixed * 0.1; // Small amount from atmosphere
    let new_air_n2 = (state.air_n2.value() - n2_consumed).max(0.0);
    state.air_n2 = crate::v2::types::Nitrogen::new(new_air_n2)?;
    
    Ok(())
}

/// Calculate microbe population growth and death
fn microbe_population_dynamics(
    state: &mut EcosystemStateV2,
    params: &SimulationParameters,
    dt: f32,
) -> EcosystemResult<()> {
    let nutrient_factor = nutrient_efficiency(state.soil_nitrogen);
    let moisture_factor = moisture_efficiency(state.soil_moisture);
    let temperature_factor = temperature_efficiency(state.temperature);
    
    // Growth
    let growth_rate = params.microbial.growth_rate
        * state.microbe_pop.value()
        * nutrient_factor
        * moisture_factor
        * temperature_factor;
    
    // Death
    let ph_factor = ph_efficiency(state.soil_ph);
    let oxygen_factor = oxygen_efficiency(state.air_o2);
    let death_rate = params.microbial.death_rate
        * state.microbe_pop.value()
        * (1.0 - ph_factor)  // Higher death when pH is not optimal
        * (1.0 - oxygen_factor); // Higher death when oxygen is low
    
    let net_growth = (growth_rate - death_rate) * dt;
    let new_population = (state.microbe_pop.value() + net_growth).max(0.0);
    
    state.microbe_pop = crate::v2::types::Population::new(new_population)?;
    
    Ok(())
}

/// Calculate microbe respiration
fn microbe_respiration(
    state: &mut EcosystemStateV2,
    params: &SimulationParameters,
    dt: f32,
) -> EcosystemResult<()> {
    let respiration_rate = params.microbial.respiration_rate * state.microbe_pop.value();
    
    let oxygen_consumption = respiration_rate * dt;
    let co2_production = oxygen_consumption * params.microbial.respiration_co2_ratio;
    
    state.air_o2 = state.air_o2.subtract(oxygen_consumption)?;
    state.air_co2 = state.air_co2.add(co2_production)?;
    
    Ok(())
}

/// Check if microbes have collapsed
pub fn are_microbes_collapsed(state: &EcosystemStateV2) -> bool {
    state.microbe_pop.is_collapsed()
}

/// Get current microbe health status
pub fn microbe_health_status(state: &EcosystemStateV2) -> MicrobeHealthStatus {
    let population = state.microbe_pop.value();
    let nutrient_factor = nutrient_efficiency(state.soil_nitrogen);
    let moisture_factor = moisture_efficiency(state.soil_moisture);
    let temperature_factor = temperature_efficiency(state.temperature);
    let ph_factor = ph_efficiency(state.soil_ph);
    let oxygen_factor = oxygen_efficiency(state.air_o2);
    
    MicrobeHealthStatus {
        population,
        nutrient_adequacy: nutrient_factor,
        moisture_adequacy: moisture_factor,
        temperature_adequacy: temperature_factor,
        ph_adequacy: ph_factor,
        oxygen_adequacy: oxygen_factor,
        is_growing: population > 0.01 && nutrient_factor > 0.2 && moisture_factor > 0.2,
        is_stressed: ph_factor < 0.5 || oxygen_factor < 0.3 || temperature_factor < 0.3,
        nitrogen_fixation_rate: calculate_nitrogen_fixation_rate(state),
    }
}

fn calculate_nitrogen_fixation_rate(state: &EcosystemStateV2) -> f32 {
    let oxygen_factor = oxygen_efficiency(state.air_o2);
    let moisture_factor = moisture_efficiency(state.soil_moisture);
    
    0.008 * state.microbe_pop.value() * oxygen_factor * moisture_factor
}

#[derive(Debug, Clone)]
pub struct MicrobeHealthStatus {
    pub population: f32,
    pub nutrient_adequacy: f32,
    pub moisture_adequacy: f32,
    pub temperature_adequacy: f32,
    pub ph_adequacy: f32,
    pub oxygen_adequacy: f32,
    pub is_growing: bool,
    pub is_stressed: bool,
    pub nitrogen_fixation_rate: f32,
}

// Extension trait for Population operations
pub trait PopulationOps {
    fn add(self, amount: f32) -> EcosystemResult<Self>
    where
        Self: Sized;
    fn subtract(self, amount: f32) -> EcosystemResult<Self>
    where
        Self: Sized;
    fn multiply(self, factor: f32) -> EcosystemResult<Self>
    where
        Self: Sized;
}

impl PopulationOps for crate::v2::types::Population {
    fn add(self, amount: f32) -> EcosystemResult<Self> {
        let new_value = self.value() + amount;
        if new_value <= 0.0 {
            Ok(crate::v2::types::Population::new(0.01)?) // Minimum viable population
        } else {
            Ok(crate::v2::types::Population::new(new_value)?)
        }
    }
    
    fn subtract(self, amount: f32) -> EcosystemResult<Self> {
        let new_value = (self.value() - amount).max(0.01);
        Ok(crate::v2::types::Population::new(new_value)?)
    }
    
    fn multiply(self, factor: f32) -> EcosystemResult<Self> {
        let new_value = (self.value() * factor).max(0.01);
        Ok(crate::v2::types::Population::new(new_value)?)
    }
}