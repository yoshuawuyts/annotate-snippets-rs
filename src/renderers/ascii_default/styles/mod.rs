#[cfg(feature = "ansi_term")]
pub mod color;
#[cfg(feature = "termcolor")]
pub mod color2;
pub mod plain;

use std::fmt;

pub trait Style {
    fn fmt(w: &mut dyn std::io::Write, pattern: impl fmt::Display) -> std::io::Result<()>;
}