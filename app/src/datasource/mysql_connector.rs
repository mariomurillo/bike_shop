use r2d2::Pool;
use std::fs::File;
use std::io::Read;
use diesel::r2d2::ConnectionManager;
use diesel::MysqlConnection;
use yaml_rust::YamlLoader;

pub fn get_connection() -> Pool<ConnectionManager<MysqlConnection>> {
    let database_url = get_db_url();
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    r2d2::Pool::builder().build(manager).expect("Failed to create pool")
}

fn get_db_url() -> String {
    let mut f = File::open("app/conf/conf.yml").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    let docs = YamlLoader::load_from_str(&s).unwrap();
    let doc = &docs[0];
    format!("{}://{}:{}@{}:{}/{}",
            doc["database"]["dialect"].as_str().unwrap(),
            doc["database"]["user"].as_str().unwrap(),
            doc["database"]["password"].as_i64().unwrap(),
            doc["database"]["host"].as_str().unwrap(),
            doc["database"]["port"].as_i64().unwrap(),
            doc["database"]["schema"].as_str().unwrap())
}