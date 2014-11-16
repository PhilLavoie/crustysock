//TODO: Define how errors and what errors are handled here. Problem: There is no mention of error handling for this in the linux manual, however windows seem to provide error codes for these functions.
//MODULE need to be tested sequentially, maybe whole crate. Will investigate...
use c::funcs::{getprotobyname, getprotobynumber, getprotoent, setprotoent, endprotoent};
use c::types::{c_int, c_char, protoent};

use std::c_str::CString;
use std::iter::Iterator;
use std::iter::order;
use std::vec::Vec;

pub type ProtocolInt = c_int;

///Native c string to rust owned String conversion.
fn c_str_to_string(c_string: *const c_char, owns: bool) -> String {
  assert!(!c_string.is_null());
  return unsafe{ CString::new(c_string, owns) }.as_str().unwrap().to_string(); 
}

//Below are utility functions used to extract fields from struct protoent.

///Get the protocol number.
fn extract_number(entry: *const protoent) -> c_int {
  assert!(!entry.is_null());
  unsafe{ (*entry).p_proto }
}

///Get the protocol name. Allocates a copy.
fn extract_name(entry: *const protoent) -> String {
  assert!(!entry.is_null());
  c_str_to_string(unsafe{ (*entry).p_name }, false)
}

/**
  Get the aliases of the protocol. Each one of them is copied into an
  allocated string.
*/
fn extract_aliases(entry: *const protoent) -> Vec<String> {
  assert!(!entry.is_null());

  let mut aliases: Vec<String> = Vec::new();
  let mut current_alias = unsafe{ (*entry).p_aliases };

  if current_alias.is_null() { return aliases; }
    
  while !(unsafe{ (*current_alias) }.is_null()) {
    let rust_string = 
      c_str_to_string(unsafe{ *current_alias }, false);
  
    aliases.push(rust_string);

    current_alias = unsafe{ current_alias.offset(1) }; 
  }

  aliases
}

///Trait to convert the native protocol entry to an equivalent rust protocol type.
pub trait FromNative {
  fn from_native(entry: *const protoent) -> Self;
}

fn result_for<P: FromNative, E>(entry: *const protoent, err: || -> E) -> Result<P, E> {
  if entry.is_null() { return Err(err()); }
  return Ok(FromNative::from_native(entry));
}

///To be used for listing all database entries.
///It is very important to note that this structure is just a layer above
///the calls setprotoent(), getprotoent() and endprotoent(), provided from the C api.
///Constructing an instance of this type should ONLY BE DONE when no other instances
///are in use. 2 instances should never be used in parallel. If such a feature is needed,
///it is strongly suggested that the user makes a local copy of the whole database
///(which isn't very large anyways).
///When the iterator is constructed, it opens a connection to the database and rewinds
///it's current entry to the first one. Every call to next() fetches and moves the current
///entry. When the end of the file has been reached, None is returned. When the iterator is
///destroyed, the connection to the database is closed. 
pub struct DatabaseIterator<P: FromNative>;

impl<P: FromNative> DatabaseIterator<P> {
  fn new() -> DatabaseIterator<P> { 
    unsafe{ setprotoent(1) }; //Stay open for fast access and rewind to first entry.
    DatabaseIterator
  }
}

#[unsafe_destructor]
impl<P: FromNative> Drop for DatabaseIterator<P> {
  fn drop(&mut self) { unsafe{ endprotoent() }; }
}

impl<P: FromNative> Iterator<P> for DatabaseIterator<P> {
  fn next(&mut self) -> Option<P> {
   let entry = unsafe{ getprotoent() };

    if entry.is_null() {
      return None;
    }

    return Some(FromNative::from_native(entry)); 
  }
}

///Structure representing a protocol. This is a stripped down version
///of ProtocolEntry. It does not hold any owned data regarding names and aliases.
///It is, however, perfectly usable with the rest of the api (only the protocol number
///matters in function calls).
pub struct Protocol {
  number: ProtocolInt
}

impl FromNative for Protocol {
  fn from_native(entry: *const protoent) -> Protocol {
    Protocol{ number: extract_number(entry) }
  }
}

impl Protocol {
  ///Retrieves the protocol by name from the database, like "tcp" or "udp" for example.
  pub fn by_name(name: &str) -> Result<Protocol, String> {
    let entry = unsafe{ getprotobyname(name.to_c_str().as_ptr()) };
    result_for(entry, || { format!("unable to retrieve protocol by name: {}", name) } )
  }
  ///Retrieves the protocol by number from the database.
  pub fn by_number(number: ProtocolInt) -> Result<Protocol, String> {
    let entry = unsafe{ getprotobynumber(number) };
    result_for(entry, || { format!("unable to retrieve protocol by number: {}", number) } )
  }
  ///Returns a database iterator of procotols.
  pub fn database_iter() -> DatabaseIterator<Protocol> {
    DatabaseIterator
  }
  ///Sentinel value identifying the default protocol for a given address family
  ///and socket type. For example, the default for Ipv4 on a stream socket is
  ///tcp.
  pub fn default() -> Protocol { Protocol::new(0) }
  ///This is a constructor provided in case, for some reason, either the database is unavailable
  ///or the protocol desired is not in there.
  pub fn new(proto: ProtocolInt) -> Protocol { Protocol{ number: proto } }
}

