use crate::config::{GameConfig, SoilType};
use crate::state::EcosystemState;

pub fn update_ecosystem(config: &GameConfig, state: &mut EcosystemState, is_day: bool) {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    // 1% chance to get a "miracle" run with a small boost to O2/CO2 each interval
    let miracle = rng.gen_bool(0.01);

    // 1. Temperature influenced by window proximity
    if is_day {
        let heat_from_sun = (6 - config.window_proximity) as f32 * 0.2;
        state.temperature += heat_from_sun;
    } else {
        state.temperature -= 0.5;
    }

    // 2. Humidity influenced by temperature and water
    state.humidity += (state.temperature - 20.0) * 0.1 + (config.water_liters as f32 * 0.05);
    state.humidity = state.humidity.max(0.0).min(100.0);

    // 3. Plant growth, photosynthesis (day) and respiration (night)
    if is_day {
        let light_factor = (6 - config.window_proximity) as f32 * 0.006;
        let humidity_factor = state.humidity / 800.0;
        let growth_rate = (light_factor + humidity_factor) * state.co2 * 0.18; // much lower growth
        let competition_factor = 1.0 - (state.plant_size / 40.0); // much faster decay
        state.plant_size += growth_rate * competition_factor * config.num_plants as f32;
        state.plant_size = state.plant_size.max(0.0);

        // Photosynthesis: consume CO2, produce O2
        let photosynthesis_rate = state.plant_size * 0.002; // much lower rate
        let co2_consumed = (photosynthesis_rate * state.co2).min(state.co2);
        state.co2 -= co2_consumed;
        state.oxygen += co2_consumed * 0.7; // much less O2 produced
    } else {
        // Respiration at night: consume O2, produce CO2
        let respiration_rate = state.plant_size * 0.004; // much higher rate
        let o2_consumed = (respiration_rate * state.oxygen).min(state.oxygen);
        state.oxygen -= o2_consumed;
        state.co2 += o2_consumed * 1.2; // more CO2 produced
    }

    // 4. Microbial activity
    let temp_factor = (state.temperature - 15.0).max(0.0) / 10.0;
    let soil_factor = if let SoilType::Porous = config.soil_type { 2.0 } else { 1.0 };
    let microbe_growth = temp_factor * soil_factor * (config.water_liters as f32 * 0.0015); // much lower growth
    let microbe_capacity = config.soil_kg as f32 * 2.0;
    if state.microbial_levels < microbe_capacity {
        state.microbial_levels += microbe_growth;
    }

    // Microbes consume O2 and produce CO2
    let microbe_respiration = state.microbial_levels * 0.0025; // much higher rate
    let o2_consumed_by_microbes = (microbe_respiration * state.oxygen).min(state.oxygen);
    state.oxygen -= o2_consumed_by_microbes;
    state.co2 += o2_consumed_by_microbes * 1.2;

    // 5. pH changes
    let ph_change_from_microbes = state.microbial_levels * 0.00012; // much higher acidification
    state.ph -= ph_change_from_microbes;
    let buffering_effect = (7.0 - state.ph) * (config.water_liters as f32 * 0.004);
    state.ph += buffering_effect;

    // "Miracle" runs: rare indefinite survival, but much smaller effect
    if miracle {
        state.oxygen += 0.05;
        state.co2 += 0.01;
    }

    // Clamp values to reasonable ranges
    state.temperature = state.temperature.max(5.0).min(45.0);
    state.plant_size = state.plant_size.max(0.0).min(100.0);
    state.microbial_levels = state.microbial_levels.max(0.0);
    state.ph = state.ph.max(0.0).min(14.0);
    state.oxygen = state.oxygen.max(0.0);
    state.co2 = state.co2.max(0.0);

    // Normalize air composition to roughly 100%
    let total_air = state.nitrogen + state.oxygen + state.co2;
    state.nitrogen = (state.nitrogen / total_air) * 100.0;
    state.oxygen = (state.oxygen / total_air) * 100.0;
    state.co2 = (state.co2 / total_air) * 100.0;
}