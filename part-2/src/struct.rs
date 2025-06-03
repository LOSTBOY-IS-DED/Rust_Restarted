struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        return self.width * self.height;
    }

    fn print_something() {
        // we are not passing any argument from the struct hence we dont need to pass self
        println!("I am a stactic function");
    }
}

pub fn main() {
    let r = Rect {
        width: 30,
        height: 40,
    };

    println!("width : {} , height : {}", r.width, r.height);
    print!("Area is {}", r.area());
    Rect::print_something(); // this doest not exist on r , it exist directly on the class like struct Rect here
}
