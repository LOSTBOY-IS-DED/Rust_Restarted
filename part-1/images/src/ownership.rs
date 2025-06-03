// // wrong peice of code

// pub fn wrong_ownership() {
//     let str = String::from("Ishika");
//     let len = get_length(str);
//     println!("{}", len);

//     print!("{}", str);
// }

// pub fn get_length(str: String) -> usize {
//     return str.len();
// }

// right peice of code

// fix : 1 Transferring back the ownership

// ✅ Borrow the string instead of moving it
pub fn get_length(s: &str) -> usize {
    s.len()
}

// Optional: if you want to showcase the fixed version by reference
pub fn fixed_ownership() {
    let str = String::from("mahi verma");
    let len = get_length(&str);
}
