use std::rc::Rc;

struct Person2 {
  name: Option<String>,
  birth: i32,
}

// if all fields of a struct are themselves implement "Copy" trait, then
// struct itself can derive from "Copy" trait
#[derive(Copy, Clone)]
struct Label {
  number: u32
}

// String type does not have "Copy" trait, therefore StringLabel cannot be
// derived form it
// #[derive(Copy, Clone)]
struct StringLabel {
  name: String
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn move_control_flow_demo() {
    let mut v = Vec::new();

    for i in 101..106 {
      v.push(i.to_string());
    }

    // let _third = v[2];   // error: cannot move out of indexed content
    let _third = &v[2];
    let _fifth = &v[4];

    // pop a value off the end of the vector
    let fifth = v.pop().unwrap();
    assert_eq!(fifth, "105");

    // move a value out of the middle of the vector, and move the last element 
    // into its spot
    let second = v.swap_remove(1);
    assert_eq!(second, "102");

    // swap in another value for the one we're taking out
    let third = std::mem::replace(&mut v[2], "substitute".to_string());
    assert_eq!(third, "103");

    assert_eq!(v, vec!["101", "104", "substitute"]);

    // demo #2 - moving out owned value example using Option
    let mut composers = Vec::new();
    composers.push(Person2 { 
      name: Some("Palestrina".to_string()), 
      birth: 1525,
    });

    let first_name = composers[0].name.take();
    // is the same as
    // let first_name = std::mem::replace(&mut composers[0].name, None);

    assert_eq!(first_name, Some("Palestrina".to_string()));
    assert_eq!(composers[0].name, None);
  }

  // Rc - reference count
  #[test]
  fn shared_ownership_demo() {
    let s: Rc<String> = Rc::new("shirataki".to_string());
    let t: Rc<String> = s.clone();  // clones reference address and increments a
                                    // reference count, value itself is left 
                                    // untouched
    let u: Rc<String> = s.clone();

    // type's methods can be used directly
    assert!(s.contains("shira"));
    assert_eq!(t.find("taki"), Some(5));
  }
}
