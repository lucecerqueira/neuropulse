use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UserRole {
    Admin,
    Client,
    Clinician,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SubscriptionTier {
    FreeGuest,
    FreeCommunity,
    ProAthlete,
    PerformanceLab,
    Clinical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DifficultyLevel {
    pub level: u8,
    pub name: &'static str,
    pub stimulus_interval_ms: u64,
    pub required_precision_pct: f64,
    pub max_saccadic_delay_ms: u64,
}

impl DifficultyLevel {
    pub fn get_config(level: u8) -> Result<Self, &'static str> {
        match level {
            1 => Ok(Self {
                level: 1,
                name: "Iniciação Visual",
                stimulus_interval_ms: 1000,
                required_precision_pct: 80.0,
                max_saccadic_delay_ms: 300,
            }),
            2 => Ok(Self {
                level: 2,
                name: "Saccadic Tracking",
                stimulus_interval_ms: 750,
                required_precision_pct: 85.0,
                max_saccadic_delay_ms: 250,
            }),
            3 => Ok(Self {
                level: 3,
                name: "Discriminação Periférica",
                stimulus_interval_ms: 500,
                required_precision_pct: 90.0,
                max_saccadic_delay_ms: 200,
            }),
            4 => Ok(Self {
                level: 4,
                name: "Supressão Inibitória (Go/No-Go)",
                stimulus_interval_ms: 350,
                required_precision_pct: 93.0,
                max_saccadic_delay_ms: 160,
            }),
            5 => Ok(Self {
                level: 5,
                name: "Sobrecarga Neuro-Visuomotora",
                stimulus_interval_ms: 200,
                required_precision_pct: 96.0,
                max_saccadic_delay_ms: 120,
            }),
            _ => Err("Nível inválido (escolha entre 1 e 5)"),
        }
    }
}
