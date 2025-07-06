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
            // Simple v2 Monte Carlo: run 100 v2 games, print summary
            let runs = 100;
            let mut survivors = 0;
            for _ in 0..runs {
                let config = v2::config::V2Config::default();
                let mut state = v2::state::EcosystemStateV2::new(&config);
                let mut day = 0;
                let mut alive = true;
                while day < 30 {
                    let is_day = day % 2 == 0;
                    v2::simulation::update_ecosystem_v2(&config, &mut state, is_day);
                    if state.plant_biomass <= 0.0 || state.microbe_pop <= 0.0 || state.worm_pop <= 0.0 || state.shrimp_pop <= 0.0 {
                        alive = false;
                        break;
                    }
                    day += 1;
                }
                if alive { survivors += 1; }
            }
            println!("v2 Monte Carlo: {}/{} survived 30 days", survivors, runs);
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
        run_montecarlo_simulations(num_runs, day_cap);
    } else {
        let config = setup_game();
        let state = EcosystemState::new();
        run_game(config, state);
    }
}
