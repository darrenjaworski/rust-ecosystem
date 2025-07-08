// v2/organisms/plants.rs
// Plant simulation logic

use crate::v2::config::parameters::SimulationParameters;
use crate::v2::state::EcosystemStateV2;
use crate::v2::environmental::*;
use crate::v2::errors::EcosystemResult;

/// Update plant biomass and associated processes
pub fn update_plants(
    state: &mut EcosystemStateV2,
    params: &SimulationParameters,
    is_day: bool,
    dt: f32,
) -> EcosystemResult<()> {
    if is_day {
        // Photosynthesis during day
        photosynthesis(state, params, dt)?;
        
        // Plant growth
        plant_growth(state, params, dt)?;
        
        // Nitrogen uptake
        nitrogen_uptake(state, params, dt)?;
    } else {
        // Respiration at night
        plant_respiration(state, params, dt)?;
    }
    
    Ok(())
}

/// Calculate photosynthesis rate and update oxygen/CO2
fn photosynthesis(
    state: &mut EcosystemStateV2,
    params: &SimulationParameters,
    dt: f32,
) -> EcosystemResult<()> {
    let light_level = state.light_level();
    let humidity_factor = humidity_efficiency(state.humidity);
    let co2_factor = (state.air_co2.value() / 0.04).min(2.0); // CO2 can enhance photosynthesis
    
    let photosynthesis_rate = params.photosynthesis.base_rate 
        * state.plant_biomass.value()
        * light_efficiency(light_level)
        * humidity_factor
        * co2_factor;
    
    let oxygen_production = photosynthesis_rate * dt;
    let co2_consumption = oxygen_production * params.photosynthesis.co2_efficiency;
    
    state.air_o2 = state.air_o2.add(oxygen_production)?;
    state.air_co2 = state.air_co2.subtract(co2_consumption)?;
    
    Ok(())
}

/// Calculate plant respiration and update oxygen/CO2
fn plant_respiration(
    state: &mut EcosystemStateV2,
    params: &SimulationParameters,
    dt: f32,
) -> EcosystemResult<()> {
    let respiration_rate = params.respiration.base_rate * state.plant_biomass.value();
    
    let oxygen_consumption = respiration_rate * dt;
    let co2_production = oxygen_consumption * params.respiration.co2_production;
    
    state.air_o2 = state.air_o2.subtract(oxygen_consumption)?;
    state.air_co2 = state.air_co2.add(co2_production)?;
    
    Ok(())
}

/// Calculate plant growth based on environmental conditions
fn plant_growth(
    state: &mut EcosystemStateV2,
    params: &SimulationParameters,
    dt: f32,
) -> EcosystemResult<()> {
    let light_level = state.light_level();
    let light_factor = light_efficiency(light_level);
    let nutrient_factor = nutrient_efficiency(state.soil_nitrogen);
    let humidity_factor = humidity_efficiency(state.humidity);
    let competition_factor = competition_factor(state.plant_biomass);
    
    let growth_rate = params.photosynthesis.base_rate * 0.3 // Growth is slower than photosynthesis
        * state.plant_biomass.value()
        * light_factor
        * nutrient_factor
        * humidity_factor
        * competition_factor;
    
    let biomass_increase = growth_rate * dt;
    state.plant_biomass = state.plant_biomass.add(biomass_increase)?;
    
    Ok(())
}

/// Calculate nitrogen uptake by plants
fn nitrogen_uptake(
    state: &mut EcosystemStateV2,
    params: &SimulationParameters,
    dt: f32,
) -> EcosystemResult<()> {
    let uptake_rate = params.environmental.plant_nitrogen_uptake 
        * state.plant_biomass.value();
    
    let nitrogen_consumed = uptake_rate * dt;
    state.soil_nitrogen = state.soil_nitrogen.subtract(nitrogen_consumed)?;
    
    Ok(())
}

/// Check if plants have collapsed
pub fn are_plants_collapsed(state: &EcosystemStateV2) -> bool {
    state.plant_biomass.is_collapsed()
}

/// Get current plant health status
pub fn plant_health_status(state: &EcosystemStateV2) -> PlantHealthStatus {
    let biomass = state.plant_biomass.value();
    let light_level = state.light_level();
    let nutrient_level = state.soil_nitrogen.value();
    
    let light_factor = light_efficiency(light_level);
    let nutrient_factor = nutrient_efficiency(state.soil_nitrogen);
    let humidity_factor = humidity_efficiency(state.humidity);
    
    PlantHealthStatus {
        biomass,
        light_adequacy: light_factor,
        nutrient_adequacy: nutrient_factor,
        humidity_adequacy: humidity_factor,
        is_growing: biomass > 0.01 && light_factor > 0.3 && nutrient_factor > 0.1,
        is_stressed: light_factor < 0.5 || nutrient_factor < 0.2 || humidity_factor < 0.4,
    }
}

#[derive(Debug, Clone)]
pub struct PlantHealthStatus {
    pub biomass: f32,
    pub light_adequacy: f32,
    pub nutrient_adequacy: f32,
    pub humidity_adequacy: f32,
    pub is_growing: bool,
    pub is_stressed: bool,
}

// Extension trait for Biomass to add plant-specific operations
pub trait BiomassOps {
    fn add(self, amount: f32) -> EcosystemResult<Self>
    where
        Self: Sized;
    fn subtract(self, amount: f32) -> EcosystemResult<Self>
    where
        Self: Sized;
}

impl BiomassOps for crate::v2::types::Biomass {
    fn add(self, amount: f32) -> EcosystemResult<Self> {
        let new_value = self.value() + amount;
        Ok(crate::v2::types::Biomass::new(new_value)?)
    }
    
    fn subtract(self, amount: f32) -> EcosystemResult<Self> {
        let new_value = (self.value() - amount).max(0.0);
        Ok(crate::v2::types::Biomass::new(new_value)?)
    }
}

// Extension trait for gas concentrations
pub trait GasOps {
    fn add(self, amount: f32) -> EcosystemResult<Self>
    where
        Self: Sized;
    fn subtract(self, amount: f32) -> EcosystemResult<Self>
    where
        Self: Sized;
}

impl GasOps for crate::v2::types::Oxygen {
    fn add(self, amount: f32) -> EcosystemResult<Self> {
        let new_value = self.percentage() + amount;
        Ok(crate::v2::types::Oxygen::new(new_value)?)
    }
    
    fn subtract(self, amount: f32) -> EcosystemResult<Self> {
        let new_value = (self.percentage() - amount).max(0.0);
        Ok(crate::v2::types::Oxygen::new(new_value)?)
    }
}

impl GasOps for crate::v2::types::CarbonDioxide {
    fn add(self, amount: f32) -> EcosystemResult<Self> {
        let new_value = self.value() + amount;
        Ok(crate::v2::types::CarbonDioxide::new(new_value)?)
    }
    
    fn subtract(self, amount: f32) -> EcosystemResult<Self> {
        let new_value = (self.value() - amount).max(0.0);
        Ok(crate::v2::types::CarbonDioxide::new(new_value)?)
    }
}

impl GasOps for crate::v2::types::Nitrogen {
    fn add(self, amount: f32) -> EcosystemResult<Self> {
        let new_value = self.value() + amount;
        Ok(crate::v2::types::Nitrogen::new(new_value)?)
    }
    
    fn subtract(self, amount: f32) -> EcosystemResult<Self> {
        let new_value = (self.value() - amount).max(0.0);
        Ok(crate::v2::types::Nitrogen::new(new_value)?)
    }
}