use crate::service::bike_service::BikeService;
use actix_web::web;

pub struct BikeApp {
    pub app_name: String,
    pub service: BikeService,
}

pub async fn get_state(data: web::Data<BikeApp>) -> String {
    let app_name = &data.app_name;
    format!("{} Successful!", app_name)
}
