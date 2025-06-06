// use chrono::prelude::*;   //means importing everything
// use chrono::{Local, Utc};
mod display;

// fn main() {
//     let utc = Utc::now();
//     let local_time = Local::now();
//     print!("The time is :  {}\n", utc);
//     print!("The local time is : {}", local_time)
// }

// understanding generics

// fn main() {
//     let ans = sum(2, 4);
//     // let ans2 = sum(2, 4.0);  // this piece of line would give me error
//     let name = String::from("Subh the great");
//     display::display(name);
//     println!("The ans value is : {}", ans)
// }

// // without trait bound

// // fn sum<T>(a: T, b: T) -> T {
// //     return a + b;
// // }

// // with trait bound
// fn sum<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
//     return a + b;
// }

// what are traits ? similar as interfaces in ts we can implement them

// trait Shape {
//     fn area(&self) -> u32;
// }

// struct Rect {
//     width: u32,
//     height: u32,
// }

// impl Shape for Rect {
//     fn area(&self) -> u32 {
//         return self.width * self.height;
//     }
// }

// struct Circle {
//     radius: u32,
// }

// impl Shape for Circle {
//     fn area(&self) -> u32 {
//         return self.radius * self.radius;
//     }
// }

// fn main() {
//     let r = Rect {
//         width: 10,
//         height: 10,
//     };
//     let circle = Circle { radius: 20 };
//     let circle_ans = get_area(circle);
//     let ans = get_area(r);
//     println!("{}", circle_ans);
//     println!("{}", ans)
// }

// fn get_area(s: impl Shape) -> u32 {
//     return s.area();
// }

// can you implement this for circle in the next commit  ?
// step 1 : create struct
// step 2 : implement struct
// step 3 : create variable
// step 4 : call the function

// macros-> declarative macro , procedural micro

// there is a struct i haven't implemented it yet

// #[derive(Debug)]
// struct User {
//     username: String,
//     password: String,
//     age: u32,
// }

// impl Debug for User {
// derive line just added this piece of code
// impl ::core::fmt::Debug for User {
//     #[inline]
//     fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
//         ::core::fmt::Formatter::debug_struct_field3_finish(
//             f,
//             "User",
//             "username",
//             &self.username,
//             "password",
//             &self.password,
//             "age",
//             &&self.age,
//         )
//     }
// }
// }

// fn main() {
//     println!("Hello world !!!");

//     let user = User {
//         username: String::from("Subh"),
//         password: String::from("password"),
//         age: 32,
//     };

//     println!("{:?}", user); // Debug print
//                             // println!("{}", user); // Pretty Display print
//     println!("Users username is {}", user.username)
// }

// clone and copy
// variable of a stack can be easily copied but not that of a heap
// heap are slow

// fn main() {
//     let num = 1;
//     let num2 = 2;

//     print_it(num2); // ownership didn't transferred here
//     println!("{}", num);
//     println!("{}", num2);
// }

// fn print_it(a: u32) {
//     println!("{}", a);
// }

// fn main() {
//     let name = String::from("Ishika");
//     print_name(name.clone());
//     println!("{}", name);
// }

// fn print_name(name: String) {
//     println!("{}", name);
// }

// now the question is if i define my own struct ?

#[derive(Debug, Copy, Clone)]

struct User {
    is_name: bool,
    age: u32,
}

fn main() {
    let u1 = User {
        is_name: true,
        age: 22,
    };

    let u2 = u1;
    // or u1.clone()

    println!("{:?} {:?}", u1, u2)
}

// if i  add strings it doesn't implements copy
