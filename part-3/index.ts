interface Shape {
  area: () => number;
  perimeter: () => number;
}

class Rect implements Shape {
  constructor() {}
  area() {
    return 10;
  }
  perimeter() {
    return 10;
  }
}
// if you pass it down here

// it will work as long as it implements
// function getArea(s : Shape){

// }

// what we can do in rust
