fn main() {
    
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    fn build_user(email: String, username: String) -> User {
        User {
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
    }

    let user1 = build_user("usernumber1@email.com".to_string(), "u1".to_string());

    println!("user 1: username: {}, email: {}, sign_in_count: {}, active: {}", user1.username, user1.email, user1.sign_in_count, user1.active);

    let user2 = User {
        email: String::from("myotheraccount@email.com"),
        username: String::from("u2"),
        ..user1
};

    println!("user 2: username: {}, email: {}, sign_in_count: {}, active: {}", user2.username, user2.email, user2.sign_in_count, user2.active);

}
