extern crate private_module;

fn main() {
  // outermost would become accessible if it was public
  // private_module::outermost::middle_function();
}