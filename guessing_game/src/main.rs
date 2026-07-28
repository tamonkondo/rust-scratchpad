#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let user1 = create_user(String::from("test"), String::from("test@emai.com"));

    let user2 = create_user(user1.username, String::from("user2@email.com"));
    println!("{:#?}", user2);
    let black = Color(1, 1, 2);
    let origin = Point(1, 1, 2);

    let rect1 = Rectangle {
        width: 30,
        height: 30,
    };

    dbg!(&rect1);
}

fn create_user(username: String, email: String) -> User {
    User {
        active: false,
        username,
        email,
        sign_in_count: 1,
    }
}
