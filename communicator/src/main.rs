extern crate communicator;

fn main() {
  communicator::network::connect();
  communicator::server::connect();
  communicator::client::connect();
}