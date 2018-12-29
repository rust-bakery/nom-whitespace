
#[macro_use]
extern crate nom;

pub mod lib {
  pub use nom::IResult;
}

pub use self::whitespace::*;

#[macro_use]
mod whitespace;
