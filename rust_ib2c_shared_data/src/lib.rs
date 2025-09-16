use std::fmt::Display;

use serde::{Deserialize, Serialize};
use data_types::prelude::*;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SharedData {
    pub index: u64,
    pub activity: f32,
    pub target_rating: f32,
    pub stimulation: f32,
    pub inhibition: f32,
    pub source: String,
    pub data: Vec<(String, PortData)>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum PortData {
    Float(f64), 
    Int(i64),
    Unsigned(u64),
    Bool(bool),
    String(String),
    MetaSignal(f32),
    SiValue {
        value: f64,
        unit: String,   
    },
}

impl Display for PortData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PortData::Float(v) => write!(f, "{:.4}", v),
            PortData::Int(v) => write!(f, "{}", v),
            PortData::Unsigned(v) => write!(f, "{}", v),
            PortData::Bool(v) => write!(f, "{}", v),
            PortData::String(v) => write!(f, "{}", v),
            PortData::MetaSignal(v) => write!(f, "{:?}", v),
            PortData::SiValue { value, unit } => write!(f, "{:.4} {}", value, unit),
        }
    }
}