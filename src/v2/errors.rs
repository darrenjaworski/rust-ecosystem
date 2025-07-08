// v2/errors.rs
// Error types for ecosystem simulation

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum EcosystemError {
    PopulationCollapse { population: String },
    EnvironmentalFailure { parameter: String, value: f32 },
    ConfigurationError { message: String },
    ValidationError(crate::v2::types::ValidationError),
    SimulationError { message: String },
}

impl fmt::Display for EcosystemError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EcosystemError::PopulationCollapse { population } => {
                write!(f, "Population collapsed: {}", population)
            }
            EcosystemError::EnvironmentalFailure { parameter, value } => {
                write!(f, "Environmental parameter '{}' reached critical value: {}", parameter, value)
            }
            EcosystemError::ConfigurationError { message } => {
                write!(f, "Configuration error: {}", message)
            }
            EcosystemError::ValidationError(e) => {
                write!(f, "Validation error: {}", e)
            }
            EcosystemError::SimulationError { message } => {
                write!(f, "Simulation error: {}", message)
            }
        }
    }
}

impl std::error::Error for EcosystemError {}

impl From<crate::v2::types::ValidationError> for EcosystemError {
    fn from(error: crate::v2::types::ValidationError) -> Self {
        EcosystemError::ValidationError(error)
    }
}

pub type EcosystemResult<T> = Result<T, EcosystemError>;

#[derive(Debug, Clone, PartialEq)]
pub enum CollapseReason {
    PlantsDied,
    MicrobesDied,
    WormsDied,
    ShrimpDied,
    OxygenDepletion,
    PhImbalance,
    TemperatureExtreme,
    Multiple(Vec<CollapseReason>),
}

impl fmt::Display for CollapseReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CollapseReason::PlantsDied => write!(f, "All plants died"),
            CollapseReason::MicrobesDied => write!(f, "All microbes died"),
            CollapseReason::WormsDied => write!(f, "All worms died"),
            CollapseReason::ShrimpDied => write!(f, "All shrimp died"),
            CollapseReason::OxygenDepletion => write!(f, "Oxygen levels too low"),
            CollapseReason::PhImbalance => write!(f, "pH levels became toxic"),
            CollapseReason::TemperatureExtreme => write!(f, "Temperature became extreme"),
            CollapseReason::Multiple(reasons) => {
                write!(f, "Multiple failures: ")?;
                for (i, reason) in reasons.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", reason)?;
                }
                Ok(())
            }
        }
    }
}