// strings can grow in size there is no specific size for them
// vectors and string are gonna give you trouble
pub fn say_hello(name: &str) {
    println!("Hello {}", name);
}

pub fn name() {
    let name: String = String::from("ishika");
    println!("Hello {}", name);
}
