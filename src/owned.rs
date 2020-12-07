#[cfg(not(feature = "std"))]
use alloc::{boxed::Box, vec::Vec};
use core::fmt;

/// An owned version of font [`Face`](struct.Face.html).
pub struct OwnedFace(Box<SelfRefVecFace>);

impl OwnedFace {
    /// Creates an `OwnedFace` from owned data.
    ///
    /// You can set index for font collections. For simple ttf fonts set index to 0.
    ///
    /// # Example
    /// ```
    /// # use owned_ttf_parser::OwnedFace;
    /// # let owned_font_data = include_bytes!("../fonts/font.ttf").to_vec();
    /// let owned_face = OwnedFace::from_vec(owned_font_data, 0).unwrap();
    /// ```
    // Note: not `try_from_vec` to better mimic `ttf_parser::Face::from_data`.
    pub fn from_vec(data: Vec<u8>, index: u32) -> Result<Self, ttf_parser::FaceParsingError> {
        let inner = SelfRefVecFace::try_from_vec(data, index)?;
        Ok(Self(inner))
    }
}

impl fmt::Debug for OwnedFace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "OwnedFace()")
    }
}

impl crate::convert::AsFaceRef for OwnedFace {
    #[inline]
    fn as_face_ref(&self) -> &ttf_parser::Face<'_> {
        &self.0.face
    }
}

impl crate::convert::AsFaceRef for &OwnedFace {
    #[inline]
    fn as_face_ref(&self) -> &ttf_parser::Face<'_> {
        &self.0.face
    }
}

// Face data in a `Vec` with a self-referencing `Face`.
struct SelfRefVecFace {
    _data: Box<[u8]>, // safety: this data must never be mutated or dropped while face lives
    face: ttf_parser::Face<'static>, // safe to copy, but fairly large
}

impl SelfRefVecFace {
    /// Creates an underlying face object from owned data.
    fn try_from_vec(data: Vec<u8>, index: u32) -> Result<Box<Self>, ttf_parser::FaceParsingError> {
        let _data = data.into_boxed_slice();
        let face = unsafe {
            // 'static lifetime is a lie, this data is owned, it has pseudo-self lifetime.
            let slice: &'static [u8] = extend_lifetime(&*_data);
            ttf_parser::Face::from_slice(slice, index)?
        };
        Ok(Box::new(Self { _data, face }))
    }
}

unsafe fn extend_lifetime<'b, T: ?Sized>(r: &'b T) -> &'static T {
    core::mem::transmute::<&'b T, &'static T>(r)
}
