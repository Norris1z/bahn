use crate::database::Database;

pub struct User {}

impl User {
    pub fn create_table() {
        let connection = Database::get_connection();
        connection
            .execute("CREATE TABLE IF NOT EXISTS users (name TEXT, password TEXT, path TEXT, UNIQUE(name));")
            .unwrap();
    }

    /**
     * Bcrypt password with value: password
     **/
    pub fn seed_table() {
        let password = "$2a$12$CBdYvXHDeSHe6.htLJt2wuW/teWdhZbtf7HDQLId9pbTBfU3/6Ksq";

        User::insert("admin", password, "/");
        User::insert("user", password, "/user");
    }

    pub fn insert(name: &str, password: &str, path: &str) {
        let connection = Database::get_connection();
        connection
            .execute(format!(
                "INSERT OR IGNORE INTO users (name, password, path) VALUES ('{}', '{}','{}');",
                name, password, path
            ))
            .unwrap();
    }
}
