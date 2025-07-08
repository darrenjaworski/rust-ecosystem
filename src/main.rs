mod v1 {
    pub mod config;
    pub mod state;
    pub mod simulation;
    pub mod game;
    pub mod input;
}

// Use the new modular v2 structure
mod v2;

// Temporarily disable montecarlo for v2 compatibility
#[cfg(feature = "v1-montecarlo")]
mod montecarlo;

use v1::config::setup_game;
use v1::state::EcosystemState;
use v1::game::run_game;

#[cfg(feature = "v1-montecarlo")]
use montecarlo::run_montecarlo_simulations;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && args[1] == "v2" {
        if args.len() > 2 && args[2] == "montecarlo" {
            let num_runs = if args.len() > 3 {
                args[3].parse::<usize>().unwrap_or(1000)
            } else {
                1000
            };
            let day_cap = if args.len() > 4 {
                args[4].parse::<usize>().unwrap_or(30)
            } else {
                30
            };
            
            let mc_config = v2::montecarlo::MonteCarloConfig {
                num_runs,
                day_cap,
                difficulty_range: (0.3, 0.7),
                randomize_environment: true,
                randomize_organisms: true,
                show_progress: true,
            };
            
            let results = v2::montecarlo::run_monte_carlo_v2(mc_config);
            v2::montecarlo::print_monte_carlo_results(&results);
        } else {
            v2::game::run_game_v2();
        }
    } else if args.len() > 1 && args[1] == "montecarlo" {
        #[cfg(feature = "v1-montecarlo")]
        {
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
        }
        
        #[cfg(not(feature = "v1-montecarlo"))]
        {
            println!("Monte Carlo simulation temporarily disabled during v2 refactoring");
            println!("Use 'cargo run' to play the regular game or 'cargo run v2' for the new version");
        }
    } else {
        let config = setup_game();
        let state = EcosystemState::new();
        run_game(config, state);
    }
}
