use rand::Rng;
use crate::v1::config::{GameConfig, SoilType};
use crate::v1::state::EcosystemState;
use crate::v1::simulation::update_ecosystem;
use std::collections::BTreeMap;

#[allow(dead_code)]
pub enum MonteCarloModel {
    V1,
    V2,
}

pub fn run_montecarlo_simulations(num_runs: usize, day_cap: usize, model: MonteCarloModel) {
    match model {
        MonteCarloModel::V1 => run_v1_montecarlo(num_runs, day_cap),
        MonteCarloModel::V2 => run_v2_montecarlo(num_runs, day_cap),
    }
}

fn run_v1_montecarlo(num_runs: usize, day_cap: usize) {
    let num_runs = num_runs.min(100_000); // limit to 100,000
    let day_cap = day_cap.min(1000); // limit to 1,000
    let mut survived = 0;
    let mut total_days = 0;
    let mut histogram: BTreeMap<usize, usize> = BTreeMap::new();
    // Track configs of successful runs
    use std::collections::HashMap;
    let mut survivors: Vec<GameConfig> = Vec::new();
    for i in 0..num_runs {
        // Show progress bar
        if num_runs >= 20 && i % (num_runs / 100).max(1) == 0 {
            let percent = (i * 100) / num_runs;
            print!("\rProgress: [{:3}%] {}/{} runs", percent, i, num_runs);
            use std::io::Write;
            std::io::stdout().flush().unwrap();
        }
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
                survivors.push(config.clone());
                break;
            }
            day_number += 1;
        }
        total_days += day_number;
        *histogram.entry(day_number).or_insert(0) += 1;
    }
    // Clear progress bar and print newline
    if num_runs >= 20 {
        println!("\rProgress: [100%] {}/{} runs", num_runs, num_runs);
    }
    println!("Monte Carlo Results ({} runs, {} day cap):", num_runs, day_cap);
    println!("  Survived {} days: {} times ({:.1}%)", day_cap, survived, (survived as f64 / num_runs as f64) * 100.0);
    println!("  Average days survived: {:.2}", total_days as f64 / num_runs as f64);
    print_histogram(&histogram, num_runs, day_cap);
    // Analyze survivors
    if survived > 0 {
        let mut soil_type_count = [0; 2];
        let mut num_plants_count = HashMap::new();
        let mut soil_kg_count = HashMap::new();
        let mut window_proximity_count = HashMap::new();
        let mut water_liters_count = HashMap::new();
        for config in &survivors {
            match config.soil_type {
                SoilType::Porous => soil_type_count[0] += 1,
                SoilType::NonPorous => soil_type_count[1] += 1,
            }
            *num_plants_count.entry(config.num_plants).or_insert(0) += 1;
            *soil_kg_count.entry(config.soil_kg).or_insert(0) += 1;
            *window_proximity_count.entry(config.window_proximity).or_insert(0) += 1;
            *water_liters_count.entry(config.water_liters).or_insert(0) += 1;
        }
        println!("\nMost common variables among survivors:");
        println!("  Soil type: Porous {} / NonPorous {}", soil_type_count[0], soil_type_count[1]);
        let top = |map: &HashMap<_,_>| map.iter().max_by_key(|e| e.1).map(|(k,v)| (*k,*v));
        if let Some((val, count)) = top(&num_plants_count) {
            println!("  Most common num_plants: {} ({} survivors)", val, count);
        }
        if let Some((val, count)) = top(&soil_kg_count) {
            println!("  Most common soil_kg: {} ({} survivors)", val, count);
        }
        if let Some((val, count)) = top(&window_proximity_count) {
            println!("  Most common window_proximity: {} ({} survivors)", val, count);
        }
        if let Some((val, count)) = top(&water_liters_count) {
            println!("  Most common water_liters: {} ({} survivors)", val, count);
        }
    }
}

fn run_v2_montecarlo(num_runs: usize, day_cap: usize) {
    let num_runs = num_runs.min(100_000); // limit to 100,000
    let day_cap = day_cap.min(1000); // limit to 1,000
    
    use crate::v2::config::V2Config;
    use crate::v2::state::EcosystemStateV2;
    use crate::v2::simulation::{update_ecosystem_v2, is_ecosystem_collapsed};
    let num_runs = num_runs.min(100_000);
    let day_cap = day_cap.min(1000);
    let mut survived = 0;
    let mut total_days = 0;
    let mut histogram: BTreeMap<usize, usize> = BTreeMap::new();
    for i in 0..num_runs {
        // Show progress bar
        if num_runs >= 20 && i % (num_runs / 100).max(1) == 0 {
            let percent = (i * 100) / num_runs;
            print!("\rProgress: [{:3}%] {}/{} runs", percent, i, num_runs);
            use std::io::Write;
            std::io::stdout().flush().unwrap();
        }

        let mut rng = rand::thread_rng();
        let mut config = V2Config::default();
        config.window_proximity = rng.gen_range(1..=6);
        config.water_liters = rng.gen_range(1..=10) as f32;
        config.rocks = rng.gen_range(0..=5);
        config.num_microbes = rng.gen_range(500..=2000);
        config.num_worms = rng.gen_range(1..=10);
        config.num_shrimp = rng.gen_range(1..=5);
        config.initial_temp = rng.gen_range(15.0..=30.0);
        config.initial_humidity = rng.gen_range(30.0..=90.0);
        let mut state = EcosystemStateV2::new(&config);
        let mut day = 1;
        let difficulty = rng.gen_range(0.6..=1.0);
        loop {
            let is_day = day % 2 == 0;
            update_ecosystem_v2(&config, &mut state, is_day, difficulty);
            if is_ecosystem_collapsed(&state) {
                break;
            }
            if day >= day_cap {
                survived += 1;
                break;
            }
            day += 1;
        }
        total_days += day;
        *histogram.entry(day).or_insert(0) += 1;
    }
    if num_runs >= 20 {
        println!("\rProgress: [100%] {}/{} runs", num_runs, num_runs);
    }
    println!("V2 Monte Carlo Results ({} runs, {} day cap):", num_runs, day_cap);
    println!("  Survived {} days: {} times ({:.1}%)", day_cap, survived, (survived as f64 / num_runs as f64) * 100.0);
    println!("  Average days survived: {:.2}", total_days as f64 / num_runs as f64);
    print_histogram(&histogram, num_runs, day_cap);
}

fn print_histogram(histogram: &BTreeMap<usize, usize>, num_runs: usize, day_cap: usize) {
    println!("\nHistogram of days survived before collapse:");
    for (days, count) in histogram {
        let bar = "#".repeat((*count * 50 / num_runs).max(1));
        let label = if *days == day_cap {
            format!("{}+", days)
        } else {
            format!("{}", days)
        };
        println!("  {:>4} days: {:>4} | {}", label, count, bar);
    }
}
