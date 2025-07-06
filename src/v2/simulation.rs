// v2/simulation.rs
// Simulation logic and ODE/difference equations for v2

use crate::v2::config::V2Config;
use crate::v2::state::EcosystemStateV2;

pub fn update_ecosystem_v2(config: &V2Config, state: &mut EcosystemStateV2, is_day: bool) {
    // --- Parameters (to be tuned) ---
    let k_photo = 0.02;
    let k_resp = 0.01;
    let alpha_photo = 1.0;
    let alpha_resp = 1.0;
    let k_grow = 0.01;
    let k_n_plant = 0.005;
    let k_fix = 0.008;
    let k_m_grow = 0.01;
    let k_m_death = 0.005;
    let k_m_resp = 0.008;
    let alpha_m_resp = 1.0;
    let k_worm_air = 0.01;
    let k_worm_decomp = 0.01;
    let k_w_grow = 0.01;
    let k_w_death = 0.005;
    let k_shrimp_det = 0.01;
    let k_shrimp_waste = 0.005;
    let k_s_grow = 0.01;
    let k_s_death = 0.005;
    let k_acid = 0.001;
    let k_buffer_rock = 0.002;
    let k_buffer_water = 0.001;
    // --- Helper functions ---
    let f_light = |l: f32| (l / 6.0).min(1.0);
    let f_hum = |h: f32| (h / 100.0).min(1.0);
    let f_temp = |t: f32| (-((t - 24.0).powi(2)) / 32.0).exp();
    let f_nutr = |n: f32| (n / 2.0).min(1.0);
    let f_comp = |p: f32| (1.0 - (p / 100.0)).max(0.0);
    let f_moist = |w: f32| (w / 2.0).min(1.0);
    let f_p_h = |ph: f32| (-(ph - 7.0).powi(2) / 8.0).exp();
    let f_o2 = |o2: f32| (o2 / 21.0).min(1.0);
    let f_det = |d: f32| (d / 2.0).min(1.0);
    let f_tox = |_tox: f32| 0.0; // placeholder
    // --- Light and humidity factors ---
    let light = (6 - config.window_proximity) as f32;
    let hum = state.humidity;
    let temp = state.temperature;
    // --- Plant equations ---
    if is_day {
        let d_o2 = k_photo * state.plant_biomass * light * f_hum(hum) * f_o2(state.air_co2);
        let d_co2 = -alpha_photo * d_o2;
        let d_n = -k_n_plant * state.plant_biomass;
        let d_p = k_grow * state.plant_biomass * f_light(light) * f_nutr(state.soil_nitrogen) * f_hum(hum) * f_comp(state.plant_biomass);
        state.air_o2 += d_o2;
        state.air_co2 += d_co2;
        state.soil_nitrogen += d_n;
        state.plant_biomass += d_p;
    } else {
        let d_o2 = -k_resp * state.plant_biomass;
        let d_co2 = -alpha_resp * d_o2;
        state.air_o2 += d_o2;
        state.air_co2 += d_co2;
    }
    // --- Microbial equations ---
    let d_n_fix = k_fix * state.microbe_pop * f_o2(state.air_o2) * f_moist(state.soil_moisture);
    let d_m = k_m_grow * state.microbe_pop * f_nutr(state.soil_nitrogen) * f_moist(state.soil_moisture) * f_temp(temp)
        - k_m_death * state.microbe_pop * f_p_h(state.soil_ph) * f_o2(state.air_o2);
    let d_o2_m = -k_m_resp * state.microbe_pop;
    let d_co2_m = -alpha_m_resp * d_o2_m;
    state.soil_nitrogen += d_n_fix;
    state.microbe_pop += d_m;
    state.air_o2 += d_o2_m;
    state.air_co2 += d_co2_m;
    // --- Worms ---
    let d_a_soil = k_worm_air * state.worm_pop;
    let d_d_worm = -k_worm_decomp * state.worm_pop;
    let d_w = k_w_grow * state.worm_pop * f_det(state.detritus) * f_moist(state.soil_moisture)
        - k_w_death * state.worm_pop * f_tox(0.0);
    state.soil_aeration += d_a_soil;
    state.detritus += d_d_worm;
    state.worm_pop += d_w;
    // --- Shrimp ---
    let d_d_shrimp = -k_shrimp_det * state.shrimp_pop;
    let d_n_shrimp = k_shrimp_waste * state.shrimp_pop;
    let d_s = k_s_grow * state.shrimp_pop * f_det(state.detritus) * f_o2(state.water_o2)
        - k_s_death * state.shrimp_pop * f_tox(0.0);
    state.detritus += d_d_shrimp;
    state.soil_nitrogen += d_n_shrimp;
    state.shrimp_pop += d_s;
    // --- Soil, water, air ---
    let dp_h = -k_acid * state.microbe_pop + k_buffer_rock * state.rocks as f32 + k_buffer_water * state.water_liters;
    state.soil_ph += dp_h;
    // Clamp and normalize
    state.temperature = state.temperature.max(5.0).min(45.0);
    state.humidity = state.humidity.max(0.0).min(100.0);
    state.plant_biomass = state.plant_biomass.max(0.0);
    state.microbe_pop = state.microbe_pop.max(0.0);
    state.worm_pop = state.worm_pop.max(0.0);
    state.shrimp_pop = state.shrimp_pop.max(0.0);
    state.soil_nitrogen = state.soil_nitrogen.max(0.0);
    state.soil_ph = state.soil_ph.max(0.0).min(14.0);
    state.detritus = state.detritus.max(0.0);
    state.soil_aeration = state.soil_aeration.max(0.0);
    state.water_liters = state.water_liters.max(0.0);
    state.water_o2 = state.water_o2.max(0.0);
    state.air_o2 = state.air_o2.max(0.0);
    state.air_co2 = state.air_co2.max(0.0);
    state.air_n2 = (100.0 - state.air_o2 - state.air_co2).max(0.0);
}
