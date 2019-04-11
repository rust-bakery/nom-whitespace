//! Support for whitespace delimited formats
//!
//! a lot of textual formats allows spaces and other
//! types of separators between tokens. Handling it
//! manually with nom means wrapping all parsers
//! like this:
//!
//! ```ignore
//! named!(token, delimited!(space, tk, space));
//! ```
//!
//! To ease the development of such parsers, you
//! can use the whitespace parsing facility, which works
//! as follows:
//!
//! ```
//! # #[macro_use] extern crate nom;
//! # fn main() {
//! named!(tuple<&[u8], (&[u8], &[u8]) >,
//!   ws!(tuple!( take!(3), tag!("de") ))
//! );
//!
//! assert_eq!(
//!   tuple(&b" \t abc de fg"[..]),
//!  Ok((&b"fg"[..], (&b"abc"[..], &b"de"[..])))
//! );
//! # }
//! ```
//!
//! The `ws!` combinator will modify the parser to
//! intersperse space parsers everywhere. By default,
//! it will consume the following characters: `" \t\r\n"`.
//!
//! If you want to modify that behaviour, you can make
//! your own whitespace wrapper. As an example, if
//! you don't want to consume ends of lines, only
//! spaces and tabs, you can do it like this:
//!
//! ```
//! # #[macro_use] extern crate nom;
//! named!(pub space, eat_separator!(&b" \t"[..]));
//!
//! #[macro_export]
//! macro_rules! sp (
//!   ($i:expr, $($args:tt)*) => (
//!     {
//!       use nom::Convert;
//!       use nom::Err;
//!
//!       match sep!($i, space, $($args)*) {
//!         Err(e) => Err(e),
//!         Ok((i1,o))    => {
//!           match space(i1) {
//!             Err(e) => Err(Err::convert(e)),
//!             Ok((i2,_))    => Ok((i2, o))
//!           }
//!         }
//!       }
//!     }
//!   )
//! );
//!
//! # fn main() {
//! named!(tuple<&[u8], (&[u8], &[u8]) >,
//!   sp!(tuple!( take!(3), tag!("de") ))
//! );
//!
//! assert_eq!(
//!   tuple(&b" \t abc de fg"[..]),
//!  Ok((&b"fg"[..], (&b"abc"[..], &b"de"[..])))
//! );
//! # }
//! ```
//!
//! This combinator works by replacing each combinator with
//! a version that supports wrapping with separator parsers.
//! It will not support the combinators you wrote in your
//! own code. You can still manually wrap them with the separator
//! you want, or you can copy the macros defined in src/whitespace.rs
//! and modify them to support a new combinator:
//!
//! * copy the combinator's code here, add the _sep suffix
//! * add the `$separator:expr` as second argument
//! * wrap any sub parsers with sep!($separator, $submac!($($args)*))
//! * reference it in the definition of `sep!` as follows:
//!
//! ```ignore
//!  ($i:expr,  $separator:path, my_combinator ! ($($rest:tt)*) ) => {
//!    wrap_sep!($i,
//!      $separator,
//!      my_combinator_sep!($separator, $($rest)*)
//!    )
//!  };
//! ```
//!
#![cfg_attr(not(feature = "std"), no_std)]

#[macro_use]
extern crate nom;

pub mod lib {
  #[cfg(not(feature = "std"))]
  pub mod std {
    pub use core::{option, result};
  }

  #[cfg(feature = "std")]
  pub mod std {
    pub use std::{option, result};
  }

  pub mod nom {
    pub use nom::{Err,Convert,IResult,ErrorKind, AsChar, FindToken, InputTakeAtPosition};
  }
}

pub use self::whitespace::*;

#[macro_use]
mod whitespace;
