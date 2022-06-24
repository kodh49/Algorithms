fn main() {
    let mut user_a = User {
        email: String::from("dzk5572@gmail.com"),
        name: String::from("Dohyoung Ko"),
        sign_in_attempts: 1,
        active: true,
    };
    user_a.sign_in_attempts += 1;
    println!("User Information | name:{} | email:{} | active:{} | attempts:{}", user_a.name, user_a.email, user_a.active, user_a.sign_in_attempts);
}

struct User {
    name: String,
    email: String,
    sign_in_attempts: u64,
    active: bool,
}