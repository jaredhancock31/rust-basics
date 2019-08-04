
#[derive(Debug)]
pub struct User {
    pub username: String,
    pub email: String,
}

impl User {
    pub fn new(username: &str, email: &str) -> User {
        User {
            username: String::from(username),
            email: String::from(email)
        }
    }
}
pub fn build_user(username: String, email: String) -> User {
    User {
        username,
        email
    }
}


