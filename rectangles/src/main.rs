// With methods

#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle {
  fn area(&self) -> u32 {
    self.width * self.height
  }

  fn width(&self) -> bool {
    self.width > 0
  }
}

fn main() {
  let rect1 = Rectangle {
    width: 30,
    height: 50,
  };

  println!(
    "The area of the rectangle is {} square pixels.",
    rect1.area()
  );

  if rect1.width() {
    println!("The rectangle has a nonzero width; it is {}", rect1.width);
  }
}

/*
// The parameter is an immutable borrow for a struct Rectangle instance.
// main retains the ownership of this instance and can continue using it.
fn area(rectangle: &Rectangle) -> u32 {
  rectangle.width * rectangle.height
}

// With structs

#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

fn main() {
  let scale = 2;
  let rect1 = Rectangle {
    width: dbg!(30 * scale),
    height: 50
  };

  println!("rect1 is {:?}", rect1);

  println!(
    "The area of the rectangle is {} square pixels.",
    area(&rect1)
  );

  dbg!(&rect1);
}

// The parameter is an immutable borrow for a struct Rectangle instance.
// main retains the ownership of this instance and can continue using it.
fn area(rectangle: &Rectangle) -> u32 {
  rectangle.width * rectangle.height
}

// With tuples.
fn main() {
  let rect1 = (30, 50);

  println!(
    "The area of the rectangle is {} square pixels.",
    area(rect1)
  );
}

fn area(dimensions: (u32, u32)) -> u32 {
  dimensions.0 * dimensions.1
}

// With single variables.
fn main() {
  let width1 = 30;
  let height1 = 50;

  println!(
    "The area of the rectangle is {} square pixels.",
    area(width1, height1)
  );
}

fn area(width: u32, height: u32) -> u32 {
  width * height
}
*/
