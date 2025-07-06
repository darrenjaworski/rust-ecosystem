// v2/config.rs
// Configuration and parameters for v2 ecosystem simulation

pub struct V2Config {
    pub num_plants: usize,
    pub num_microbes: usize,
    pub num_worms: usize,
    pub num_shrimp: usize,
    pub soil_kg: f32,
    pub soil_type: SoilType,
    pub water_liters: f32,
    pub rocks: usize,
    pub window_proximity: u8,
    pub initial_temp: f32,
    pub initial_humidity: f32,
}

pub enum SoilType {
    Porous,
    NonPorous,
}

impl Default for V2Config {
    fn default() -> Self {
        Self {
            num_plants: 2,
            num_microbes: 1000,
            num_worms: 5,
            num_shrimp: 2,
            soil_kg: 1.0,
            soil_type: SoilType::Porous,
            water_liters: 0.5,
            rocks: 2,
            window_proximity: 3,
            initial_temp: 22.0,
            initial_humidity: 60.0,
        }
    }
}
