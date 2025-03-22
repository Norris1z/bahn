use crate::database::Database;

pub struct User {
    pub password: String,
    pub path: String,
}

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
                "INSERT OR IGNORE INTO users (name, password, path) VALUES ('{name}', '{password}','{path}');"
            ))
            .unwrap();
    }

    pub fn get_by_name(name: &str) -> Option<User> {
        let connection = Database::get_connection();
        let query = "SELECT * FROM users WHERE name = ?";

        let user = connection
            .prepare(query)
            .unwrap()
            .into_iter()
            .bind((1, name))
            .unwrap()
            .last()?
            .unwrap();

        Some(User {
            password: user.read::<&str, _>("password").to_string(),
            path: user.read::<&str, _>("path").to_string(),
        })
    }
}
