struct Point {
  x: i32,
  y: i32
}

struct Point3d {
  x: i32,
  y: i32,
  z: i32
}

enum Message {
  Quit,
  Move { x: i32, y: i32 },
  Write(String),
  ChangeColor(i32, i32, i32),
}

enum Color {
  Rgb(i32, i32, i32),
  Hsv(i32, i32, i32)
}

enum Message2 {
  Quit,
  Move { x: i32, y: i32 },
  Write(String),
  ChangeColor(Color),
}

enum Message3 {
  Hello { id: i32 }
}

fn foo (_: i32, y: i32) {
  println!("This code only uses the y parameter: {}", y);
}

fn main() {
  // *** conditional "if let" expression
  // the downside is that the compiler doesn't check exhaustiveness, whereas 
  // with "math" expressions it does. Omitting the last "else" block would not
  // alert us the possible logic bug.
  let favorite_color: Option<&str> = Some("blue");  // let PATTERN = EXPRESSION
  let is_tuesday = false;
  let age: Result<u8, _> = "34".parse();

  if let Some(color) = favorite_color {
    println!("Using your favorite color, {}, as the background", color);
  } else if is_tuesday {
    println!("Tuesday is green day!");
  } else if let Ok(age) = age {
    if age > 30 {
      println!("Using purple as the background color");
    } else {
      println!("Using orange as the backgroung color");
    }
  } else {
    println!("using blue as the background color");
  }

  // *** "while let" conditional loop
  let mut stack = Vec::new();

  stack.push(1);
  stack.push(2);
  stack.push(3);

  while let Some(top) = stack.pop() {
    println!("{}", top);
  }

  // ** Pattern Syntax
  // *** Matching literals
  let x = 1;

  match x {
    1 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),
    _ => println!("anything")
  }

  // *** Matching Named Variables
  // let x = Some(5);
  let x = None;
  let y = 10;

  match x {
    Some(50) => println!("Got 50"),
    Some(n) if n == y => println!("Matched, n = {:?}", x),
    Some(y) => println!("Matched, y = {:?}", y),  // y matched any value inside of Some value
    _ => println!("Default case, x = {:?}", x),
  }

  println!("at the end: x = {:?}, y = {:?}", x, y);

  // *** Multiple patterns
  let x = 1;

  match x {
    1 | 2 => println!("one or two"),
    3 => println!("three"),
    _ => println!("anything")
  }

  // *** Matching ranges of values
  let x = 4;

  match x {
    1 ... 5 => println!("one through five"),
    _ => println!("anything"),
  }

  // also works with char data type
  let x = 'c';

  match x {
    'a' ... 'j' => println!("early ASCII letter"),
    'k' ... 'z' => println!("late ASCII letter"),
    _ => println!("anything"),
  }

  // ** Destructuring
  // *** Destructuring Structs
  let p = Point { x: 0, y: 7 };
  let Point { x: a, y: b } = p;

  assert_eq!(0, a);
  assert_eq!(7, b);

  // short syntax
  let Point { x, y } = p;

  assert_eq!(0, x);
  assert_eq!(7, y);

  match p {
    Point { x, y: 0 } => println!("One the x axis at {}", x),
    Point { x: 0, y } => println!("One the y asix at {}", y),
    Point { x, y } => println!("On neither axis: ({}, {})", x, y),
  }

  // *** Destructuring Enums
  let msg = Message::ChangeColor(0, 160, 255);

  match msg {
    Message::Quit => println!("The Quit variant has no data to destructure"),
    Message::Move { x, y } => println!(
      "Move in the x direction {} and in the y direction {}",
      x,
      y
    ),
    Message::Write(text) => println!("Text message: {}", text),
    Message::ChangeColor(r, g, b) => println!(
      "Change the color to red {}, green {}, blue {}",
      r, 
      g, 
      b
    )
  }

  // *** Destructuring nested structs and Enums
  let msg = Message2::ChangeColor(Color::Hsv(0, 160, 255));

  match msg {
    Message2::ChangeColor(Color::Rgb(r, g, b)) => println!(
      "Change the color to red {}, green {}, blue {}", r, g, b
    ),
    Message2::ChangeColor(Color::Hsv(h, s, v)) => println!(
      "Change the color hue {}, saturation {}, and value {}", h, s, v
    ),
    _ => ()
  }

  // *** Destrcturing Structs and Tuples
  let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
  println!("feet = {}, inches = {} and point ({}, {})", feet, inches, x, y);

  // ** Ignoring Values in a Pattern
  // *** Ignoring the entire value with _
  foo(3, 4);

  // *** Ignoring parts of a value with a nested _
  let mut setting_value = Some(5);
  let new_setting_value = Some(10);

  match (setting_value, new_setting_value) {
    (Some(_), Some(_)) => println!("Can't overwrite an existing customized value"),
    _ => setting_value = new_setting_value,
  }

  println!("setting is {:?}", setting_value);

  let numbers = (2, 4, 8, 16, 32);

  match numbers {
    (first, _, third, _, fifth) => println!("Some numbers: {}, {}, {}", first, third, fifth)
  }

  // *** Ignore an unused variable by starting its name with _
  let _x = 5;   // <- no warning

  // ** ignore remaining parts of a value with ..new_setting_value
  let origin = Point3d { x: 0, y: 0, z: 0 };

  match origin {
    Point3d { x, .. } => println!("x is {}", x),
  }

  let numbers = (2, 4, 8, 16, 32);

  match numbers {
    (first, .., last) => println!("Some nubmers: {}, {}", first, last),
  }

  // ** Match Guards
  let num = Some(4);

  match num {
    Some(x) if x < 5 => println!("less than five: {}", x),
    Some(x) => println!("{}", x),
    None => (),
  }

  let x = 4;
  let y = false;

  match x {
    4 | 5 | 6 if y => println!("yes"),
    _ => println!("no"),
  }

  // ** @ binding
  let msg = Message3::Hello { id: 5 };

  match msg {
    Message3::Hello { id: id_variable @ 3...7 } => println!(
      "Found an id in range: {}", id_variable
    ),
    Message3::Hello { id: 10...12 } => println!(
      "Found an id in another range"
    ),
    Message3::Hello { id } => println!("Found some other id: {}", id),
  }
}
