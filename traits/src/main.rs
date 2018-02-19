#![allow(dead_code)]
trait HasArea {
    fn area(&self) -> f64;
}

struct Circle {
    x: f64,
    y: f64,
    radius: f64
}

impl HasArea for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

struct Square {
    x: f64,
    y: f64,
    side: f64
}

impl HasArea for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

fn print_area<T: HasArea>(shape: T) {
    println!("The shape of area is {}", shape.area());
}

fn main() {
    let c = Circle {x: 1.0, y: 1.0, radius: 1.0};
    println!("c.area() is {}", c.area());

    let circle = Circle {
        x: 0.0,
        y: 0.0, 
        radius: 1.0
    };
    let square = Square {
        x: 0.0,
        y: 0.0, 
        side: 1.0
    };
    print_area(circle);
    print_area(square);
    
}