pub struct ProtocolEntry {
  name: String,
  aliases: Vec<String>,
  number: ProtocolInt
}

impl FromNative for ProtocolEntry {
  fn from_native(entry: *const protoent) -> ProtocolEntry {
    ProtocolEntry {
      name: extract_name(entry),
      aliases: extract_aliases(entry),
      number: extract_number(entry)
    }
  }
}

impl ProtocolEntry {
  ///Retrieves the protocol entry by name from the database, like "tcp" or "udp" for example.
  pub fn by_name(name: &str) -> Result<ProtocolEntry, String> {
    let entry = unsafe{ getprotobyname(name.to_c_str().as_ptr()) };
    result_for(entry, || { format!("unable to retrieve protocol by name: {}", name) } )
  }
  ///Retrieves the protocol entry by number from the database.
  pub fn by_number(number: ProtocolInt) -> Result<ProtocolEntry, String> {
    let entry = unsafe{ getprotobynumber(number) };
    result_for(entry, || { format!("unable to retrieve protocol by number: {}", number) } )
  }
  ///Returns a database iterator of protocol entries.
  pub fn database_iter() -> DatabaseIterator<ProtocolEntry> {
    DatabaseIterator
  }
}

///Functions that use the protocol structure actually only need its number.
///A trait is provided to allow both Protocol and ProtocolEntry to work
///with the api.
pub trait Proto {
  fn protocol_number(&self) -> ProtocolInt;
}
impl Proto for Protocol {
  fn protocol_number(&self) -> ProtocolInt { self.number }
}
impl Proto for ProtocolEntry {
  fn protocol_number(&self) -> ProtocolInt { self.number }
}

#[test]
fn test_by_number() {
  {
    let proto = Protocol::by_number(6);
    assert!(proto.is_ok());
    //This also tests for host endianness.
    assert!(proto.unwrap().protocol_number() == 6);
  }
  {
    let proto_entry = ProtocolEntry::by_number(17);
    assert!(proto_entry.is_ok());
    //TODO: Find a way to easily do case insensitive string comparisons
    //because there is no guarantee on caps me thinks, anyways shouldn't assume.
    let unwrapped = proto_entry.unwrap();
    assert!(unwrapped.name.as_slice() == "udp");
    assert!(unwrapped.protocol_number() == 17);
  }
}

#[test]
fn test_by_name() {
  {
    let mut proto = Protocol::by_name("icmp");
    assert!(proto.is_ok());
    assert!(proto.unwrap().protocol_number() == 1);
    proto = Protocol::by_name("tcp");
    assert!(proto.is_ok());
    assert!(proto.unwrap().protocol_number() == 6);
    proto = Protocol::by_name("udp");
    assert!(proto.is_ok());
    assert!(proto.unwrap().protocol_number() == 17);
  }
  {
    let mut proto = ProtocolEntry::by_name("icmp");
    assert!(proto.is_ok());
    let mut unwrapped = proto.unwrap();
    assert!(unwrapped.protocol_number() == 1);
    assert!(unwrapped.name.as_slice() == "icmp");

    proto = ProtocolEntry::by_name("tcp");
    assert!(proto.is_ok());
    unwrapped = proto.unwrap();
    assert!(unwrapped.protocol_number() == 6);
    assert!(unwrapped.name.as_slice() == "tcp");

    proto = ProtocolEntry::by_name("udp");
    assert!(proto.is_ok());
    unwrapped = proto.unwrap();
    assert!(unwrapped.protocol_number() == 17);
    assert!(unwrapped.name.as_slice() == "udp");
  }
}

#[test]
fn test_database_iter() {
  let proto_db: Vec<Protocol> = FromIterator::from_iter(Protocol::database_iter());
  let proto_entry_db: Vec<ProtocolEntry> = FromIterator::from_iter(ProtocolEntry::database_iter());

  //At the very least we expect both iterator to provide the same database.
  assert!(proto_db.len() == proto_entry_db.len(), "protocol database length: {}, protocol entry database length: {}", proto_db.len(), proto_entry_db.len());
  assert!(
    order::cmp(
      proto_db.iter().map(|p: &Protocol| { return p.protocol_number() }),
      proto_entry_db.iter().map(|p: &ProtocolEntry| { return p.protocol_number() }),
    ) == Equal
  );
}
