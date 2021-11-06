//! Logic to avoid re-parsing subtables in ttf_parser::Face methods
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;
use core::fmt;
use ttf_parser::{cmap, kern, Face, GlyphId};

use crate::{AsFaceRef, OwnedFace};

/// A `Face` with cmap & kern subtables parsed once on initialization.
///
/// For best performance use [`PreParsedSubtables::glyph_index`] &
/// [`PreParsedSubtables::glyphs_kerning`] rather than `.as_face_ref()`
/// equivalents that must parse their subtables on each call.
#[derive(Clone)]
pub struct PreParsedSubtables<'face, F> {
    pub(crate) face: F,
    // note must not be public as could be self-referencing
    pub(crate) subtables: FaceSubtables<'face>,
}

impl<'face> From<Face<'face>> for PreParsedSubtables<'face, Face<'face>> {
    fn from(face: Face<'face>) -> Self {
        let subtables = FaceSubtables::from(&face);
        Self { face, subtables }
    }
}

impl From<OwnedFace> for PreParsedSubtables<'static, OwnedFace> {
    fn from(face: OwnedFace) -> Self {
        face.pre_parse_subtables()
    }
}

impl<F> fmt::Debug for PreParsedSubtables<'_, F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PreParsedSubtables")
    }
}

#[derive(Clone)]
pub(crate) struct FaceSubtables<'face> {
    cmap: Vec<cmap::Subtable<'face>>,
    kern: Vec<kern::Subtable<'face>>,
}

impl<'face> From<&Face<'face>> for FaceSubtables<'face> {
    fn from(face: &Face<'face>) -> Self {
        let cmap = face
            .tables()
            .cmap
            .iter()
            .flat_map(|cmap| cmap.subtables)
            .filter(|st| st.is_unicode())
            .collect();
        let kern = face
            .tables()
            .kern
            .iter()
            .flat_map(|c| c.subtables)
            .filter(|st| st.horizontal && !st.variable)
            .collect();
        Self { cmap, kern }
    }
}

impl<F> PreParsedSubtables<'_, F> {
    /// Maps a character to a `GlyphId` using pre-parsed unicode cmap subtables.
    #[inline]
    pub fn glyph_index(&self, c: char) -> Option<GlyphId> {
        self.subtables.cmap.iter().find_map(|t| t.glyph_index(c))
    }

    /// Returns kerning for a pair of glyphs using pre-parsed kern subtables.
    #[inline]
    pub fn glyphs_kerning(&self, first: GlyphId, second: GlyphId) -> Option<i16> {
        self.subtables
            .kern
            .iter()
            .find_map(|st| st.glyphs_kerning(first, second))
    }
}

impl<F> AsFaceRef for PreParsedSubtables<'_, F>
where
    F: AsFaceRef,
{
    #[inline]
    fn as_face_ref(&self) -> &ttf_parser::Face<'_> {
        self.face.as_face_ref()
    }
}
