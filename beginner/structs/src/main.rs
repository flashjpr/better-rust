struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
fn main() {

    let mut user1 = User {
        email: String::from("alex@removebounce.com"),
        username: String::from("alexf"),
        sign_in_count: 1,
        active: true
    };

    user1.email = String::from("flashjpr@gmail.com");

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);




}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        sign_in_count: 1,
        active: true
    }
}