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
}
