use crate::database::tables::User;

pub mod tables;
pub struct Database {}

//TODO: maybe a connection pool for the database?
impl Database {
    fn get_connection() -> sqlite::Connection {
        sqlite::open(dotenv::var("SQLITE_DATABASE_FILE").expect("SQLITE_DATABASE_FILE not set"))
            .unwrap()
    }

    pub fn run_migrations() {
        User::create_table();
    }

    pub fn seed() {
        User::seed_table();
    }
}
