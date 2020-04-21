/// Used to perform a cheap conversion to a [`Font`](struct.Font.html) reference.
pub trait AsFontRef {
    /// Convert to a [`Font`](struct.Font.html) reference.
    fn as_font(&self) -> &ttf_parser::Font<'_>;
}

impl AsFontRef for ttf_parser::Font<'_> {
    #[inline]
    fn as_font(&self) -> &ttf_parser::Font<'_> {
        self
    }
}

impl AsFontRef for &ttf_parser::Font<'_> {
    #[inline]
    fn as_font(&self) -> &ttf_parser::Font<'_> {
        self
    }
}
