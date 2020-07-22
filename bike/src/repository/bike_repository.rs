use crate::repository::model::bike::Bike;

pub trait BikeRepository {
    fn find_all(&self) -> Vec<Bike>;
    fn find_by_id(&self, id: String) -> Bike;
    fn save(&self, bike: Bike);
    fn update(&self, id: String, bike: Bike);
    fn delete(&self, id: String);
}
