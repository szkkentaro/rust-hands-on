struct Circle {
    x: f64,
    y: f64,
    radius: f64
}
impl Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
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

fn main() {
    let c = Circle {
        x: 0.0,
        y: 0.0,
        radius: 2.0,
    };
    println!("{},{},{}", c.area(), c.x, c.y);
    c.reference();
    c.takes_ownership();
   
    let mut c1 = Circle {
        x: 0.0,
        y: 0.0,
        radius: 2.0,
    };
    c1.mutable_reference();
    println!("{}", c1.x);
}
