//TODO: think of a design for reading the protocol database, that keeps the "connection alive".
//There might be no real use case...
//TODO: figure out if I HAVE to export the trait publically outside this module.

//TODO: Define how errors and what errors are handled here.

//TODO: The retrieved protoent's number field is supposed to be in host endianness. Test it.

use c::funcs::{getprotobyname, getprotobynumber};
use c::types::{c_int, c_char, protoent};

use std::c_str::CString;

//Below are utility functions used to extract fields from struct protoent.
fn c_str_to_string(c_string: *const c_char, owns: bool) -> String {
  if c_string.is_null() { 
    return String::new(); 
  }
  return unsafe{ CString::new(c_string, owns) }.as_str().unwrap().to_string(); 
}

///Get the protocol number.
fn extract_number(entry: *const protoent) -> c_int {
  unsafe{ (*entry).p_proto }
}
///Get the protocol name. Allocates a copy.
fn extract_name(entry: *const protoent) -> String {
  c_str_to_string(unsafe{ (*entry).p_name }, false)
}
///Get the aliases of the protocol. Each one of them is copied into an
///allocated string.
fn extract_aliases(entry: *const protoent) -> Vec<String> {
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
trait FromNative {
  fn from_native(entry: *const protoent) -> Self;
}

///Structure representing a protocol. This is a stripped down version
///of ProtocolEntry. It does not hold any owned data regarding names and aliases.
///It is, however, perfectly usable with the rest of the api (only the protocol number
///matters in function calls).
pub struct Protocol {
  number: c_int
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

    if entry.is_null() {
      return Err(format!("unable to retrieve protocol by name: {}", name));
    }

    Ok(FromNative::from_native(entry))
  }
  ///Retrieves the protocol by number from the database.
  pub fn by_number(number: c_int) -> Result<Protocol, String> {
    let entry = unsafe{ getprotobynumber(number) };

    if entry.is_null() {
      return Err(format!("unable to retrieve protocol by number: {}", number));
    }

    Ok(FromNative::from_native(entry))
  }

  ///Sentinel value identifying the default protocol for a given address family
  ///and socket type. For example, the default for Ipv4 on a stream socket is
  ///tcp.
  pub fn default() -> Protocol { Protocol{ number: 0 } }
}

pub struct ProtocolEntry {
  name: String,
  aliases: Vec<String>,
  number: c_int 
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
  ///Retrieves the protocol by name from the database, like "tcp" or "udp" for example.
  pub fn by_name(name: &str) -> Result<ProtocolEntry, String> {
    let entry = unsafe{ getprotobyname(name.to_c_str().as_ptr()) };

    if entry.is_null() {
      return Err(format!("unable to retrieve protocol by name: {}", name));
    }
    
    Ok( 
      FromNative::from_native(entry) 
    )
  }

  ///Retrieves the protocol by number from the database.
  pub fn by_number(number: c_int) -> Result<ProtocolEntry, String> {
    let entry = unsafe{ getprotobynumber(number) };

    if entry.is_null() {
      return Err(format!("unable to retrieve protocol by number: {}", number));
    }

    Ok(FromNative::from_native(entry))
  }
}

///Functions that use the protocol structure actually only need its number.
///A trait is provided to allow both Protocol and ProtocolEntry to work
///with the api.
pub trait Proto {
  fn protocol_number(&self) -> c_int;
}

impl Proto for Protocol {
  fn protocol_number(&self) -> c_int { self.number }
}

impl Proto for ProtocolEntry {
  fn protocol_number(&self) -> c_int { self.number }
}

#[test]
fn test_endianness() {
  let proto = Protocol::by_name("tcp").unwrap();
  assert!(proto.protocol_number() == 6);
  let proto_entry = ProtocolEntry::by_number(6).unwrap();
  assert!(proto_entry.name.as_slice() == "tcp");
  assert!(proto_entry.protocol_number() == 6);
}
