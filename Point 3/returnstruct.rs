struct User {
    username: String,
}

fn create_user(name: &str) -> User {
    User {
        username: name.to_string(),
    }
}

fn main() {
    let user = create_user("Qinthar");
    println!("Username: {}", user.username);
}
