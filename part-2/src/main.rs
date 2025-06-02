fn main() {
    let str: String = String::from("Hello Ishika");

    println!("{}\n", get_length(str));
    println!("{}\n", str);
}
fn get_length(str: String) -> usize {
    return str.len();
}
