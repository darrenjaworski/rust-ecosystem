// v2/state.rs
// State struct for v2 ecosystem simulation

pub struct EcosystemStateV2 {
    pub plant_biomass: f32,
    pub microbe_pop: f32,
    pub worm_pop: f32,
    pub shrimp_pop: f32,
    pub soil_nitrogen: f32,
    pub soil_ph: f32,
    pub soil_moisture: f32,
    pub soil_aeration: f32,
    pub detritus: f32,
    pub water_liters: f32,
    pub water_o2: f32,
    pub air_n2: f32,
    pub air_o2: f32,
    pub air_co2: f32,
    pub temperature: f32,
    pub humidity: f32,
    pub rocks: usize,
}

impl EcosystemStateV2 {
    #[allow(dead_code)]
    pub fn new(config: &crate::v2::config::V2Config) -> Self {
        Self {
            plant_biomass: 1.0,
            microbe_pop: config.num_microbes as f32,
            worm_pop: config.num_worms as f32,
            shrimp_pop: config.num_shrimp as f32,
            soil_nitrogen: 1.0,
            soil_ph: 7.0,
            soil_moisture: config.water_liters,
            soil_aeration: 1.0,
            detritus: 0.5,
            water_liters: config.water_liters,
            water_o2: 8.0,
            air_n2: 78.0,
            air_o2: 21.0,
            air_co2: 0.04,
            temperature: config.initial_temp,
            humidity: config.initial_humidity,
            rocks: config.rocks,
        }
    }
    pub fn new_with_seed(config: &crate::v2::config::V2Config, seed: u64) -> Self {
        use rand::{Rng, SeedableRng};
        use rand::rngs::StdRng;
        let mut rng = StdRng::seed_from_u64(seed);
        Self {
            plant_biomass: 1.0,
            microbe_pop: rng.gen_range(500.0..=2000.0),
            worm_pop: rng.gen_range(1.0..=10.0),
            shrimp_pop: rng.gen_range(1.0..=5.0),
            soil_nitrogen: rng.gen_range(0.5..=2.0),
            soil_ph: rng.gen_range(5.5..=8.5),
            soil_moisture: rng.gen_range(0.2..=config.water_liters),
            soil_aeration: rng.gen_range(0.5..=2.0),
            detritus: rng.gen_range(0.1..=2.0),
            water_liters: config.water_liters,
            water_o2: rng.gen_range(6.0..=10.0),
            air_n2: 78.0,
            air_o2: 21.0,
            air_co2: 0.04,
            temperature: rng.gen_range(18.0..=28.0),
            humidity: rng.gen_range(40.0..=80.0),
            rocks: config.rocks,
        }
    }
}
