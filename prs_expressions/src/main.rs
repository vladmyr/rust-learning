// demo #2 - example of a divergent function
fn exit(code: i32) -> ! {
  std::process::exit(code);
}

fn main() {
  let v1 = vec![10, 11, 12, 13, 14];
  let v2 = vec![6, 3, 4, 8, 7];

  // demo #1 - loop can be labeled
  'search:
  for a in v1.iter() {
    for b in v2.iter() {
      if a % b == 0 {
        println!("{} % {} = 0", a, b);
        break 'search;
      }
    }
  }

  exit(0);
}