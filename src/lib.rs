//! Extends [ttf_parser](https://docs.rs/ttf-parser) with owned version of
//! [`Font`](struct.Font.html): [`OwnedFont`](struct.OwnedFont.html).
//!
//! Re-exports `ttf_parser::*`.
//!
//! # Example
//! ```
//! use owned_ttf_parser::{AsFontRef, OwnedFont, Font};
//!
//! # let owned_font_data = include_bytes!("../fonts/font.ttf").to_vec();
//! let owned_font = OwnedFont::from_vec(owned_font_data, 0).unwrap();
//! let font_ref: &Font<'_> = owned_font.as_font();
//!
//! assert_eq!(font_ref.ascender(), 2254);
//! ```
#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

mod convert;
mod owned;

pub use convert::*;
pub use owned::*;
pub use ttf_parser::*;
