use rand::Rng;
use crate::config::{GameConfig, SoilType};
use crate::state::EcosystemState;
use crate::simulation::update_ecosystem;
use std::collections::BTreeMap;

pub fn run_montecarlo_simulations(num_runs: usize, day_cap: usize) {
    let mut survived = 0;
    let mut total_days = 0;
    let mut histogram: BTreeMap<usize, usize> = BTreeMap::new();
    for _ in 0..num_runs {
        let mut rng = rand::thread_rng();
        let soil_type = if rng.gen_bool(0.5) { SoilType::Porous } else { SoilType::NonPorous };
        let num_plants = rng.gen_range(2..=5);
        let soil_kg = rng.gen_range(10..=30);
        let window_proximity = rng.gen_range(1..=5);
        let water_liters = rng.gen_range(1..=10);
        let config = GameConfig {
            soil_type,
            num_plants,
            soil_kg,
            window_proximity,
            water_liters,
        };
        let mut state = EcosystemState::new();
        let mut day_number = 1;
        loop {
            for _ in 0..10 { update_ecosystem(&config, &mut state, true); }
            for _ in 0..6 { update_ecosystem(&config, &mut state, false); }
            if state.plant_size <= 0.0 || state.oxygen < 5.0 {
                break;
            }
            if day_number >= day_cap {
                survived += 1;
                break;
            }
            day_number += 1;
        }
        total_days += day_number;
        *histogram.entry(day_number).or_insert(0) += 1;
    }
    println!("Monte Carlo Results ({} runs, {} day cap):", num_runs, day_cap);
    println!("  Survived {} days: {} times ({:.1}%)", day_cap, survived, (survived as f64 / num_runs as f64) * 100.0);
    println!("  Average days survived: {:.2}", total_days as f64 / num_runs as f64);
    println!("\nHistogram of days survived before collapse:");
    for (days, count) in &histogram {
        let bar = "#".repeat((*count * 50 / num_runs).max(1));
        println!("  {:>3} days: {:>4} | {}", days, count, bar);
    }
}
