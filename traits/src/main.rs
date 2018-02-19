struct Circle {
    x: f64,
    y: f64,
    radius: f64
}

trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}


fn main() {
    let c = Circle { x: 1.0, y: 1.0, radius: 1.0};
    println!("x: {}, y: {}", c.x, c.y);
    println!("radius: {}, area(): {}", c.radius, c.area());
}
