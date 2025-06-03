#[derive(Debug)]

pub struct User {
    pub name: String,
    pub is_active: bool,
    pub email: String,
    pub password: String,
}

pub fn print_user() {
    let user = User {
        name: String::from("Ishika"),
        is_active: true,
        email: String::from("ishika@gmail.com"),
        password: String::from("123456"),
    };

    println!("{}", user.name);
    println!("{:?}", user);
    // --> debug print the whole struct
}
