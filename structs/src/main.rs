fn main() {
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    let mut user1 = User {
        email: String::from("example@example.com"),
        username: String::from("example"),
        active: true,
        sign_in_count: 1,
    }

    //only works if user1 is mutable
    user1.email = String::from("anotheremail@example.com");

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        // inherits active and sign_in_count from user1
        ..user1
    }

    fn build_user(email: String, username: String) -> User {
        User {
            // field init shorthand
            // email: email,
            // username: username,
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
    }

    // tuple structs
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
