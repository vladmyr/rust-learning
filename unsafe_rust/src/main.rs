/**
 * Unsafe superpowers include hte ability to:
 * - Dereference a raw pointer
 * - Call an unsafe function or method
 * - Access or modify a mutable static variable
 * - Implement an usafe trait
 */

use std::slice;

static HELLO_WORLD: &str = "Hello, world!";
static mut COUNTER: u32 = 0;

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
  let len = slice.len();
  let ptr = slice.as_mut_ptr();

  assert!(mid <= len);

  unsafe {
    (
      slice::from_raw_parts_mut(ptr, mid), 
      slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid)
    )
  }
}

unsafe fn dangerous() {}

fn add_to_count(inc: u32) {
  unsafe {
    COUNTER += inc;
  }
}

fn main() {
  // *** Dereferencing a raw pointer
  // raw pointers:
  // - are allowed to ignore the borrowing rules by having both immutable and 
  //   mutable pointers or multiple mutable pointers to the same location
  // - Aren't guaranteed to point to valid memory
  // - Are allowed to be null
  // - Don't implement any automatic cleanup

  let mut num = 5;

  // creating raw pointes from reference
  let r1 = &num as *const i32;
  let r2 = &mut num as *mut i32;

  // raw pointers cannot be dereferences outside unsafe block
  unsafe {
    println!("r1 is: {}", *r1);
    println!("r2 is: {}", *r2);

    *r2 = 6;

    println!("r1 is: {}", *r1);
    println!("r2 is: {}", *r2);
  }

  // *** Calling an unsafe functon or method
  unsafe {
    dangerous();
  }

  // *** Creating a safe abstraction over unsafe code
  let mut v = vec![1, 2, 3, 4, 5, 6];
  let r = &mut v[..];

  let (a, b) = r.split_at_mut(3);

  assert_eq!(&mut [1, 2, 3], a);
  assert_eq!(&mut [4, 5, 6], b);

  // *** Accessing or modifying a mutable static variables
  println!("name is: {}", HELLO_WORLD);
  add_to_count(3);
  
  unsafe {
    println!("COUNTER: {}", COUNTER);
  }
}
