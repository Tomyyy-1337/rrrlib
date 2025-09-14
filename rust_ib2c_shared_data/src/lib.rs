use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SharedData {
    pub index: u64,
    pub activity: f32,
    pub target_rating: f32,
    pub stimulation: f32,
    pub inhibition: f32,
    pub source: String,
    // pub data: String
}

