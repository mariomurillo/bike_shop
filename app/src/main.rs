mod datasource;

use actix_web::{HttpServer, App, web};
use bike::controller::bike_controller::{get_bikes, get_bike, post_bike, put_bike, delete_bike};
use bike::bike_app::{BikeApp, get_state};
use bike::repository::executor::mysql_executor::MySqlExecutor;
use bike::service::bike_service::BikeService;
use std::rc::Rc;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let database_pool = datasource::mysql_connector::get_connection();
    HttpServer::new(move || {
        let executor = MySqlExecutor {
            connector: database_pool.clone()
        };
        let service = BikeService{
            repository: Rc::new(executor.clone())
        };
        let state = BikeApp {
            app_name: String::from("Bike Shop"),
            service: service.clone()
        };
        App::new()
            .data(state)
            .route("/", web::get().to(get_state))
            .service(web::scope("/bike-shop")
                .service(get_bikes)
                .service(get_bike)
                .service(post_bike)
                .service(put_bike)
                .service(delete_bike)
            )
    }).bind("127.0.0.1:8088")?
        .run()
        .await
}
