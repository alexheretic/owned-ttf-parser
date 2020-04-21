use owned_ttf_parser::{AsFontRef, OwnedFont};

const FONT: &[u8] = include_bytes!("../fonts/font.ttf");

#[test]
fn move_and_use() {
    let owned_data = FONT.to_vec();
    let pin_font = OwnedFont::from_vec(owned_data, 0).unwrap();

    let ascent = pin_font.as_font().ascender();

    // force a move
    let moved = Box::new(pin_font);

    assert_eq!(moved.as_font().ascender(), ascent);
}
