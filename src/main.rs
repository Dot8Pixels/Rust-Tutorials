// Define a trait for shapes
trait Shape {
    fn draw(&self);
}

// Define a struct for Circle
struct Circle {
    radius: f64,
}

// Implement Shape trait for Circle
impl Shape for Circle {
    fn draw(&self) {
        println!("Drawing a circle with radius {}", self.radius);
    }
}

// Define a struct for Rectangle
struct Rectangle {
    width: f64,
    height: f64,
}

// Implement Shape trait for Rectangle
impl Shape for Rectangle {
    fn draw(&self) {
        println!(
            "Drawing a rectangle with width {} and height {}",
            self.width, self.height
        );
    }
}

// Define a struct for managing drawings
struct Drawing {
    shapes: Vec<Box<dyn Shape>>,
}

impl Drawing {
    fn new() -> Self {
        Drawing { shapes: Vec::new() }
    }

    fn add_shape(&mut self, shape: Box<dyn Shape>) {
        self.shapes.push(shape);
    }

    fn draw_all(&self) {
        for shape in &self.shapes {
            shape.draw();
        }
    }
}

fn main() {
    let mut drawing = Drawing::new();
    drawing.add_shape(Box::new(Circle { radius: 5.0 }));
    drawing.add_shape(Box::new(Rectangle {
        width: 3.0,
        height: 4.0,
    }));

    drawing.draw_all();
}
