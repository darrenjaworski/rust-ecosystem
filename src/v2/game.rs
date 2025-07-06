// v2/game.rs
// Game loop and CLI for v2

use crate::v2::config::V2Config;
use crate::v2::state::EcosystemStateV2;
use crate::v2::simulation::update_ecosystem_v2;

pub fn run_game_v2() {
    let config = V2Config::default();
    let mut state = EcosystemStateV2::new(&config);
    let mut day = 0;
    loop {
        let is_day = day % 2 == 0;
        update_ecosystem_v2(&config, &mut state, is_day);
        // TODO: Add CLI, win/loss, and state display for v2
        println!("Day {}: T = {:.1}°C, H = {:.1}%", day, state.temperature, state.humidity);
        day += 1;
        if day > 30 { break; }
    }
}
