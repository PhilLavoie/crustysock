#![feature(globs)]
extern crate crustysock;

use crustysock::thincrust::protocol::*;

fn main() {
  let proto_db: Vec<Protocol> = FromIterator::from_iter(Protocol::database_iter());
  let proto_entry_db: Vec<ProtocolEntry> = FromIterator::from_iter(ProtocolEntry::database_iter());

  println!("Protocol database, {} entries:", proto_db.len());
  for ref proto in proto_db.iter() {
    println!("  {}", proto.protocol_number());
  }
  
  println!("Protocol entry database, {} entries:", proto_entry_db.len());
  for ref proto in proto_entry_db.iter() {
    println!("  {}", proto.protocol_number());
  }
}
