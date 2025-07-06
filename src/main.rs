mod v1 {
    pub mod config;
    pub mod state;
    pub mod simulation;
    pub mod game;
    pub mod input;
}
mod v2 {
    pub mod config;
    pub mod state;
    pub mod simulation;
    pub mod game;
    pub mod input;
}
mod montecarlo;

use v1::config::setup_game;
use v1::state::EcosystemState;
use v1::game::run_game;
use montecarlo::run_montecarlo_simulations;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && args[1] == "v2" {
        if args.len() > 2 && args[2] == "montecarlo" {
            let runs = if args.len() > 3 {
                args[3].parse::<usize>().unwrap_or(100)
            } else {
                100
            };
            let days = if args.len() > 4 {
                args[4].parse::<usize>().unwrap_or(30)
            } else {
                30
            };
            let mut survivors = 0;
            let mut total_final_biomass = 0.0;
            let mut max_biomass = f32::MIN;
            let mut min_biomass = f32::MAX;
            let mut total_days_alive = 0;
            for _ in 0..runs {
                let config = v2::config::V2Config::default();
                let mut state = v2::state::EcosystemStateV2::new_with_seed(&config, rand::random());
                let mut day = 0;
                let mut alive = true;
                let difficulty = 0.7; // hard by default for montecarlo
                while day < days {
                    let is_day = day % 2 == 0;
                    v2::simulation::update_ecosystem_v2(&config, &mut state, is_day, difficulty);
                    if state.plant_biomass <= 0.0 || state.microbe_pop <= 0.0 || state.worm_pop <= 0.0 || state.shrimp_pop <= 0.0 {
                        alive = false;
                        break;
                    }
                    day += 1;
                }
                total_days_alive += day;
                total_final_biomass += state.plant_biomass.max(0.0);
                if state.plant_biomass > max_biomass { max_biomass = state.plant_biomass; }
                if state.plant_biomass < min_biomass { min_biomass = state.plant_biomass; }
                if alive { survivors += 1; }
            }
            println!("v2 Monte Carlo: {}/{} survived {} days", survivors, runs, days);
            println!("  Avg. final plant biomass: {:.2}", total_final_biomass / runs as f32);
            println!("  Max plant biomass: {:.2}", max_biomass);
            println!("  Min plant biomass: {:.2}", min_biomass);
            println!("  Avg. days alive: {:.2}", total_days_alive as f32 / runs as f32);
        } else {
            v2::game::run_game_v2();
        }
    } else if args.len() > 1 && args[1] == "montecarlo" {
        let num_runs = if args.len() > 2 {
            args[2].parse::<usize>().unwrap_or(100)
        } else {
            100
        };
        let day_cap = if args.len() > 3 {
            args[3].parse::<usize>().unwrap_or(30)
        } else {
            30
        };
        run_montecarlo_simulations(num_runs, day_cap, montecarlo::MonteCarloModel::V1);
    } else {
        let config = setup_game();
        let state = EcosystemState::new();
        run_game(config, state);
    }
}
