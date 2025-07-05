# Rust Ecosystem-in-a-Bottle Simulation Game

This project is a Rust CLI simulation game that models a closed ecosystem in a bottle. The goal is to prevent your ecosystem from collapsing before 30 days have elapsed.

## Features

- **Ecosystem Simulation:** Models plant growth, microbial activity, air composition, pH, temperature, humidity, and more.
- **Player Actions:** Each day, the player can move the bottle, open it to intervene, or do nothing.
- **Win/Loss Conditions:** Survive 30 days to win, or lose if all plants die or oxygen drops too low.
- **Monte Carlo Mode:** Run thousands of randomized simulations to analyze survivability and see a histogram of outcomes.
- **Progress Bar:** See a live status bar while running large Monte Carlo simulations.
- **Survivor Analysis:** After Monte Carlo runs, see which input variables are most common among surviving runs.
- **Modern CLI:** Colored output and ASCII bar graphs for ecosystem health.

## How to Run

1. **Build the project:**

   ```sh
   cargo build --release
   ```

2. **Play the game:**

   ```sh
   cargo run --release
   ```

3. **Run Monte Carlo simulation:**

   ```sh
   cargo run --release -- montecarlo <num_runs> <day_cap>
   ```

   - Example: `cargo run --release -- montecarlo 10000 30`
   - The simulation will show a progress bar and print a histogram and survivor analysis at the end.

## Project Structure

- `src/main.rs` — CLI entry point
- `src/config.rs` — Game configuration and setup
- `src/state.rs` — Ecosystem state representation and display
- `src/simulation.rs` — Core simulation logic and ecosystem update rules
- `src/game.rs` — Main game loop and player actions
- `src/input.rs` — User input handling
- `src/montecarlo.rs` — Monte Carlo simulation and analysis

## Balancing Philosophy

The game difficulty is balanced so that most simulations collapse around day 20. Can you survive to day 30?

## License

MIT
