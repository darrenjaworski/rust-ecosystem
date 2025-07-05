mod config;
mod state;
mod simulation;
mod game;
mod input;
mod montecarlo;

use config::setup_game;
use state::EcosystemState;
use game::run_game;
use montecarlo::run_montecarlo_simulations;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && args[1] == "montecarlo" {
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
