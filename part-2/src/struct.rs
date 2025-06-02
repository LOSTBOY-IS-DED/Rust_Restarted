struct Rect {
    width: u32,
    height: u32,
}

pub fn main() {
    let r = Rect {
        width: 30,
        height: 40,
    };

    println!("width : {} , height : {}", r.width, r.height);
}
