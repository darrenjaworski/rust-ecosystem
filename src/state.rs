use colored::*;

#[derive(Debug)]
pub struct EcosystemState {
    pub ph: f32,
    pub temperature: f32,
    pub humidity: f32,
    pub plant_size: f32,
    pub microbial_levels: f32,
    // Air composition
    pub nitrogen: f32,
    pub co2: f32,
    pub oxygen: f32,
}

impl EcosystemState {
    pub fn new() -> Self {
        EcosystemState {
            ph: 7.0,
            temperature: 20.0, // Celsius
            humidity: 50.0,    // Percent
            plant_size: 5.0,   // start with slightly larger plants
            microbial_levels: 5.0, // start with more microbes
            nitrogen: 73.0,    // slightly less N2
            co2: 2.0,          // much higher starting CO2
            oxygen: 25.0,      // higher starting O2
        }
    }
}

fn colorize_value(value: f32, good_range: (f32, f32), warn_range: (f32, f32)) -> ColoredString {
    let s = format!("{:.2}", value);
    if value >= good_range.0 && value <= good_range.1 {
        s.green()
    } else if value >= warn_range.0 && value <= warn_range.1 {
        s.yellow()
    } else {
        s.red()
    }
}

pub fn print_state(state: &EcosystemState) {
    println!("  Temperature:      {}°C", colorize_value(state.temperature, (18.0, 28.0), (15.0, 35.0)));
    println!("  Humidity:         {}%", colorize_value(state.humidity, (40.0, 70.0), (30.0, 85.0)));
    println!("  pH:               {}", colorize_value(state.ph, (6.5, 7.5), (6.0, 8.0)));
    println!("  Plant Size:       {}", colorize_value(state.plant_size, (10.0, 80.0), (5.0, 95.0)));
    println!("  Microbial Levels: {}", colorize_value(state.microbial_levels, (0.0, 50.0), (50.0, 75.0)));
    println!(
        "  Air: {}% N2, {}% O2, {}% CO2",
        format!("{:.2}", state.nitrogen).cyan(),
        colorize_value(state.oxygen, (18.0, 25.0), (10.0, 30.0)),
        colorize_value(state.co2, (0.1, 1.0), (0.05, 2.0))
    );
}

fn print_bar(label: &str, value: f32, max_value: f32, color: Color) {
    let bar_width = 20;
    let filled_width = ((value / max_value) * bar_width as f32).round() as usize;
    let bar = "█".repeat(filled_width) + &"-".repeat(bar_width - filled_width);
    println!("  {:16} [{}]", label, bar.color(color));
}

pub fn print_graphs(state: &EcosystemState) {
    println!(
"--- Visual Indicators ---");
    print_bar("Temperature", state.temperature, 45.0, Color::Yellow);
    print_bar("Humidity", state.humidity, 100.0, Color::Green);
    print_bar("Plant Size", state.plant_size, 100.0, Color::Green);
    print_bar("Oxygen", state.oxygen, 30.0, Color::Green);
    println!();
}