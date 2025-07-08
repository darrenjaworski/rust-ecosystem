# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust ecosystem simulation with two distinct versions:
- **v1**: Interactive CLI game where players make daily decisions to keep their bottle ecosystem alive
- **v2**: Mathematical simulation using ODEs (Ordinary Differential Equations) with Monte Carlo analysis capabilities

Both versions model a closed ecosystem with plants, microbes, worms, and shrimp interacting through complex biochemical processes.

## Build and Run Commands

### Basic Operations
```bash
# Build project
cargo build --release

# Run v1 interactive game
cargo run

# Run v2 simulation
cargo run v2

# Run v2 Monte Carlo analysis
cargo run v2 montecarlo <runs> <days>
# Example: cargo run v2 montecarlo 1000 30
```

### Development Commands
```bash
# Check code quality
cargo clippy

# Run tests (if any exist)
cargo test

# Format code
cargo fmt
```

## Architecture Overview

### Dual Version System
The codebase maintains two parallel implementations:

**v1** (`src/v1/`): Simple game-focused implementation
- `config.rs` - Game setup and player choices
- `game.rs` - Turn-based game loop
- `simulation.rs` - Basic ecosystem rules
- `state.rs` - Simple state representation
- `input.rs` - Player input handling

**v2** (`src/v2/`): Scientific simulation with sophisticated modeling
- `config/` - Modular configuration system with difficulty scaling
- `organisms/` - Detailed organism-specific simulation modules
- `simulation_refactored.rs` - ODE-based ecosystem engine
- `montecarlo.rs` - Statistical analysis and batch simulation
- `types.rs` - Type-safe ecosystem values with validation
- `errors.rs` - Comprehensive error handling
- `traits.rs` - Ecosystem behavior abstractions

### Key Architectural Patterns

**Type Safety in v2**: All ecosystem values use validated newtypes (Ph, Temperature, Population) that prevent invalid states at compile time.

**Modular Organism Design**: Each organism type (plants, microbes, worms, shrimp) has its own module with specific update functions and health monitoring.

**Configuration System**: v2 uses a hierarchical config system with difficulty scaling that affects survival rates and complexity.

**Mathematical Foundation**: v2 implements real ecological ODEs modeling photosynthesis, respiration, nitrogen fixation, population dynamics, and environmental chemistry.

## Monte Carlo Analysis

The v2 system includes comprehensive Monte Carlo capabilities for ecosystem survival analysis:
- Randomized parameter generation within realistic ranges
- Detailed collapse reason tracking
- Survival histogram analysis
- Success factor identification
- Progress tracking for long runs

Current survival rates are intentionally low (~4%) to simulate the difficulty of maintaining real closed ecosystems.

## Version Selection Logic

The `main.rs` router determines which system to run:
- No args: v1 interactive game
- `v2`: v2 interactive simulation
- `v2 montecarlo <runs> <days>`: v2 Monte Carlo analysis
- Legacy `montecarlo` arg: Disabled v1 Monte Carlo (feature-gated)

## Development Context

This project evolved from a simple educational game into a sophisticated research tool. The v2 system represents a complete architectural refactoring focused on:
- Mathematical accuracy using real ecological principles
- Modular design for extensibility
- Type safety to prevent ecosystem modeling errors
- Statistical analysis capabilities for research applications

The mathematical models are based on real ecological research and include proper stoichiometry for biochemical processes like photosynthesis and nitrogen fixation.