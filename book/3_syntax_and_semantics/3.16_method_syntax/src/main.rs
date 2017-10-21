
use std::f64::consts::PI;

fn main() {
  let circle = Circle::new(0.0,0.0,2.0);
  println!("{}", circle.area());
  println!("{}", circle.grow(1.0).area());

  let circle = CircleBuilder::new()
    .x(1.0)
    .y(1.0)
    .radius(2.0)
    .finalize();
  println!("{}", circle.area());
  println!("{}", circle.grow(1.0).area());

}

struct Circle {
    x: f64
  , y: f64
  , radius: f64
  }

impl Circle {
  fn new(x: f64, y: f64, radius: f64) -> Circle {
    Circle { x: x, y: y, radius: radius }
  }

  fn area(&self) -> f64 {
    PI * (self.radius * self.radius)
  }
  fn grow(&self, increment: f64) -> Circle {
    Circle::new(self.x, self.y
      , self.radius + increment
      )
  }
}


struct CircleBuilder {
    x: f64
  , y: f64
  , radius: f64
  }

impl CircleBuilder {
  fn new() -> CircleBuilder {
    CircleBuilder {
        x: 0.0
      , y: 0.0
      , radius: 1.0
      }
  }

  fn x(&mut self, coordinate: f64) -> &mut CircleBuilder {
    self.x = coordinate;
    self
  }
  fn y(&mut self, coordinate: f64) -> &mut CircleBuilder {
    self.y = coordinate;
    self
  }
  fn radius(&mut self, radius: f64) -> &mut CircleBuilder {
    self.radius = radius;
    self
  }

  fn finalize(&self) -> Circle {
    Circle::new(self.x, self.y, self.radius)
  }
}

