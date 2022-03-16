use serde_derive::{Deserialize, Serialize};

pub mod bxcad;
pub(crate) mod bytestream_addon;

pub use bxcad::{bccad::BCCAD, brcad::BRCAD};

#[derive(Serialize, Deserialize)]
pub struct Color {
    pub red: u8,
    pub blue: u8,
    pub green: u8,
}

pub struct VarLenString(String);
