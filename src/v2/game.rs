// v2/game.rs
// Game loop and CLI for v2

use crate::v2::config::V2Config;
use crate::v2::state::EcosystemStateV2;
use crate::v2::simulation::update_ecosystem_v2;

pub fn run_game_v2() {
    use std::io::{self, Write};
    let mut config = V2Config::default();
    // Prompt for user-selectable options
    print!("Enter window proximity (1=closest, 6=farthest) [default: {}]: ", config.window_proximity);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    if let Ok(val) = input.trim().parse::<u8>() {
        config.window_proximity = val.clamp(1, 6);
    }
    print!("Enter water liters [default: {:.2}]: ", config.water_liters);
    io::stdout().flush().unwrap();
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    if let Ok(val) = input.trim().parse::<f32>() {
        config.water_liters = val.max(0.1);
    }
    print!("Enter starting plant biomass [default: 1.0]: ");
    io::stdout().flush().unwrap();
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let user_plant_biomass = input.trim().parse::<f32>().ok().filter(|v| *v > 0.0).unwrap_or(1.0);
    print!("Enter difficulty (0.0 = easy, 1.0 = hard) [default: 0.5]: ");
    io::stdout().flush().unwrap();
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let difficulty = input.trim().parse::<f32>().ok().filter(|v| *v >= 0.0 && *v <= 1.0).unwrap_or(0.5);
    print!("Enter a seed (u64) for your ecosystem, or leave blank for random: ");
    io::stdout().flush().unwrap();
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let seed = input.trim().parse::<u64>().unwrap_or_else(|_| rand::random());
    let mut state = EcosystemStateV2::new_with_seed(&config, seed);
    state.plant_biomass = user_plant_biomass;
    let mut day = 0;
    let mut prev_temp = state.temperature;
    let mut prev_humidity = state.humidity;
    let mut prev_biomass = state.plant_biomass;
    let mut prev_o2 = state.air_o2;
    loop {
        let is_day = day % 2 == 0;
        update_ecosystem_v2(&config, &mut state, is_day, difficulty);
        // --- End of Day Styling ---
        println!("\n--- End of Day {} ---", day + 1);
        println!("  Temperature:      {:6.2}°C", state.temperature);
        println!("  Humidity:         {:6.2}%", state.humidity);
        println!("  pH:               {:6.2}", state.soil_ph);
        println!("  Plant Biomass:    {:6.2}", state.plant_biomass);
        println!("  Microbe Pop:      {:6.2}", state.microbe_pop);
        println!("  Worm Pop:         {:6.2}", state.worm_pop);
        println!("  Shrimp Pop:       {:6.2}", state.shrimp_pop);
        println!("  Soil Nitrogen:    {:6.2}", state.soil_nitrogen);
        println!("  Soil Moisture:    {:6.2}", state.soil_moisture);
        println!("  Soil Aeration:    {:6.2}", state.soil_aeration);
        println!("  Detritus:         {:6.2}", state.detritus);
        println!("  Water (L):        {:6.2}", state.water_liters);
        println!("  Water O2:         {:6.2}", state.water_o2);
        println!("  Air: {:5.2}% N2, {:5.2}% O2, {:5.2}% CO2", state.air_n2, state.air_o2, state.air_co2);
        println!("  Window proximity (distance from window): {}", config.window_proximity);
        println!("  Rocks:            {}", state.rocks);
        println!("--- Visual Indicators ---");
        print_bar("Temperature", state.temperature, prev_temp, 5.0, 45.0);
        print_bar("Humidity", state.humidity, prev_humidity, 0.0, 100.0);
        print_bar("Plant Biomass", state.plant_biomass, prev_biomass, 0.0, 100.0);
        print_bar("Oxygen", state.air_o2, prev_o2, 0.0, 21.0);
        // User intervention
        println!("\n[c] Closer to light  [f] Farther from light  [n] None  [q] Quit");
        print!("Your action: ");
        io::stdout().flush().unwrap();
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim() {
            "c" => { if config.window_proximity > 1 { config.window_proximity -= 1; } },
            "f" => { if config.window_proximity < 6 { config.window_proximity += 1; } },
            "q" => break,
            _ => {},
        }
        day += 1;
        prev_temp = state.temperature;
        prev_humidity = state.humidity;
        prev_biomass = state.plant_biomass;
        prev_o2 = state.air_o2;
        // Collapse check
        if crate::v2::simulation::is_ecosystem_collapsed(&state) {
            println!("\n*** ECOSYSTEM COLLAPSE: One or more populations have crashed. ***");
            break;
        }
        if day > 30 { break; }
    }
}

fn print_bar(label: &str, value: f32, prev: f32, min: f32, max: f32) {
    let width = 20;
    let percent = ((value - min) / (max - min)).clamp(0.0, 1.0);
    let bar_count = (percent * width as f32).round() as usize;
    let bar = "█".repeat(bar_count) + &"-".repeat(width - bar_count);
    let diff = value - prev;
    let diff_percent = if prev.abs() > 1e-6 { (diff / prev) * 100.0 } else { 0.0 };
    let sign = if diff_percent >= 0.0 { "+" } else { "" };
    println!("  {:15} [{}] ({}{:.1}%)", label, bar, sign, diff_percent);
}
