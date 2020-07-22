use actix_web::{Responder, HttpResponse, get, post, put, delete, web};
use crate::controller::model::bike::Bike;
use super::super::service::model::bike::Bike as BikeDomain;
use crate::bike_app::BikeApp;

#[get("/bike")]
pub async fn get_bikes(data: web::Data<BikeApp>) -> impl Responder {
    let service = data.service.clone();
    let bikes_found = service.find_all();
    let mut bikes = vec![];
    for bike in bikes_found {
        let bike_found = Bike {
            id: bike.id,
            model: bike.model,
            price: bike.price
        };
        bikes.push(bike_found);
    };
    HttpResponse::Ok().json(bikes)
}

#[get("/bike/{id}")]
pub async fn get_bike(info: web::Path<Bike>, data: web::Data<BikeApp>) -> impl Responder {
    let service = data.service.clone();
    let bike_found = service.find_by_id(info.id);
    HttpResponse::Ok().body(format!("A bike found with id {}, model {}, and price {}", bike_found.id,
                                    bike_found.model.unwrap(), bike_found.price.unwrap()))
}

#[post("/bike")]
pub async fn post_bike(info: web::Json<Bike>, data: web::Data<BikeApp>) -> impl Responder {
    let bike_request = info.0;
    let bike = BikeDomain{
        id: bike_request.id,
        model: bike_request.model,
        price: bike_request.price
    };
    let service = data.service.clone();
    service.save(bike);
    HttpResponse::Ok().body(format!("Bike created"))
}

#[put("/bike")]
pub async fn put_bike(info: web::Json<Bike>, data: web::Data<BikeApp>) -> impl Responder {
    let bike_request = info.0;
    let bike = BikeDomain{
        id: bike_request.id,
        model: bike_request.model,
        price: bike_request.price
    };
    let service = data.service.clone();
    service.update(bike);
    HttpResponse::Ok().body(format!("Bike updated"))
}

#[delete("/bike")]
pub async fn delete_bike(info: web::Json<Bike>, data: web::Data<BikeApp>) -> impl Responder {
    let service = data.service.clone();
    service.delete(info.id);
    HttpResponse::Ok().body(format!("Bike deleted"))
}
