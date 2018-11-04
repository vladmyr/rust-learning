struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

// tuple structs
struct Color(i32, i32, i32, i32);
struct Point(i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn build_user(email: &str, username: &str) -> User {
    User {
        email: String::from(email),
        username: String::from(username),
        active: true,
        sign_in_count: 1
    }
}

fn log_user(user: &User) {
    println!("User:\n\temail: {}\n\tusername: {}\n\tactive: {}\n\tsign_in_count: {}",
        user.email,
        user.username,
        user.active,
        user.sign_in_count
    )
}

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1
    };

    log_user(&user1);

    // access a value of a struct and update it
    user1.email = String::from("anotheremail@example.com");

    log_user(&user1);

    let user2 = build_user("anotherone@example.com", "anothername123");

    log_user(&user2);

    let user3 = User {
        email: String::from("thirduser@example.com"),
        username: String::from("anotherusername567"),
        ..user1     //example of "struct update syntax"
    };

    log_user(&user3);

    let black = Color(0, 0, 0, 1);
    let origin = Point(0, 0, 0);

    println!("black color is a tuple of value ({}, {}, {}, {})", black.0, black.1, black.2, black.3);
    println!("origin point is a tuple of value ({:?}, {}, {})", origin.0, origin.1, origin.2);

    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!("rect1 is {:?}", rect1);
    println!("rect1 is {:#?}", rect1);

    println!("The area of the rectangle is {} square pixels.", rect1.area());

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("Can rect2 hold rect3? {}", rect2.can_hold(&rect3));

    let sq = Rectangle::square(3);

    println!("sq is {:?}", sq);
}
