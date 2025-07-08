// v2/simulation_refactored.rs
// Refactored simulation engine using modular components

use crate::v2::config::V2Config;
use crate::v2::state::EcosystemStateV2;
use crate::v2::organisms;
use crate::v2::environmental::*;
use crate::v2::errors::EcosystemResult;

/// Main simulation update function - now much cleaner and more modular
pub fn update_ecosystem_v2(
    config: &V2Config,
    state: &mut EcosystemStateV2,
    is_day: bool,
) -> EcosystemResult<()> {
    let dt = 1.0; // Time step
    
    // Update all organisms using the modular system
    organisms::update_all_organisms(state, &config.parameters, is_day, dt)?;
    
    // Update environmental parameters
    update_environmental_parameters(state, config, dt)?;
    
    // Apply environmental penalties
    organisms::apply_environmental_penalties(state)?;
    
    // Clamp all values to valid ranges
    state.clamp_values()?;
    
    Ok(())
}

/// Update environmental parameters like pH buffering
fn update_environmental_parameters(
    state: &mut EcosystemStateV2,
    config: &V2Config,
    dt: f32,
) -> EcosystemResult<()> {
    // pH changes
    update_ph(state, config, dt)?;
    
    // Water oxygen exchange with air
    update_water_oxygen_exchange(state, dt)?;
    
    Ok(())
}

/// Update soil pH based on various factors
fn update_ph(
    state: &mut EcosystemStateV2,
    config: &V2Config,
    dt: f32,
) -> EcosystemResult<()> {
    let acidification = config.parameters.environmental.ph_acidification_rate 
        * state.microbe_pop.value();
    
    let rock_buffering = config.parameters.environmental.rock_buffer_rate 
        * state.rocks as f32;
    
    let water_buffering = config.parameters.environmental.water_buffer_rate 
        * state.water_liters.value();
    
    let ph_change = (-acidification + rock_buffering + water_buffering) * dt;
    let new_ph = (state.soil_ph.value() + ph_change).clamp(0.0, 14.0);
    
    state.soil_ph = crate::v2::types::Ph::new(new_ph)?;
    
    Ok(())
}

/// Update water oxygen through surface exchange with air
fn update_water_oxygen_exchange(
    state: &mut EcosystemStateV2,
    dt: f32,
) -> EcosystemResult<()> {
    // Oxygen exchange between air and water
    let exchange_rate = 0.01; // Surface exchange rate
    let oxygen_gradient = state.air_o2.percentage() - state.water_o2.percentage();
    let oxygen_transfer = exchange_rate * oxygen_gradient * dt;
    
    let new_water_oxygen = (state.water_o2.percentage() + oxygen_transfer).max(0.0);
    state.water_o2 = crate::v2::types::Oxygen::new(new_water_oxygen)?;
    
    Ok(())
}

/// Returns true if any critical population is at or below collapse threshold
pub fn is_ecosystem_collapsed(state: &EcosystemStateV2) -> bool {
    use crate::v2::traits::CollapseDetection;
    state.is_collapsed()
}

/// Get detailed collapse analysis
pub fn analyze_ecosystem_collapse(state: &EcosystemStateV2) -> EcosystemCollapseAnalysis {
    use crate::v2::traits::CollapseDetection;
    
    EcosystemCollapseAnalysis {
        is_collapsed: state.is_collapsed(),
        collapse_risk: state.collapse_risk(),
        collapse_reasons: state.collapse_reasons(),
        time_to_collapse_estimate: estimate_time_to_collapse(state),
    }
}

/// Estimate time until ecosystem collapse (in days)
fn estimate_time_to_collapse(state: &EcosystemStateV2) -> Option<f32> {
    use crate::v2::traits::CollapseDetection;
    
    let risk = state.collapse_risk();
    if risk < 0.1 {
        None // Very low risk, no estimate
    } else {
        // Simple estimation: higher risk = faster collapse
        let days_estimate = (1.0 - risk) * 30.0; // 0-30 days based on risk
        Some(days_estimate.max(1.0))
    }
}

#[derive(Debug, Clone)]
pub struct EcosystemCollapseAnalysis {
    pub is_collapsed: bool,
    pub collapse_risk: f32,
    pub collapse_reasons: Vec<crate::v2::errors::CollapseReason>,
    pub time_to_collapse_estimate: Option<f32>,
}

/// Get ecosystem health summary
pub fn get_ecosystem_health(state: &EcosystemStateV2) -> EcosystemHealthSummary {
    use crate::v2::traits::{EcosystemValidation, EcosystemMonitoring};
    
    EcosystemHealthSummary {
        is_healthy: state.is_healthy(),
        health_warnings: state.health_warnings(),
        key_metrics: state.key_metrics(),
        alert_conditions: state.alert_conditions(),
    }
}

#[derive(Debug, Clone)]
pub struct EcosystemHealthSummary {
    pub is_healthy: bool,
    pub health_warnings: Vec<String>,
    pub key_metrics: Vec<(String, f32)>,
    pub alert_conditions: Vec<crate::v2::traits::AlertCondition>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::v2::config::V2Config;
    
    #[test]
    fn test_ecosystem_update() {
        let config = V2Config::new();
        let mut state = EcosystemStateV2::new(&config).expect("Failed to create state");
        
        // Test that update doesn't panic
        let result = update_ecosystem_v2(&config, &mut state, true);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_collapse_detection() {
        let config = V2Config::new();
        let mut state = EcosystemStateV2::new(&config).expect("Failed to create state");
        
        // Healthy ecosystem should not be collapsed
        assert!(!is_ecosystem_collapsed(&state));
        
        // Force collapse by setting populations to zero
        state.plant_biomass = crate::v2::types::Biomass::new(0.0).unwrap();
        state.microbe_pop = crate::v2::types::Population::new(0.01).unwrap(); // Minimum viable
        
        assert!(is_ecosystem_collapsed(&state));
    }
}