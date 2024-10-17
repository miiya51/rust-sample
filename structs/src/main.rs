struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
    println!("{}", user1.email);
    println!("{}", user1.username);
    println!("{}", user1.active);
    println!("{}", user1.sign_in_count);

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };
    println!("{}", user2.email);
    println!("{}", user2.username);
    println!("{}", user2.active);
    println!("{}", user2.sign_in_count);

    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );

}


fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

