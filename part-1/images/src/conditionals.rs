pub fn conditionals() {
    let x = 99;
    let is_even = is_even(x);
    if is_even {
        println!("x is even");
    } else {
        println!("x is odd");
    }
}

pub fn is_even(x: i32) -> bool {
    return x % 2 == 0;
}
