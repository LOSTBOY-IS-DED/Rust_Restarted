// fn main() {
//     let str: String = String::from("Hello Ishika");

//     println!("{}\n", get_length(str));
//     println!("{}\n", str);
// }
// fn get_length(str: String) -> usize {
//     return str.len();
// }

// Fix transferring back the ownership

fn main() {
    let str: String = String::from("Hello Ishika");
    let (len, str) = get_length(str);
    println!("Length is {} \n", len);
    println!("{}\n", str);
}
fn get_length(str: String) -> (usize, String) {
    return (str.len(), str);
}
