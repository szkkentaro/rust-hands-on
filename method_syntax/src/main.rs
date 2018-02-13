struct Circle {
    x: f64,
    y: f64,
    radius: f64
}
impl Circle {
    fn new(x: f64, y: f64, radius: f64) -> Circle {
        Circle { x: x, y: y, radius: radius }
    }
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
    fn grow(&self, increment: f64) -> Circle {
        Circle { x: self.x, y: self.y, radius: self.radius + increment }
    }
    fn reference(&self) {
        println!("taking self by reference!");
    }
    fn mutable_reference(&mut self) {
        self.x = 1.0;
        println!("taking self by mutable reference!");
    }
}

impl Circle {
    fn takes_ownership(self) {
        println!("taking ownership of self!");
    }
}

struct CircleBuilder {
    x: f64,
    y: f64,
    radius: f64
}

impl CircleBuilder {
    fn new() -> CircleBuilder {
        CircleBuilder{ x: 0.0, y: 0.0, radius: 1.0 }
    }
    fn x(&mut self, coordinate: f64) -> &mut CircleBuilder {
        self.x = coordinate;
        self
    }
    fn y(&mut self, coordinate: f64) -> &mut CircleBuilder {
        self.y = coordinate;
        self
    }
    fn radius(&mut self, coordinate: f64) -> &mut CircleBuilder {
        self.radius = coordinate;
        self
    }
    fn finalize(&self) -> Circle {
        Circle {x: self.x, y: self.y, radius: self.radius}
    }
}

fn main() {
    let c = Circle {
        x: 0.0,
        y: 0.0,
        radius: 2.0,
    };
    println!("{},{},{}", c.area(), c.x, c.y);
    c.reference();
    println!("{}", c.grow(2.0).area());
    c.takes_ownership();
   
    let mut c1 = Circle::new(0.0, 0.0, 2.0);
    c1.mutable_reference();
    println!("{}", c1.x);

    let c2 = CircleBuilder::new()
            .x(1.0)
            .y(2.0)
            .radius(2.0)
            .finalize();
    println!("{}", c2.area());
}
