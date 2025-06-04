// enum with values

enum Shapes {
    Circle(f32),
    Square(f32),
    Rectangle(f32, f32),
}

fn main() {
    let circle = Shapes::Circle(10.0);
    let square = Shapes::Square(10.0);
    let rectangle = Shapes::Rectangle(10.0, 10.0);

    let circle_area = calc_area(&circle);
    let square_area = calc_area(&square);
    let rectangle_area = calc_area(&rectangle);

    let circle_perimeter = calc_perimeter(&circle);
    let square_perimeter = calc_perimeter(&square);
    let rectangle_perimeter = calc_perimeter(&rectangle);

    println!("Circle area is {}", circle_area);
    println!("Square area is {}", square_area);
    println!("Rectangle area is {}", rectangle_area);

    println!("Circle perimeter is {}", circle_perimeter);
    println!("Square perimeter is {}", square_perimeter);
    println!("Rectangle perimeter is {}", rectangle_perimeter);
}

// write the function that takes the shape as an argument and prints the area
// write one that prints its perimeter as well

fn calc_area(s: &Shapes) -> f32 {
    match s {
        Shapes::Circle(radius) => 3.14 * radius * radius,
        Shapes::Square(side) => side * side,
        Shapes::Rectangle(width, height) => width * height,
    }
}

fn calc_perimeter(s: &Shapes) -> f32 {
    match s {
        Shapes::Circle(radius) => 2.0 * 3.14 * radius,
        Shapes::Square(side) => 4.0 * side,
        Shapes::Rectangle(width, height) => 2.0 * (width + height),
    }
}
