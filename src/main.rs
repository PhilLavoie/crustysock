#![feature(globs)]
extern crate crustysock;

use crustysock::c::consts::*;

fn main() {
  println!("SOCK_NONBLOCK: {}", SOCK_NONBLOCK);
  println!("SOCK_CLOEXEC:  {}", SOCK_CLOEXEC);
}
