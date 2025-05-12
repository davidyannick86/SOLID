/*
Les entités logicielles (classes, modules, fonctions) doivent être ouvertes à l'extension, mais fermées à la modification.
 */

pub trait Shape {
    fn area(&self) -> f64;
}

pub struct Circle {
    pub radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

pub fn print_area(shape: &dyn Shape) {
    println!("Area: {}", shape.area());
}
