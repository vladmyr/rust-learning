use std::thread;
use std::time::Duration;
use std::collections::HashMap;

struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: HashMap<u32, u32>,
}


impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
      let v = self.value.entry(arg).or_insert((self.calculation)(arg));
      *v
    }
}

pub fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes", expensive_result.value(intensity));
        }
    }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn cacher_arg_sensitive() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v1, 1);
    assert_eq!(v2, 2);
  }

  // TODO: make Cacher accept generic types
  #[test]
  fn cacher_arg_generic() {
    assert_eq!(false, false);
  }

  #[test]
  fn closure_capture_environment() {
    let x = 4;
    let y = 4;
    let equal_to_x = |z| z == x;

    assert!(equal_to_x(y));
  }
}