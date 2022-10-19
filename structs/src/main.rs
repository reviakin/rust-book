struct User {
    active: boolean,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    println!("Hello, world!");

    // Immutable instance
    let user1 =  User {
        email: String::from("some@mail.com"),
        username: String::from("Emos"),
        active: true,
        sign_in_count: 1
    }

    // Mutable instance
    let mut user2 = User {
        email: String::from("some_new@mail.com"),
        username: String::from("Emos_wen"),
        active: true,
        sign_in_count: 9
    }

    user2.email = String::from("my_super_email@mail.com");
    
    let user3 = build_user(String::from("asdasd"), String::from("asdasd"))

    let user4 = User {
        email: String::from("no@mail.com"),
        // Struct update syntax
        ..user2
    }
}

fn build_user(email: String, username: String) -> User {
    User {
        // Shorthand if the prop's name is the same
        email,
        username,
        active: true,
        sign_in_count: 1
    }
}

