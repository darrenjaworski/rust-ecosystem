// v2/environmental.rs
// Environmental functions for ecosystem modeling

use crate::v2::types::*;

/// Temperature efficiency function - bell curve with optimum at 24°C
pub fn temperature_efficiency(temp: Temperature) -> f32 {
    (-((temp.celsius() - 24.0).powi(2)) / 32.0).exp()
}

/// Humidity efficiency function - increases with humidity, plateaus at 100%
pub fn humidity_efficiency(humidity: Humidity) -> f32 {
    (humidity.percentage() / 100.0).min(1.0)
}

/// Light efficiency function - linear increase with light, saturates at 6
pub fn light_efficiency(light_level: f32) -> f32 {
    (light_level / 6.0).min(1.0)
}

/// Nutrient efficiency function - saturating function for nitrogen
pub fn nutrient_efficiency(nitrogen: Nitrogen) -> f32 {
    (nitrogen.value() / 2.0).min(1.0)
}

/// Competition factor for plant growth - decreases as biomass increases
pub fn competition_factor(biomass: Biomass) -> f32 {
    (1.0 - (biomass.value() / 100.0)).max(0.0)
}

/// Moisture efficiency function - optimal range for water
pub fn moisture_efficiency(moisture: Moisture) -> f32 {
    (moisture.value() / 2.0).min(1.0)
}

/// pH efficiency function - bell curve with optimum at 7.0
pub fn ph_efficiency(ph: Ph) -> f32 {
    (-(ph.value() - 7.0).powi(2) / 8.0).exp()
}

/// Oxygen efficiency function - linear increase with oxygen, saturates at 21%
pub fn oxygen_efficiency(oxygen: Oxygen) -> f32 {
    (oxygen.percentage() / 21.0).min(1.0)
}

/// Detritus availability function - more detritus means more food
pub fn detritus_availability(detritus: Detritus) -> f32 {
    (detritus.value() / 2.0).min(1.0)
}

/// Toxicity factor - placeholder for future toxicity modeling
pub fn toxicity_factor(_toxicity: f32) -> f32 {
    0.0 // Placeholder - no toxicity effects implemented yet
}

/// Water oxygen efficiency - similar to air oxygen but for aquatic organisms
pub fn water_oxygen_efficiency(water_oxygen: Oxygen) -> f32 {
    (water_oxygen.percentage() / 21.0).min(1.0)
}

/// Combined environmental stress factor
pub fn environmental_stress(temp: Temperature, ph: Ph, oxygen: Oxygen) -> f32 {
    let temp_stress = if temp.is_optimal() { 0.0 } else { 
        ((temp.celsius() - 24.0).abs() / 20.0).min(1.0) 
    };
    
    let ph_stress = if (6.5..=7.5).contains(&ph.value()) { 0.0 } else { 
        ((ph.value() - 7.0).abs() / 7.0).min(1.0) 
    };
    
    let oxygen_stress = if oxygen.is_dangerously_low() { 
        (5.0 - oxygen.percentage()) / 5.0 
    } else { 
        0.0 
    };
    
    (temp_stress + ph_stress + oxygen_stress).min(1.0)
}

/// Calculate penalty factor for low pH conditions
pub fn ph_penalty_factor(ph: Ph) -> f32 {
    if ph.value() < 6.5 {
        (6.5 - ph.value()) / 6.5
    } else {
        0.0
    }
}

/// Calculate penalty factor for low oxygen conditions
pub fn oxygen_penalty_factor(oxygen: Oxygen) -> f32 {
    if oxygen.is_dangerously_low() {
        (5.0 - oxygen.percentage()) / 5.0
    } else {
        0.0
    }
}

/// Calculate water oxygen penalty factor
pub fn water_oxygen_penalty_factor(water_oxygen: Oxygen) -> f32 {
    if water_oxygen.percentage() < 5.0 {
        (5.0 - water_oxygen.percentage()) / 5.0
    } else {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_temperature_efficiency() {
        let optimal_temp = Temperature::new(24.0).unwrap();
        let cold_temp = Temperature::new(10.0).unwrap();
        let hot_temp = Temperature::new(40.0).unwrap();
        
        assert!(temperature_efficiency(optimal_temp) > temperature_efficiency(cold_temp));
        assert!(temperature_efficiency(optimal_temp) > temperature_efficiency(hot_temp));
    }

    #[test]
    fn test_ph_efficiency() {
        let neutral_ph = Ph::new(7.0).unwrap();
        let acidic_ph = Ph::new(3.0).unwrap();
        let basic_ph = Ph::new(11.0).unwrap();
        
        assert!(ph_efficiency(neutral_ph) > ph_efficiency(acidic_ph));
        assert!(ph_efficiency(neutral_ph) > ph_efficiency(basic_ph));
    }

    #[test]
    fn test_oxygen_penalty() {
        let normal_oxygen = Oxygen::new(21.0).unwrap();
        let low_oxygen = Oxygen::new(3.0).unwrap();
        
        assert_eq!(oxygen_penalty_factor(normal_oxygen), 0.0);
        assert!(oxygen_penalty_factor(low_oxygen) > 0.0);
    }
}