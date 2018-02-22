#![allow(dead_code)]
use std::fmt::Debug;

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

// generics struct 
struct Rectangle<T> {
    x: T,
    y: T,
    width: T,
    height: T,
}
impl <T: PartialEq> Rectangle<T> {
    fn is_squea(&self) -> bool {
        self.width == self.height
    }
}


// multiple trait bound
fn foo<T: Clone + Debug>(x: T) {
    x.clone();
    println!("{:?}", x);
}

// where syntax
fn bar<T, K>(x: T, y: K)
    where T: Clone,
          K: Clone + Debug {
    x.clone();
    y.clone();
    println!("{:?}", y);
}

trait ConvertTo<T> {
    fn convert(&self) -> T;
}

impl ConvertTo<i64> for i32 {
    fn convert(&self) -> i64 {
        *self as i64
    }
}

// can call when T == i32
fn normal<T>(x: &T) -> i64
    where T: ConvertTo<i64> {
    x.convert()
}

// can call when T == i64
fn inverse<T>() -> T
    // use as if this is ConvertTo<i64>
    where i32: ConvertTo<T> {
    42.convert()
}

// use default
trait Default {
    fn is_valid(&self) -> bool;
    fn is_invalid(&self) -> bool { !self.is_valid() }
}

struct UseDefault;
impl Default for UseDefault {
    fn is_valid(&self) -> bool {
        println!("Called UseDefalt is valid");
        true
    }
    // there is no need to impl is_invalid()
}

struct OverrideDefault;
impl Default for OverrideDefault {
    fn is_valid(&self) -> bool {
        println!("Called OversrideDefault.is_valid");
        true
    }
    fn is_invalid(&self) -> bool {
        println!("Called OversrideDefault.is_invalid");
        true
    }
}

// inherite
trait Foo {
    fn foo(&self);
}

trait FooBar : Foo {
    fn foobar(&self);
}

struct Baz;
impl Foo for Baz {
    fn foo(&self) { println!("foo") }
}
impl FooBar for Baz {
    fn foobar(&self) { println!("foobar") }
}

#[derive(Debug)]
struct DeriveSample;

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

    // generics struct
    let mut r = Rectangle{
        x: 0,
        y: 0,
        width: 47,
        height: 47,
    };
    assert!(r.is_squea());

    // change width
    r.width = 42;
    assert!(!r.is_squea());
    
    foo("bar");
    bar("hello", "world");

    // where
    println!("{}", normal(&10));
    println!("{}", inverse());

    let default = UseDefault;
    assert!(!default.is_invalid());

    let over = OverrideDefault;
    assert!(over.is_invalid());

    // inherite
    let baz = Baz;
    baz.foo();
    baz.foobar();

    // derive
    println!("{:?}", DeriveSample);
    /* 
    derive is limitted to use methods below
        Clone
        Copy
        Debug
        Default
        Eq
        Hash
        Ord
        PartialEq
        PartialOrd
    */
}

