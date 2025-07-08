// v2/traits.rs
// Trait definitions for ecosystem components

use crate::v2::errors::{EcosystemResult, CollapseReason};
use crate::v2::state::EcosystemStateV2;
use std::fmt;

/// Trait for components that can display their status
pub trait EcosystemDisplay {
    fn display_status(&self) -> String;
    fn display_summary(&self) -> String;
    fn display_detailed(&self) -> String;
}

/// Trait for components that can be validated
pub trait EcosystemValidation {
    fn validate(&self) -> EcosystemResult<()>;
    fn is_healthy(&self) -> bool;
    fn health_warnings(&self) -> Vec<String>;
}

/// Trait for components that can collapse
pub trait CollapseDetection {
    fn is_collapsed(&self) -> bool;
    fn collapse_risk(&self) -> f32; // 0.0 = safe, 1.0 = imminent collapse
    fn collapse_reasons(&self) -> Vec<CollapseReason>;
}

/// Trait for components that can be monitored over time
pub trait EcosystemMonitoring {
    fn key_metrics(&self) -> Vec<(String, f32)>;
    fn trend_indicators(&self) -> Vec<TrendIndicator>;
    fn alert_conditions(&self) -> Vec<AlertCondition>;
}

/// Trait for components that can be configured
pub trait Configurable {
    type Config;
    fn configure(&mut self, config: &Self::Config) -> EcosystemResult<()>;
    fn get_config(&self) -> &Self::Config;
}

/// Trait for components that can simulate time steps
pub trait Simulatable {
    fn step(&mut self, dt: f32) -> EcosystemResult<()>;
    fn can_step(&self) -> bool;
    fn reset(&mut self) -> EcosystemResult<()>;
}

/// Trait for components that interact with other ecosystem components
pub trait EcosystemInteraction {
    fn affects_environment(&self) -> Vec<EnvironmentalEffect>;
    fn requires_resources(&self) -> Vec<ResourceRequirement>;
    fn provides_services(&self) -> Vec<EcosystemService>;
}

#[derive(Debug, Clone)]
pub struct TrendIndicator {
    pub metric: String,
    pub direction: TrendDirection,
    pub strength: f32, // 0.0 = no trend, 1.0 = strong trend
    pub confidence: f32, // 0.0 = low confidence, 1.0 = high confidence
}

#[derive(Debug, Clone, PartialEq)]
pub enum TrendDirection {
    Increasing,
    Decreasing,
    Stable,
    Oscillating,
}

#[derive(Debug, Clone)]
pub struct AlertCondition {
    pub severity: AlertSeverity,
    pub message: String,
    pub parameter: String,
    pub current_value: f32,
    pub threshold: f32,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum AlertSeverity {
    Info,
    Warning,
    Critical,
    Emergency,
}

#[derive(Debug, Clone)]
pub struct EnvironmentalEffect {
    pub parameter: String,
    pub effect_type: EffectType,
    pub magnitude: f32,
    pub duration: Option<f32>, // None = permanent
}

#[derive(Debug, Clone, PartialEq)]
pub enum EffectType {
    Increase,
    Decrease,
    Stabilize,
    Oscillate,
}

#[derive(Debug, Clone)]
pub struct ResourceRequirement {
    pub resource: String,
    pub amount: f32,
    pub criticality: ResourceCriticality,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ResourceCriticality {
    Optional,
    Helpful,
    Important,
    Critical,
    Essential,
}

#[derive(Debug, Clone)]
pub struct EcosystemService {
    pub service: String,
    pub benefit_type: ServiceType,
    pub magnitude: f32,
    pub reliability: f32, // 0.0 = unreliable, 1.0 = always available
}

#[derive(Debug, Clone, PartialEq)]
pub enum ServiceType {
    NutrientCycling,
    WaterPurification,
    SoilAeration,
    OxygenProduction,
    CarbonSequestration,
    WasteDecomposition,
    PhBuffering,
    BiodiversitySupport,
}

// Implementations for fmt::Display
impl fmt::Display for TrendDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TrendDirection::Increasing => write!(f, "↗"),
            TrendDirection::Decreasing => write!(f, "↘"),
            TrendDirection::Stable => write!(f, "→"),
            TrendDirection::Oscillating => write!(f, "↕"),
        }
    }
}

impl fmt::Display for AlertSeverity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AlertSeverity::Info => write!(f, "ℹ"),
            AlertSeverity::Warning => write!(f, "⚠"),
            AlertSeverity::Critical => write!(f, "🔴"),
            AlertSeverity::Emergency => write!(f, "🚨"),
        }
    }
}

impl fmt::Display for EffectType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EffectType::Increase => write!(f, "+"),
            EffectType::Decrease => write!(f, "-"),
            EffectType::Stabilize => write!(f, "="),
            EffectType::Oscillate => write!(f, "~"),
        }
    }
}

impl fmt::Display for ResourceCriticality {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ResourceCriticality::Optional => write!(f, "Optional"),
            ResourceCriticality::Helpful => write!(f, "Helpful"),
            ResourceCriticality::Important => write!(f, "Important"),
            ResourceCriticality::Critical => write!(f, "Critical"),
            ResourceCriticality::Essential => write!(f, "Essential"),
        }
    }
}

impl fmt::Display for ServiceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServiceType::NutrientCycling => write!(f, "Nutrient Cycling"),
            ServiceType::WaterPurification => write!(f, "Water Purification"),
            ServiceType::SoilAeration => write!(f, "Soil Aeration"),
            ServiceType::OxygenProduction => write!(f, "Oxygen Production"),
            ServiceType::CarbonSequestration => write!(f, "Carbon Sequestration"),
            ServiceType::WasteDecomposition => write!(f, "Waste Decomposition"),
            ServiceType::PhBuffering => write!(f, "pH Buffering"),
            ServiceType::BiodiversitySupport => write!(f, "Biodiversity Support"),
        }
    }
}

// Helper trait for converting numeric values to trend indicators
pub trait TrendAnalysis {
    fn analyze_trend(&self, historical_values: &[f32]) -> TrendIndicator;
}

impl TrendAnalysis for f32 {
    fn analyze_trend(&self, historical_values: &[f32]) -> TrendIndicator {
        if historical_values.len() < 2 {
            return TrendIndicator {
                metric: "unknown".to_string(),
                direction: TrendDirection::Stable,
                strength: 0.0,
                confidence: 0.0,
            };
        }

        let recent_values = &historical_values[historical_values.len().saturating_sub(5)..];
        let first = recent_values[0];
        let last = recent_values[recent_values.len() - 1];
        let change = (last - first) / first.abs().max(0.001);

        let direction = if change > 0.05 {
            TrendDirection::Increasing
        } else if change < -0.05 {
            TrendDirection::Decreasing
        } else {
            TrendDirection::Stable
        };

        let strength = change.abs().min(1.0);
        let confidence = (recent_values.len() as f32 / 10.0).min(1.0);

        TrendIndicator {
            metric: "value".to_string(),
            direction,
            strength,
            confidence,
        }
    }
}