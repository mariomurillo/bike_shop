use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Bike {
    pub id: i32,
    pub model: Option<String>,
    pub price: Option<f32>,
}
