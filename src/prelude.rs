//! Re-exports of the most common traits and types.
pub use super::*;

pub use self::{
    color::{palette::Palette, RGB},
    ditherer::{Dither, Ditherer},
    error::{Error, IOError, Result},
    img::Img,
};
