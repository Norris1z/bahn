use crate::database::tables::User;

pub struct Auth {}

impl Auth {
    pub fn attempt(name: &str, password: &str) -> Option<String> {
        let user = User::get_by_name(name)?;

        let verify = bcrypt::verify(password, user.password.as_str());

        if verify.is_err() || !verify.unwrap() {
            return None;
        }

        Some(user.path)
    }
}
