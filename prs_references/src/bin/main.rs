use prs_references::Table;
use prs_references::show;
use prs_references::sort_works;

use prs_references::Point;

use prs_references::factorial;

fn main() {
  // demo #1
  let mut table = Table::new();
  table.insert(
    "Gesualdo".to_string(), 
    vec![
      "many madrigals".to_string(),
      "Tenebrae Responsoria".to_string(),
    ]
  );

  table.insert(
    "Caravaggio".to_string(),
    vec![
      "The Musicians".to_string(),
      "The Calling of St. Matthew".to_string(),
    ]
  );

  table.insert(
    "Cellini".to_string(),
    vec![
      "Perseus with the head of Medusa".to_string(),
      "a salt cellar".to_string(),
    ]
  );

  sort_works(&mut table);
  show(&table);

  // demo #2
  let x = 10;
  let r = &x;           // &x is a shared reference to x
  assert!(*r == 10);    // explicitly dereference r

  let mut y = 32;
  let m = &mut y;       // &mut y is a murable reference to y
  *m += 32;             // explicitly dereference m to set y's value
  assert!(*m == 64);

  // demo #3 - references to references
  let point = Point { x: 1000, y: 729 };
  let r: &Point = &point;
  let rr: &&Point = &r;
  let rrr: &&&Point = &rr;

  assert_eq!(rrr.y, 729);

  // demo #4 - using operators
  let x = 10;
  let y = 10;

  let rx = &x;
  let ry = &y;

  let rrx = &rx;
  let rry = &ry;

  // compares values references point to
  assert!(rrx <= rry);
  assert!(rrx == rry);

  // compare references themselves
  assert!(!std::ptr::eq(rx, ry));

  // demo $5 - borrowing references to arbitrary expressions
  let r = &factorial(6);
  assert_eq!(r + &1009, 1729);  // &1009 lives only within this instruction
}