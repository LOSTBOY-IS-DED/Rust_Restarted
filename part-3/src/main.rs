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

fn main() {
    let ans = sum(2, 4);
    // let ans2 = sum(2, 4.0);  // this piece of line would give me error
    let name = String::from("Subh the great");
    display::display(name);
    println!("The ans value is : {}", ans)
}

// without trait bound

// fn sum<T>(a: T, b: T) -> T {
//     return a + b;
// }

// with trait bound
fn sum<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    return a + b;
}
