use crate::input::get_user_input;

#[derive(Debug)]
pub enum SoilType {
    Porous,
    NonPorous,
}

#[derive(Debug)]
pub struct GameConfig {
    pub soil_type: SoilType,
    pub num_plants: u32,
    pub soil_kg: u32,
    pub window_proximity: u32,
    pub water_liters: u32,
}

pub fn setup_game() -> GameConfig {
    println!("Welcome to your ecosystem in a bottle!");
    println!("Let's set up your ecosystem.");

    let soil_type = loop {
        let input = get_user_input("Choose soil type (porous/non-porous):");
        match input.to_lowercase().as_str() {
            "porous" => break SoilType::Porous,
            "non-porous" | "non porous" => break SoilType::NonPorous,
            _ => println!("Invalid input. Please choose 'porous' or 'non-porous'."),
        }
    };

    let num_plants = loop {
        let input = get_user_input("Enter number of plants (2-5):");
        match input.parse::<u32>() {
            Ok(n) if n >= 2 && n <= 5 => break n,
            _ => println!("Invalid input. Please enter a number between 2 and 5."),
        }
    };

    let soil_kg = loop {
        let input = get_user_input("Enter amount of soil in kg (10-30):");
        match input.parse::<u32>() {
            Ok(n) if n >= 10 && n <= 30 => break n,
            _ => println!("Invalid input. Please enter a number between 10 and 30."),
        }
    };

    let window_proximity = loop {
        let input = get_user_input("How close to the window (1-5, 1 is closest):");
        match input.parse::<u32>() {
            Ok(n) if n >= 1 && n <= 5 => break n,
            _ => println!("Invalid input. Please enter a number between 1 and 5."),
        }
    };

    let water_liters = loop {
        let input = get_user_input("Enter amount of water in liters (1-5):");
        match input.parse::<u32>() {
            Ok(n) if n >= 1 && n <= 5 => break n,
            _ => println!("Invalid input. Please enter a number between 1 and 5."),
        }
    };

    GameConfig {
        soil_type,
        num_plants,
        soil_kg,
        window_proximity,
        water_liters,
    }
}