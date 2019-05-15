use prs_references::Table;
use prs_references::show;
use prs_references::sort_works;

use prs_references::Point;

use prs_references::factorial;

use prs_references::S;

use prs_references::extend;

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

  // demo #5 - borrowing references to arbitrary expressions
  let r = &factorial(6);
  assert_eq!(r + &1009, 1729);  // &1009 lives only within this instruction

  // demo #6 - lifetimes
  let s;
  let x = 10;

  {
    let y = 20;
    
    {
      s = S { x: &x, y: &y };

      assert_eq!(*s.x, 10);
      assert_eq!(*s.y, 20);
    }
  }

  // demo #7
  let mut wave = Vec::new();
  let head = vec![0.0, 1.0];
  let tail = [0.0, -1.0];

  extend(&mut wave, &head);
  extend(&mut wave, &tail);

  assert_eq!(wave, vec![0.0, 1.0, 0.0, -1.0]);

  // Error: cannot borrow `wave` as immutable becaluse it is also borrowed as 
  // mutable. In other words, we may borrow a mutable reference to the vector, 
  // and we may borrow a shared reference ot its elements, but those two 
  // references's lifetimes may not overlap
  // extend(&mut wave, &wave);
  // assert_eq!(wave, vec![0.0, 1.0, 0.0, -1.0, 0.0, 1.0, 0.0, -1.0]);

  // demo #7.1
  let mut x = 10;
  let r1 = &x;
  let r2 = &x;        // OK: multiple shared borrows permitted
  // x += 10;         // ERROR: cannot assign to `x` becaues it is borrowed
  // let m = &mut x;  // ERROR: cannot borrow 'x' as mutable because it is also
                      // borrowed as immutable

  // demo #7.2
  let mut v = (136, 139);
  let m = &mut v;
  let m0 = &mut m.0;  // OK: reborrowing mutable from mutable
  *m0 = 137;
  let r1 = &m.1;      // OK: reborrowing shared from mutable and doesn't overlap
                      // with m0
  // let r2 = &v.1;   // ERORR: cannot borrow as immutable because it is also
                      // borrowed as mutable

  // demo #8
  let mut x = 42;     // nonconst i32 variable
  let p = &x;         // shared reference to i32

  assert_eq!(*p, 42);
  // x += 1;          // ERROR: cannot assign to x becaues it is borrowed
}