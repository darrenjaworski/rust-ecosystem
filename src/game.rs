use crate::config::{GameConfig};
use crate::state::{EcosystemState, print_state, print_graphs};
use crate::simulation::update_ecosystem;
use crate::input::get_user_input;

pub fn run_game(mut config: GameConfig, mut state: EcosystemState) {
    println!("
Ecosystem setup complete.");
    println!("
--- Initial State ---");
    print_state(&state);

    let mut day_number = 1;
    let mut prev_state = state.clone();
    loop {
        // Day Cycle (10 intervals)
        for _ in 1..=10 {
            update_ecosystem(&config, &mut state, true);
        }

        // Night Cycle (6 intervals)
        for _ in 1..=6 {
            update_ecosystem(&config, &mut state, false);
        }

        println!("\n--- End of Day {} ---", day_number);
        print_state(&state);
        println!("  Window proximity (distance from window): {}", config.window_proximity);
        print_graphs(&state, Some(&prev_state));
        prev_state = state.clone();

        // Check for win/loss conditions
        if state.plant_size <= 0.0 {
            println!("
All your plants have died! Your ecosystem collapsed.");
            break;
        }
        if state.oxygen < 5.0 {
            println!("
Oxygen levels are critical! Your ecosystem collapsed.");
            break;
        }
        if day_number >= 30 {
            println!("
Congratulations! You have maintained a balanced ecosystem for 30 days!");
            break;
        }

        let mut should_exit = false;
        loop {
            let action = get_user_input(
                "\nChoose an action for the next day:\n1. Move closer to window\n2. Move further from window\n3. Open the bottle and intervene\nType 'exit' to quit.\n(Press Enter to do nothing)",
            );

            match action.trim() {
                "1" => {
                    if config.window_proximity > 1 {
                        config.window_proximity -= 1;
                        println!("You moved the bottle closer to the window. Proximity: {}", config.window_proximity);
                    } else {
                        println!("The bottle is already as close as it can get.");
                    }
                    break;
                }
                "2" => {
                    if config.window_proximity < 5 {
                        config.window_proximity += 1;
                        println!("You moved the bottle further from the window. Proximity: {}", config.window_proximity);
                    } else {
                        println!("The bottle is already as far as it can get.");
                    }
                    break;
                }
                "3" => {
                    // Open the bottle: reset oxygen, allow add plant or water
                    state.oxygen = 21.0;
                    println!("You opened the bottle. Oxygen reset to 21.0%.");
                    let sub_action = get_user_input("Would you like to add a plant (p) or add 1L water (w)? (Enter to skip)");
                    match sub_action.trim() {
                        "p" | "P" => {
                            if config.num_plants < 5 {
                                config.num_plants += 1;
                                println!("You added a plant. Total plants: {}", config.num_plants);
                            } else {
                                println!("You can't add more than 5 plants.");
                            }
                        }
                        "w" | "W" => {
                            if config.water_liters < 10 {
                                config.water_liters += 1;
                                println!("You added 1 liter of water. Total water: {}L", config.water_liters);
                            } else {
                                println!("The bottle is full. You can't add more water.");
                            }
                        }
                        "" => {
                            println!("No further intervention this day.");
                        }
                        _ => {
                            println!("Invalid choice. No intervention performed.");
                        }
                    }
                    break;
                }
                "exit" => {
                    should_exit = true;
                    break;
                }
                "" => {
                    println!("You do nothing and let the ecosystem evolve.");
                    break;
                }
                _ => {
                    println!("Invalid choice. Please enter 1, 2, 3, press Enter, or type 'exit'.");
                }
            }
        }

        if should_exit {
            break;
        }
        day_number += 1;
    }
}