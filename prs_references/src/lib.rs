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

pub struct S<'a, 'b> {
  pub x: &'a i32,
  pub y: &'b i32,
}

pub struct T<'a, 'b> {
  pub s: S<'a, 'b>
}

pub struct StringTable {
  elements: Vec<String>,
}

impl StringTable {
  // fn find_by_prefix<'a, 'b>(&'a self, prefix: &'b str) -> Option<&'a String>
  fn find_by_prefix(&self, prefix: &str) -> Option<&String> {
    for i in 0..self.elements.len() {
      if self.elements[i].starts_with(prefix) {
        return Some(&self.elements[i]);
      }
    }
    None
  }
}

pub fn extend(vec: &mut Vec<f64>, slice: &[f64]) {
  for elt in slice {
    vec.push(*elt);
  }
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