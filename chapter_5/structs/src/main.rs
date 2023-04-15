
// Structs
struct User { 
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// Tuple Structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    // struct used
    let mut user1 = User {
        email: String::from("dev@seoseonyu.com"),
        username: String::from("seoseonyu"),
        active: true,
        sign_in_count: 1,
    }; 

    let mut user2 = User {
        email: String::from("dev@seonyu.dev"),
        username: String::from("seonyu"),
        ..user1
    };

    // Tuple struct used
    let black = Color(0, 0, 0);
    let origin=Point(0, 0, 0);
}

fn build_user(email:String, username: String) -> User {
    User{
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}