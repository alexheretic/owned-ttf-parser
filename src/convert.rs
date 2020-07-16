/// Used to perform a cheap conversion to a [`Face`](struct.Face.html) reference.
pub trait AsFaceRef {
    /// Convert to a [`Face`](struct.Face.html) reference.
    fn as_face_ref(&self) -> &ttf_parser::Face<'_>;
}

impl AsFaceRef for ttf_parser::Face<'_> {
    #[inline]
    fn as_face_ref(&self) -> &ttf_parser::Face<'_> {
        self
    }
}

impl AsFaceRef for &ttf_parser::Face<'_> {
    #[inline]
    fn as_face_ref(&self) -> &ttf_parser::Face<'_> {
        self
    }
}
