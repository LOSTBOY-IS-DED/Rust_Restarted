struct Shape {
    width: f32,
    height: f32,
}

impl Shape {
    fn area(&self) -> u32 {
        return self.width * self.height;
    }

    fn perimeter(&self) -> u32 {
        return 2 * (self.width + self.height);
    }

    fn print_area(&self) {
        println!("Area is {}", self.area());
    }

    fn print_some() {
        println!("I am a static function");
    }
}

fn main() {
    let s = Shape {
        width: 30.0,
        height: 40.0,
    };

    println!("Perimeter is {}", s.perimeter());
    Shape::print_some();
}
