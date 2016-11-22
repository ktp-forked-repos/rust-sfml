use csfml_graphics_sys as ffi;
use graphics::{FloatRect, IntRect};

use raw_conv::{Raw, FromRaw};

/// Structure describing a glyph.
///
/// A glyph is the visual representation of a character.
///
/// The sf::Glyph structure provides the information needed to handle the glyph:
///
/// - its coordinates in the font's texture
/// - its bounding rectangle
/// - the offset to apply to get the starting position of the next glyph
#[derive(Clone, Copy, Debug, Default)]
pub struct Glyph {
    /// Offset to move horizontally to the next character.
    pub advance: f32,
    /// Bounding rectangle of the glyph, in coordinates relative to the baseline.
    pub bounds: FloatRect,
    /// Texture coordinates of the glyph inside the font's texture.
    pub texture_rect: IntRect,
}

impl Raw for Glyph {
    type Raw = ffi::sfGlyph;

    fn raw(&self) -> Self::Raw {
        unsafe { ::std::mem::transmute(*self) }
    }
}

impl FromRaw for Glyph {
    fn from_raw(raw: Self::Raw) -> Self {
        unsafe { ::std::mem::transmute(raw) }
    }
}
