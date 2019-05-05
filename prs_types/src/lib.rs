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
}
