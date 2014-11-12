use c::funcs::{getprotobyname};
use c::types::{c_int, protoent};

use std::c_str::CString;

fn extract_number(entry: *const protoent) -> c_int {
  unsafe{ (*entry).p_proto }
}

fn extract_name(entry: *const protoent) -> String {
  unsafe{ CString::new((*entry).p_name, false).as_str().unwrap().to_string() } 
}

fn extract_aliases(entry: *const protoent) -> Vec<String> {
  let mut aliases: Vec<String> = Vec::new();
  let mut current_alias = unsafe{ (*entry).p_aliases };
    
  while !current_alias.is_null() {
    let rust_string = 
      unsafe{ CString::new(*current_alias, false).as_str().unwrap().to_string() };
      
    aliases.push(rust_string);

    current_alias = unsafe{ current_alias.offset(1) }; 
  }

  aliases
}

trait FromNative {
  fn from_native(entry: *const protoent) -> Self;
}

pub struct Protocol {
  number: c_int
}

impl FromNative for Protocol {
  fn from_native(entry: *const protoent) -> Protocol {
    Protocol{ number: extract_number(entry) }
  }
}

impl Protocol {
  pub fn by_name(name: &str) -> Result<Protocol, String> {
    let entry = unsafe{ getprotobyname(name.to_c_str().as_ptr()) };

    if entry.is_null() {
      return Err(format!("unable to retrieve protocol by name: {}", name));
    }

    Ok( 
      FromNative::from_native(entry)
    )
  }

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
  
  pub fn by_name(name: &str) -> Result<ProtocolEntry, String> {
    let entry = unsafe{ getprotobyname(name.to_c_str().as_ptr()) };

    if entry.is_null() {
      return Err(format!("unable to retrieve protocol by name: {}", name));
    }
    
    Ok( 
      FromNative::from_native(entry) 
    )
  }
}

pub trait Proto {
  fn protocol_number(&self) -> c_int;
}

impl Proto for Protocol {
  fn protocol_number(&self) -> c_int { self.number }
}

impl Proto for ProtocolEntry {
  fn protocol_number(&self) -> c_int { self.number }
}


