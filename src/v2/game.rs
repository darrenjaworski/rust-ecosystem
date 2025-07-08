// v2/game.rs
// Game loop and CLI for v2 - updated for refactored system

use crate::v2::config::V2Config;
use crate::v2::state::EcosystemStateV2;
use crate::v2::simulation_refactored::update_ecosystem_v2;
use crate::v2::traits::{EcosystemDisplay, CollapseDetection, EcosystemValidation};

pub fn run_game_v2() {
    println!("🧪 Rust Ecosystem v2 - Refactored Edition");
    println!("==========================================");
    
    // Create config with user input
    let config = setup_game_v2();
    
    // Create initial state
    let mut state = match EcosystemStateV2::new(&config) {
        Ok(state) => state,
        Err(e) => {
            println!("Error creating ecosystem: {}", e);
            return;
        }
    };
    
    let mut day = 0;
    let goal_days = 30;
    
    println!("\n🎯 Goal: Survive {} days without ecosystem collapse!", goal_days);
    println!("{}", state.display_detailed());
    
    loop {
        day += 1;
        let is_day = day % 2 == 1; // Odd days are day, even are night
        
        println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("🌅 Day {} ({}) 🌅", (day + 1) / 2, if is_day { "Daytime" } else { "Nighttime" });
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        
        // Update ecosystem
        if let Err(e) = update_ecosystem_v2(&config, &mut state, is_day) {
            println!("❌ Simulation error: {}", e);
            break;
        }
        
        // Display status
        println!("{}", state.display_summary());
        
        // Check for collapse
        if state.is_collapsed() {
            println!("\n💀 ECOSYSTEM COLLAPSE! 💀");
            let reasons = state.collapse_reasons();
            for reason in reasons {
                println!("   • {}", reason);
            }
            println!("You survived {} half-days ({} full days)", day, day / 2);
            break;
        }
        
        // Check for warnings
        let warnings = state.health_warnings();
        if !warnings.is_empty() {
            println!("\n⚠️  Health Warnings:");
            for warning in warnings {
                println!("   • {}", warning);
            }
        }
        
        // Show collapse risk
        let risk = state.collapse_risk();
        if risk > 0.3 {
            println!("🚨 Collapse Risk: {:.1}%", risk * 100.0);
        }
        
        // User action (only during day)
        if is_day {
            if !get_user_action() {
                println!("👋 Game ended by user");
                break;
            }
        }
        
        // Check win condition
        if day >= goal_days * 2 { // *2 because we count half-days
            println!("\n🎉 VICTORY! 🎉");
            println!("You successfully maintained your ecosystem for {} days!", goal_days);
            println!("Final ecosystem state:");
            println!("{}", state.display_detailed());
            break;
        }
        
        // Sleep a bit for dramatic effect
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}

fn setup_game_v2() -> V2Config {
    use std::io::{self, Write};
    
    println!("\n🔧 Ecosystem Configuration");
    println!("==========================");
    
    let mut config = V2Config::new();
    
    print!("Choose difficulty [1=Easy, 2=Medium, 3=Hard, 4=Extreme] (default: 2): ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    let difficulty_level = match input.trim() {
        "1" => 0.2,
        "2" => 0.5,
        "3" => 0.8,
        "4" => 1.0,
        _ => 0.5,
    };
    
    match V2Config::with_difficulty(difficulty_level) {
        Ok(new_config) => config = new_config,
        Err(e) => println!("Error setting difficulty: {}, using default", e),
    }
    
    print!("Enter a seed for reproducible results (or press Enter for random): ");
    io::stdout().flush().unwrap();
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    
    if !input.trim().is_empty() {
        if let Ok(seed) = input.trim().parse::<u64>() {
            println!("Using seed: {}", seed);
            // We could implement seeded generation here if needed
        }
    }
    
    println!("\n✅ Configuration complete!");
    println!("   Difficulty: {:.1}%", difficulty_level * 100.0);
    println!("   Organisms: {} microbes, {} worms, {} shrimp", 
             config.organisms.microbes.initial_count,
             config.organisms.worms.initial_count,
             config.organisms.shrimp.initial_count);
    println!("   Environment: {:.1}°C, {:.1}% humidity, {:.1}L water",
             config.environment.initial_temperature.celsius(),
             config.environment.initial_humidity.percentage(),
             config.environment.water_volume.value());
    
    config
}

fn get_user_action() -> bool {
    use std::io::{self, Write};
    
    println!("\n🎮 What would you like to do?");
    println!("   [Enter] Continue to next day");
    println!("   [s] Show detailed status");
    println!("   [q] Quit game");
    
    print!("Action: ");
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    match input.trim().to_lowercase().as_str() {
        "q" | "quit" => false,
        "s" | "status" => {
            // This would show detailed status if we had state access
            println!("📊 Detailed status not implemented yet");
            true
        }
        _ => true,
    }
}