use c::funcs::{getprotobyname};
use c::types::{c_int, protoent};

use std::c_str::CString;


pub struct Protocol {
  number: c_int
}

impl Protocol {
  pub fn by_name(name: &str) -> Result<Protocol, String> {
    let entry = unsafe{ getprotobyname(name.to_c_str().as_ptr()) };

    if entry.is_null() {
      return Err(format!("unable to retrieve protocol by name: {}", name));
    }

    Ok( 
      Protocol {
        number: unsafe{ (*entry).p_proto }
      }
    )
  }
 
}

pub struct ProtocolEntry {
  name: String,
  aliases: Vec<String>,
  number: c_int 
}

impl ProtocolEntry {
  pub fn by_name(name: &str) -> Result<ProtocolEntry, String> {
    let entry = unsafe{ getprotobyname(name.to_c_str().as_ptr()) };

    if entry.is_null() {
      return Err(format!("unable to retrieve protocol by name: {}", name));
    }

    let mut aliases: Vec<String> = Vec::new();
    let mut current_alias = unsafe{ (*entry).p_aliases };
    
    while !current_alias.is_null() {
      let rust_string = 
        unsafe{ CString::new(*current_alias, false).as_str().unwrap().to_string() };
      
      aliases.push(rust_string);

      current_alias = unsafe{ current_alias.offset(1) }; 
    }

    Ok( 
      ProtocolEntry {
        name: unsafe{ CString::new((*entry).p_name, false).as_str().unwrap().to_string() },
        aliases: aliases,
        number: unsafe{ (*entry).p_proto }
      }
    )
  }

  pub fn default() -> ProtocolEntry {
    ProtocolEntry{ name: String::new(), aliases: Vec::new(), number: 0 }
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


