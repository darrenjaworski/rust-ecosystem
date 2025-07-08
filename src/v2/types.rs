// v2/types.rs
// Type-safe wrappers for ecosystem values

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Biomass(f32);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Population(f32);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ph(f32);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Temperature(f32);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Humidity(f32);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Oxygen(f32);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CarbonDioxide(f32);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Nitrogen(f32);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WaterVolume(f32);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Moisture(f32);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Aeration(f32);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Detritus(f32);

// Error types for validation
#[derive(Debug, Clone, PartialEq)]
pub enum ValidationError {
    PhOutOfRange(f32),
    TemperatureOutOfRange(f32),
    HumidityOutOfRange(f32),
    NegativeValue(String),
    ZeroOrNegativePopulation(String),
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidationError::PhOutOfRange(value) => write!(f, "pH {} is out of range (0-14)", value),
            ValidationError::TemperatureOutOfRange(value) => write!(f, "Temperature {} is out of acceptable range", value),
            ValidationError::HumidityOutOfRange(value) => write!(f, "Humidity {} is out of range (0-100)", value),
            ValidationError::NegativeValue(param) => write!(f, "Parameter {} cannot be negative", param),
            ValidationError::ZeroOrNegativePopulation(pop) => write!(f, "Population {} must be positive", pop),
        }
    }
}

impl std::error::Error for ValidationError {}

// Biomass implementation
impl Biomass {
    pub fn new(value: f32) -> Result<Self, ValidationError> {
        if value < 0.0 {
            Err(ValidationError::NegativeValue("biomass".to_string()))
        } else {
            Ok(Biomass(value))
        }
    }

    pub fn value(&self) -> f32 {
        self.0
    }

    pub fn is_collapsed(&self) -> bool {
        self.0 <= 0.01
    }
}

// Population implementation
impl Population {
    pub fn new(value: f32) -> Result<Self, ValidationError> {
        if value <= 0.0 {
            Err(ValidationError::ZeroOrNegativePopulation("population".to_string()))
        } else {
            Ok(Population(value))
        }
    }

    pub fn value(&self) -> f32 {
        self.0
    }

    pub fn is_collapsed(&self) -> bool {
        self.0 <= 0.01
    }
}

// Ph implementation
impl Ph {
    pub fn new(value: f32) -> Result<Self, ValidationError> {
        if !(0.0..=14.0).contains(&value) {
            Err(ValidationError::PhOutOfRange(value))
        } else {
            Ok(Ph(value))
        }
    }

    pub fn value(&self) -> f32 {
        self.0
    }

    pub fn is_acidic(&self) -> bool {
        self.0 < 7.0
    }

    pub fn is_basic(&self) -> bool {
        self.0 > 7.0
    }
}

// Temperature implementation
impl Temperature {
    pub fn new(value: f32) -> Result<Self, ValidationError> {
        if !(-50.0..=60.0).contains(&value) {
            Err(ValidationError::TemperatureOutOfRange(value))
        } else {
            Ok(Temperature(value))
        }
    }

    pub fn celsius(&self) -> f32 {
        self.0
    }

    pub fn is_optimal(&self) -> bool {
        (20.0..=28.0).contains(&self.0)
    }
}

// Humidity implementation
impl Humidity {
    pub fn new(value: f32) -> Result<Self, ValidationError> {
        if !(0.0..=100.0).contains(&value) {
            Err(ValidationError::HumidityOutOfRange(value))
        } else {
            Ok(Humidity(value))
        }
    }

    pub fn percentage(&self) -> f32 {
        self.0
    }

    pub fn is_optimal(&self) -> bool {
        (40.0..=80.0).contains(&self.0)
    }
}

// Oxygen implementation
impl Oxygen {
    pub fn new(value: f32) -> Result<Self, ValidationError> {
        if value < 0.0 {
            Err(ValidationError::NegativeValue("oxygen".to_string()))
        } else {
            Ok(Oxygen(value))
        }
    }

    pub fn percentage(&self) -> f32 {
        self.0
    }

    pub fn is_dangerously_low(&self) -> bool {
        self.0 < 5.0
    }
}

// Helper macro for simple positive value types
macro_rules! impl_positive_value {
    ($type:ident, $name:literal) => {
        impl $type {
            pub fn new(value: f32) -> Result<Self, ValidationError> {
                if value < 0.0 {
                    Err(ValidationError::NegativeValue($name.to_string()))
                } else {
                    Ok($type(value))
                }
            }

            pub fn value(&self) -> f32 {
                self.0
            }
        }
    };
}

impl_positive_value!(CarbonDioxide, "carbon_dioxide");
impl_positive_value!(Nitrogen, "nitrogen");
impl_positive_value!(WaterVolume, "water_volume");
impl_positive_value!(Moisture, "moisture");
impl_positive_value!(Aeration, "aeration");
impl_positive_value!(Detritus, "detritus");