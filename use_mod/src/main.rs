pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

#[derive(Debug)]
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

use a::series::of;                  // use namespace
use TrafficLight::{Red, Yellow};
// use TrafficLight::*;

// Nested groups in `use`
// use foo::{
//     bar::{self, Foo},
//     baz::{*, quux::Bar}
// }

fn main() {
    println!("Hello, world!");
    of::nested_modules();

    let red = Red;
    let yellow = Yellow;
    let green = TrafficLight::Green;    // here we need to specify the full path

    println!("red is {:?}", red);
    println!("yellow is {:?}", yellow);
    println!("green is {:?}", green);
}
