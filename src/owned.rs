#[cfg(not(feature = "std"))]
use alloc::{boxed::Box, vec::Vec};
use core::{fmt, marker::PhantomPinned, pin::Pin, slice};

/// An owned version of font [`Face`](struct.Face.html).
pub struct OwnedFace(Pin<Box<SelfRefVecFace>>);

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
    pub fn from_vec(data: Vec<u8>, index: u32) -> Option<Self> {
        let inner = SelfRefVecFace::try_from_vec(data, index)?;
        Some(Self(inner))
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
        self.0.inner_ref()
    }
}

impl crate::convert::AsFaceRef for &OwnedFace {
    #[inline]
    fn as_face_ref(&self) -> &ttf_parser::Face<'_> {
        self.0.inner_ref()
    }
}

// Face data in a `Vec` with a self-referencing `Face`.
struct SelfRefVecFace {
    data: Vec<u8>,
    face: Option<ttf_parser::Face<'static>>,
    _pin: PhantomPinned,
}

impl SelfRefVecFace {
    /// Creates an underlying face object from owned data.
    fn try_from_vec(data: Vec<u8>, index: u32) -> Option<Pin<Box<Self>>> {
        let face = Self {
            data,
            face: None,
            _pin: PhantomPinned,
        };
        let mut b = Box::pin(face);
        unsafe {
            // 'static lifetime is a lie, this data is owned, it has pseudo-self lifetime.
            let slice: &'static [u8] = slice::from_raw_parts(b.data.as_ptr(), b.data.len());
            let mut_ref: Pin<&mut Self> = Pin::as_mut(&mut b);
            let mut_inner = mut_ref.get_unchecked_mut();
            mut_inner.face = Some(ttf_parser::Face::from_slice(slice, index)?);
        }
        Some(b)
    }

    // Must not leak the fake 'static lifetime that we lied about earlier to the
    // compiler. Since the lifetime 'a will not outlive our owned data it's
    // safe to provide Face<'a>
    #[inline]
    fn inner_ref<'a>(self: &'a Pin<Box<Self>>) -> &'a ttf_parser::Face<'a> {
        match self.face.as_ref() {
            Some(f) => f,
            None => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}
