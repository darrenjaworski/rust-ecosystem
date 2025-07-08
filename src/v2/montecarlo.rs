// v2/montecarlo.rs
// Monte Carlo simulation for v2 ecosystem

use crate::v2::config::V2Config;
use crate::v2::state::EcosystemStateV2;
use crate::v2::simulation_refactored::update_ecosystem_v2;
use crate::v2::traits::{CollapseDetection, EcosystemValidation, EcosystemDisplay};
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct MonteCarloConfig {
    pub num_runs: usize,
    pub day_cap: usize,
    pub difficulty_range: (f32, f32),
    pub randomize_environment: bool,
    pub randomize_organisms: bool,
    pub show_progress: bool,
}

impl Default for MonteCarloConfig {
    fn default() -> Self {
        Self {
            num_runs: 1000,
            day_cap: 30,
            difficulty_range: (0.3, 0.7),
            randomize_environment: true,
            randomize_organisms: true,
            show_progress: true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SimulationResult {
    pub run_id: usize,
    pub survived: bool,
    pub days_survived: usize,
    pub collapse_reasons: Vec<crate::v2::errors::CollapseReason>,
    pub final_state: FinalStateSnapshot,
    pub config_snapshot: ConfigSnapshot,
}

#[derive(Debug, Clone)]
pub struct FinalStateSnapshot {
    pub plant_biomass: f32,
    pub microbe_pop: f32,
    pub worm_pop: f32,
    pub shrimp_pop: f32,
    pub soil_ph: f32,
    pub air_o2: f32,
    pub temperature: f32,
    pub humidity: f32,
}

#[derive(Debug, Clone)]
pub struct ConfigSnapshot {
    pub difficulty: f32,
    pub microbe_count: usize,
    pub worm_count: usize,
    pub shrimp_count: usize,
    pub water_volume: f32,
    pub rocks: usize,
    pub window_proximity: u8,
    pub initial_temp: f32,
    pub initial_humidity: f32,
}

#[derive(Debug)]
pub struct MonteCarloResults {
    pub results: Vec<SimulationResult>,
    pub survival_rate: f32,
    pub average_days_survived: f32,
    pub survival_histogram: HashMap<usize, usize>,
    pub collapse_reasons_frequency: HashMap<String, usize>,
    pub survivor_analysis: SurvivorAnalysis,
}

#[derive(Debug)]
pub struct SurvivorAnalysis {
    pub optimal_difficulty: Option<f32>,
    pub optimal_microbe_count: Option<usize>,
    pub optimal_environment: Option<ConfigSnapshot>,
    pub success_factors: Vec<(String, f32)>,
}

pub fn run_monte_carlo_v2(mc_config: MonteCarloConfig) -> MonteCarloResults {
    let mut results = Vec::with_capacity(mc_config.num_runs);
    let mut rng = StdRng::from_entropy();
    
    println!("🧪 Running {} Monte Carlo simulations for v2 ecosystem", mc_config.num_runs);
    println!("📊 Configuration:");
    println!("   Days to survive: {}", mc_config.day_cap);
    println!("   Difficulty range: {:.1}% - {:.1}%", 
             mc_config.difficulty_range.0 * 100.0, 
             mc_config.difficulty_range.1 * 100.0);
    println!("   Randomize environment: {}", mc_config.randomize_environment);
    println!("   Randomize organisms: {}", mc_config.randomize_organisms);
    println!();

    // Progress tracking
    let progress_interval = (mc_config.num_runs / 20).max(1);
    
    for run_id in 0..mc_config.num_runs {
        if mc_config.show_progress && run_id % progress_interval == 0 {
            let percent = (run_id as f32 / mc_config.num_runs as f32) * 100.0;
            print!("\r🔄 Progress: [{:>3.0}%] Running simulation {}/{}", 
                   percent, run_id + 1, mc_config.num_runs);
            std::io::Write::flush(&mut std::io::stdout()).unwrap();
        }

        let result = run_single_simulation(run_id, &mc_config, &mut rng);
        results.push(result);
    }
    
    if mc_config.show_progress {
        println!("\r✅ Completed {} simulations!                    ", mc_config.num_runs);
    }

    analyze_results(results, mc_config)
}

fn run_single_simulation(
    run_id: usize, 
    mc_config: &MonteCarloConfig, 
    rng: &mut StdRng
) -> SimulationResult {
    // Generate random configuration
    let config = generate_random_config(mc_config, rng);
    let config_snapshot = create_config_snapshot(&config, mc_config, rng);
    
    // Create initial state
    let seed = rng.r#gen::<u64>();
    let mut state = match EcosystemStateV2::new_with_seed(&config, seed) {
        Ok(state) => state,
        Err(_) => {
            // If seeded creation fails, use default
            EcosystemStateV2::new(&config).unwrap_or_else(|_| {
                // Last resort - create with default config
                let default_config = V2Config::new();
                EcosystemStateV2::new(&default_config).unwrap()
            })
        }
    };

    let mut days_survived = 0;
    let mut survived = false;
    let mut collapse_reasons = Vec::new();

    // Run simulation
    for day in 0..(mc_config.day_cap * 2) { // *2 for day/night cycles
        let is_day = day % 2 == 0;
        
        // Update ecosystem
        if let Err(_) = update_ecosystem_v2(&config, &mut state, is_day) {
            break;
        }

        // Check for collapse
        if state.is_collapsed() {
            collapse_reasons = state.collapse_reasons();
            break;
        }

        if is_day {
            days_survived += 1;
        }

        // Check if we've reached the goal
        if days_survived >= mc_config.day_cap {
            survived = true;
            break;
        }
    }

    let final_state = FinalStateSnapshot {
        plant_biomass: state.plant_biomass.value(),
        microbe_pop: state.microbe_pop.value(),
        worm_pop: state.worm_pop.value(),
        shrimp_pop: state.shrimp_pop.value(),
        soil_ph: state.soil_ph.value(),
        air_o2: state.air_o2.percentage(),
        temperature: state.temperature.celsius(),
        humidity: state.humidity.percentage(),
    };

    SimulationResult {
        run_id,
        survived,
        days_survived,
        collapse_reasons,
        final_state,
        config_snapshot,
    }
}

fn generate_random_config(mc_config: &MonteCarloConfig, rng: &mut StdRng) -> V2Config {
    let difficulty = rng.gen_range(mc_config.difficulty_range.0..=mc_config.difficulty_range.1);
    
    let mut config = match V2Config::with_difficulty(difficulty) {
        Ok(config) => config,
        Err(_) => V2Config::new(), // Fallback to default
    };

    if mc_config.randomize_environment {
        // Randomize environment parameters
        use crate::v2::config::environment::*;
        use crate::v2::types::*;
        
        let water_volume = rng.gen_range(0.2..=2.0);
        let rocks = rng.gen_range(0..=5);
        let window_proximity = rng.gen_range(1..=6);
        let temp = rng.gen_range(15.0..=30.0);
        let humidity = rng.gen_range(30.0..=90.0);
        
        if let Ok(env_config) = EnvironmentConfig::new(
            water_volume,
            rocks,
            window_proximity,
            temp,
            humidity,
            SoilType::Balanced,
        ) {
            config.environment = env_config;
        }
    }

    if mc_config.randomize_organisms {
        // Randomize organism counts
        use crate::v2::config::organisms::*;
        
        let microbe_count = rng.gen_range(100..=5000);
        let worm_count = rng.gen_range(1..=15);
        let shrimp_count = rng.gen_range(1..=8);
        let plant_biomass = rng.gen_range(0.5..=3.0);
        
        if let Ok(org_config) = OrganismConfig::new(
            microbe_count,
            worm_count,
            shrimp_count,
            plant_biomass,
        ) {
            config.organisms = org_config;
        }
    }

    config
}

fn create_config_snapshot(config: &V2Config, mc_config: &MonteCarloConfig, rng: &StdRng) -> ConfigSnapshot {
    ConfigSnapshot {
        difficulty: mc_config.difficulty_range.0, // Approximation
        microbe_count: config.organisms.microbes.initial_count,
        worm_count: config.organisms.worms.initial_count,
        shrimp_count: config.organisms.shrimp.initial_count,
        water_volume: config.environment.water_volume.value(),
        rocks: config.environment.rocks,
        window_proximity: config.environment.window_proximity,
        initial_temp: config.environment.initial_temperature.celsius(),
        initial_humidity: config.environment.initial_humidity.percentage(),
    }
}

fn analyze_results(results: Vec<SimulationResult>, mc_config: MonteCarloConfig) -> MonteCarloResults {
    let total_runs = results.len();
    let survivors: Vec<_> = results.iter().filter(|r| r.survived).collect();
    let survival_rate = survivors.len() as f32 / total_runs as f32;
    
    // Calculate average days survived
    let total_days: usize = results.iter().map(|r| r.days_survived).sum();
    let average_days_survived = total_days as f32 / total_runs as f32;
    
    // Create survival histogram
    let mut survival_histogram = HashMap::new();
    for result in &results {
        *survival_histogram.entry(result.days_survived).or_insert(0) += 1;
    }
    
    // Analyze collapse reasons
    let mut collapse_reasons_frequency = HashMap::new();
    for result in &results {
        if !result.survived {
            for reason in &result.collapse_reasons {
                let reason_str = format!("{}", reason);
                *collapse_reasons_frequency.entry(reason_str).or_insert(0) += 1;
            }
        }
    }
    
    // Survivor analysis
    let survivor_analysis = analyze_survivors(&survivors);
    
    MonteCarloResults {
        results,
        survival_rate,
        average_days_survived,
        survival_histogram,
        collapse_reasons_frequency,
        survivor_analysis,
    }
}

fn analyze_survivors(survivors: &[&SimulationResult]) -> SurvivorAnalysis {
    if survivors.is_empty() {
        return SurvivorAnalysis {
            optimal_difficulty: None,
            optimal_microbe_count: None,
            optimal_environment: None,
            success_factors: Vec::new(),
        };
    }

    // Find most common successful configurations
    let mut difficulty_sum = 0.0;
    let mut microbe_sum = 0;
    let mut success_factors = Vec::new();

    for survivor in survivors {
        difficulty_sum += survivor.config_snapshot.difficulty;
        microbe_sum += survivor.config_snapshot.microbe_count;
    }

    let optimal_difficulty = Some(difficulty_sum / survivors.len() as f32);
    let optimal_microbe_count = Some(microbe_sum / survivors.len());

    // Calculate success factors (simplified)
    let avg_microbes = survivors.iter().map(|s| s.config_snapshot.microbe_count as f32).sum::<f32>() / survivors.len() as f32;
    let avg_water = survivors.iter().map(|s| s.config_snapshot.water_volume).sum::<f32>() / survivors.len() as f32;
    let avg_rocks = survivors.iter().map(|s| s.config_snapshot.rocks as f32).sum::<f32>() / survivors.len() as f32;

    success_factors.push(("Average Microbes".to_string(), avg_microbes));
    success_factors.push(("Average Water Volume".to_string(), avg_water));
    success_factors.push(("Average Rocks".to_string(), avg_rocks));

    SurvivorAnalysis {
        optimal_difficulty,
        optimal_microbe_count,
        optimal_environment: survivors.first().map(|s| s.config_snapshot.clone()),
        success_factors,
    }
}

pub fn print_monte_carlo_results(results: &MonteCarloResults) {
    println!("\n🏆 MONTE CARLO RESULTS");
    println!("==========================================");
    println!("📈 Overall Statistics:");
    println!("   Total simulations: {}", results.results.len());
    println!("   Survival rate: {:.1}%", results.survival_rate * 100.0);
    println!("   Average days survived: {:.1}", results.average_days_survived);
    
    println!("\n📊 Survival Histogram:");
    let mut histogram_entries: Vec<_> = results.survival_histogram.iter().collect();
    histogram_entries.sort_by_key(|(days, _)| *days);
    
    for (days, count) in histogram_entries.iter().take(20) {
        let bar_length = (**count as f32 / results.results.len() as f32 * 50.0) as usize;
        let bar = "█".repeat(bar_length.max(1));
        println!("   Day {:2}: {:4} runs {}", days, count, bar);
    }
    
    println!("\n💀 Collapse Reasons:");
    let mut collapse_entries: Vec<_> = results.collapse_reasons_frequency.iter().collect();
    collapse_entries.sort_by_key(|(_, count)| std::cmp::Reverse(**count));
    
    for (reason, count) in collapse_entries.iter().take(10) {
        let percentage = **count as f32 / results.results.len() as f32 * 100.0;
        println!("   {:25}: {:4} ({:5.1}%)", reason, count, percentage);
    }
    
    println!("\n🎯 Survivor Analysis:");
    if let Some(difficulty) = results.survivor_analysis.optimal_difficulty {
        println!("   Optimal difficulty: {:.1}%", difficulty * 100.0);
    }
    if let Some(microbes) = results.survivor_analysis.optimal_microbe_count {
        println!("   Optimal microbe count: {}", microbes);
    }
    
    println!("\n✨ Success Factors (averages for survivors):");
    for (factor, value) in &results.survivor_analysis.success_factors {
        println!("   {}: {:.2}", factor, value);
    }
    
    println!("\n💡 Recommendations:");
    if results.survival_rate < 0.1 {
        println!("   • Very low survival rate - consider easier difficulty");
        println!("   • Check initial organism counts and environment");
    } else if results.survival_rate > 0.8 {
        println!("   • High survival rate - consider increasing difficulty");
    } else {
        println!("   • Balanced survival rate - good parameter range");
    }
    
    if let Some(optimal_microbes) = results.survivor_analysis.optimal_microbe_count {
        println!("   • Try starting with ~{} microbes for better success", optimal_microbes);
    }
}