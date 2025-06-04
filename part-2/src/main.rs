// mod using_as_obj;
// mod file_hadling;
mod option_enum;

// fn main() {
//     let str: String = String::from("Hello Ishika");

//     println!("{}\n", get_length(str));
//     println!("{}\n", str);
// }
// fn get_length(str: String) -> usize {
//     return str.len();
// }

// Fix transferring back the ownership

// fn main() {
//     let str: String = String::from("Hello Ishika");
//     let (len, str) = get_length(str);
//     println!("Length is {} \n", len);
//     println!("{}\n", str);
// }
// fn get_length(str: String) -> (usize, String) {
//     return (str.len(), str);
// }

// Fix the ownership of the string --> borrowing the variable

// fn main() {
//     let str = String::from("Hello Ishika");
//     let len = get_length(&str);
//     println!("Length is {} \n", len);
//     println!("{}\n", str);
// }

// fn get_length(str: &String) -> usize {
//     return str.len();
// }

// will this compile ? --> yes

// fn main() {
//     let mut str = String::from("hello ishika");

//     let str2: &mut String = &str; // mutable reference
//     str2.push_str(" subhajit");

//     // -------------------------------- lifetime ends here isnt used below

//     let str3: &String = &str;
//     let str4: &String = &str;

//     println! {"{} {}" , str3, str4}
// }

// structs  ----------------------------------------------------------------------------------------------------------

// fn main() {
//     let user = using_as_obj::User {
//         name: String::from("Ishika"),
//         is_active: true,
//         email: String::from("ishika@gmail.com"),
//         password: String::from("123456"),
//     };

//     println!("{}", user.name);
//     println!("{:?}", user);
// }

// enums and pattern matching  ----------------------------------------------------------------------------------------------------------

// this wont work and is wrong

// enum Direction {
//     North,
//     South,
//     East,
//     West,
// }

// fn main() {
//     let direction = Direction::North;
//     stear(direction);
// }

// fn stear(dir: Direction) {
//     if dir == Direction::North {
//         println!("Going North");
//     } else if dir == Direction::South {
//         println!("Going South");
//     } else {
//         println!("Going horizontal");
//     }
// }

// correct way now pattern matching
// enum Direction {
//     North,
//     South,
//     East,
//     West,
// }

// fn main() {
//     let direction = Direction::North;
//     stear(direction);
//     file_hadling::file();
// }

// fn stear(dir: Direction) {
//     match dir {
//         Direction::East => println!("Going East"),
//         Direction::West => println!("Going West"),
//         default => println!("Going horizontal"),
//     }
// }

// finally studied option enum
fn main() {
    option_enum::option_enum_fn();
}
