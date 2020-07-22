use r2d2::Pool;
use diesel::r2d2::ConnectionManager;
use diesel::{MysqlConnection, RunQueryDsl, QueryDsl, ExpressionMethods};
use crate::repository::bike_repository::BikeRepository;
use crate::repository::model::bike::Bike;
use crate::repository::schema::bikes;

#[derive(Clone)]
pub struct MySqlExecutor {
    pub connector: Pool<ConnectionManager<MysqlConnection>>
}

impl BikeRepository for MySqlExecutor {
    fn find_all(&self) -> Vec<Bike> {
        let conn = &self.connector.get()
            .expect("Error with url connection");

        bikes::table.load::<Bike>(conn).expect("Error")
    }

    fn find_by_id(&self, id: String) -> Bike {
        let conn = &self.connector.get()
            .expect("Error with url connection");

        bikes::table.filter(bikes::id.eq(id))
            .first::<Bike>(conn)
            .expect("Error loading bike")
    }

    fn save(&self, bike: Bike) {
        let conn = &self.connector.get()
            .expect("Error with url connection");
        diesel::insert_into(bikes::table)
            .values(bike)
            .execute(conn)
            .expect("Error creating bike");
    }

    fn update(&self, id: String, bike: Bike) {
        let conn = &self.connector.get()
            .expect("Error with url connection");

        diesel::update(bikes::table)
            .filter(bikes::id.eq(id))
            .set(bike)
            .execute(conn)
            .expect("Error loading bike");
    }

    fn delete(&self, id: String) {
        let conn = &self.connector.get()
            .expect("Error with url connection");

        diesel::delete(bikes::table)
            .filter(bikes::id.eq(id))
            .execute(conn)
            .expect("Error deleting bike");
    }
}