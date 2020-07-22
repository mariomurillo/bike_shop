use crate::service::model::bike::Bike;
use crate::repository::{bike_repository::BikeRepository, model::bike::Bike as BikeModel};
use std::rc::Rc;

#[derive(Clone)]
pub struct BikeService {
    pub repository: Rc<dyn BikeRepository>,
}

impl BikeService {
    pub fn find_all(self) -> Vec<Bike> {
        let bikes_found = self.repository.find_all();
        let mut bikes = vec![];
        for bike in bikes_found {
            let bike_found = Bike {
                id: bike.id.parse().unwrap(),
                model: Option::from(bike.model),
                price: Option::from(bike.price.parse::<f32>().unwrap())
            };
            bikes.push(bike_found);
        };
        bikes
    }

    pub fn find_by_id(self, id: i32) -> Bike {
        let bike_found = self.repository.find_by_id(id.to_string());
        Bike {
            id: bike_found.id.parse().unwrap(),
            model: Option::from(bike_found.model),
            price: Option::from(bike_found.price.parse::<f32>().unwrap())
        }
    }

    pub fn save(self, bike: Bike)  {
        let bike_repository = BikeModel {
            id: bike.id.to_string(),
            model: bike.model.unwrap(),
            price: bike.price.unwrap().to_string(),
        };
        self.repository.save(bike_repository);
    }

    pub fn update(self, bike: Bike) {
        let bike_repository = BikeModel {
            id: bike.id.to_string(),
            model: bike.model.unwrap(),
            price: bike.price.unwrap().to_string(),
        };
        self.repository.update(bike.id.to_string(), bike_repository);
    }

    pub fn delete(self, id: i32) {
        self.repository.delete(id.to_string());
    }
}


