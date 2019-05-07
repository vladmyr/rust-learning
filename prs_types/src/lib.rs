#[cfg(test)]
mod tests {
  // Integers have methods
  #[test]
  fn integer_methods() {
    assert_eq!(2u16.pow(4), 16);
    assert_eq!((-4i32).abs(), 4);
    assert_eq!(0b101101u8.count_ones(), 4);
  }

  // The `f32` and `f64` types provide a full complement of methods for math
  // calculations
  #[test]
  fn floating_math_calculations() {
    assert_eq!(5f32.sqrt() * 5f32.sqrt(), 5.0);
    assert_eq!(-1.01f32.floor(), -1.0);
    assert!((-1.0 / std::f32::INFINITY).is_sign_negative());
  }

  // Character type `char` represents a single Unicode character, as a 32-bit 
  // value
  #[test]
  fn char_demo() {
    assert_eq!('*'.is_alphabetic(), false);
    assert_eq!('β'.is_alphabetic(), true);
    assert_eq!('8'.to_digit(10), Some(8));
    assert_eq!('ಠ'.len_utf8(), 3);
    assert_eq!(std::char::from_digit(2, 10), Some('2'));
  }

  // Tuples. Tuples allwo only constants as indices, like t.4, you can't write
  // t.i or t[i] to get i'th element
  // zero-tuple() is a "unit type" that is used to indicate there is no
  // meaningful value to carry
  #[test]
  fn tuple_demo() {
    let text = "I see the eigenvalue in thine eye";
    let (head, tail) = text.split_at(21);
    assert_eq!(head, "I see the eigenvalue ");
    assert_eq!(tail, "in thine eye");
  }

  // Arrays
  #[test]
  fn array_demo() {
    // demo #1
    let lazy_caterer: [u32; 6] = [1, 2, 4, 7, 11, 16];
    let taxonomy = ["Animatlia", "arthropoda", "Insecta"];

    assert_eq!(lazy_caterer[3], 7);
    assert_eq!(taxonomy.len(), 3);

    // demo #2
    let mut sieve = [true; 10000];  // all elements set to `true`
    for i in 2..100 {
      if sieve[i] {
        let mut j = i * i;
        while j < 10000 {
          sieve[j] = false;
          j += i;
        }
      }
    }

    assert!(sieve[211]);
    assert!(!sieve[9876]);

    // demo #3
    let mut chaos = [3, 5, 4, 1, 2];
    chaos.sort();
    assert_eq!(chaos, [1, 2, 3, 4, 5]);
  }

  // Vectors
  #[test]
  fn vector_demo() {
    // demo #1 - dynamic resize
    let mut v = Vec::with_capacity(2);
    assert_eq!(v.len(), 0);
    assert_eq!(v.capacity(), 2);

    v.push(1);
    v.push(2);
    assert_eq!(v.len(), 2);
    assert_eq!(v.capacity(), 2);

    v.push(3);
    assert_eq!(v.len(), 3);
    assert_eq!(v.capacity(), 4);

    // demo #2
    let mut v = vec![10, 20, 30, 40, 50];

    v.insert(3, 35);
    assert_eq!(v, vec![10, 20, 30, 35, 40, 50]);

    v.remove(1);
    assert_eq!(v, vec![10, 30, 35, 40, 50]);

    // demo #3
    let mut v = vec!["carmen", "miranda"];
    assert_eq!(v.pop(), Some("miranda"));
    assert_eq!(v.pop(), Some("carmen"));
    assert_eq!(v.pop(), None);
  }
}
