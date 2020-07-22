use crate::repository::schema::bikes;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
#[table_name="bikes"]
pub struct Bike {
    pub id: String,
    pub model: String,
    pub price: String,
}