//! Tests for the FaceMut trait.
use owned_ttf_parser::{AsFaceRef, FaceMut, NormalizedCoordinate, OwnedFace, VariationAxis};

const VFONT: &[u8] = include_bytes!("../fonts/Cantarell-VF.otf");

#[test]
fn set_variation() {
    let mut face = OwnedFace::from_vec(VFONT.to_vec(), 0).unwrap();
    let axis = face.as_face_ref().variation_axes().get(0).unwrap();
    let coord = face.as_face_ref().variation_coordinates()[0];
    let denorm_coord = denormalize_coord(axis, coord);
    assert!(
        (denorm_coord - axis.def_value).abs() < f32::EPSILON,
        "Unexpected coord value `{denorm_coord:.1}`"
    );

    // after setting variation on the owned face it should change
    let new_value = axis.max_value;
    assert_ne!(axis.def_value, new_value);
    face.set_variation(axis.tag, new_value)
        .expect("Should be some");

    let coord = face.as_face_ref().variation_coordinates()[0];
    let denorm_coord = denormalize_coord(axis, coord);
    assert!(
        (denorm_coord - new_value).abs() < f32::EPSILON,
        "Unexpected coord value `{denorm_coord:.1}`"
    );
}

fn denormalize_coord(axis: VariationAxis, coord: NormalizedCoordinate) -> f32 {
    let coord = (coord.get() as f32) / 16384.0;
    if coord < 0.0 {
        (coord * (axis.def_value - axis.min_value)) + axis.def_value
    } else {
        (coord * (axis.max_value - axis.def_value)) + axis.def_value
    }
}
