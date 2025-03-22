pub struct User {
    pub is_authenticated: bool,
    pub username: Option<String>,
    pub path: Option<String>,
}

impl User {
    pub fn new() -> Self {
        Self {
            is_authenticated: false,
            username: None,
            path: None,
        }
    }
}
