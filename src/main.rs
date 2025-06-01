mod string;
fn main() {
    println!("Hi there ishika mam");
    let ans: u32 = sum(10, 20);
    println!("Ans is {}", ans);
    print!("Is 10 even? {}\n", is_even(10));
    string::say_hello("ishika");
    string::name();
}

fn sum(a: u32, b: u32) -> u32 {
    return a + b;
}

fn is_even(num: u32) -> bool {
    return num % 2 == 0;
}

// signed -> both postive and negative
// // unsigned -> only positive
// u32
// usize
// u64
// u128
// u8
