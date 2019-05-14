use std::collections::HashMap;

pub type Table = HashMap<String, Vec<String>>;

pub fn show(table: &Table) {
  for (artist, works) in table {
    println!("works by {}:", artist);

    for work in works {
      println!(" {}", work);
    }
  }
}

pub fn sort_works(table: &mut Table) {
  for (_artist, works) in table {
    works.sort()
  }
}

pub struct Point { 
  pub x: i32, 
  pub y: i32 
}

pub fn factorial (n: usize) -> usize {
  (1..n + 1).fold(1, |a, b| a * b)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn factorial_test() {
    assert_eq!(factorial(1), 1);
    assert_eq!(factorial(2), 2);
    assert_eq!(factorial(3), 6);
    assert_eq!(factorial(4), 24);
  }


}