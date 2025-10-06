use kolor::details::color::{ColorError, RgbPrimaries, WhitePoint};

#[test]
fn detect_exact_bt709_primaries() {
    let primaries = RgbPrimaries::from_rgb_xy([0.64, 0.33], [0.30, 0.60], [0.15, 0.06]);
    assert!(matches!(primaries, RgbPrimaries::Bt709));
}

#[test]
fn detect_bt709_within_tolerance() {
    // Values within 1e-4 tolerance should still be detected as BT.709
    // Using smaller differences to stay within tolerance
    let primaries =
        RgbPrimaries::from_rgb_xy([0.64005, 0.33005], [0.30005, 0.60005], [0.15005, 0.06005]);
    assert!(matches!(primaries, RgbPrimaries::Bt709));
}

#[test]
fn custom_primaries_for_unknown_values() {
    let primaries = RgbPrimaries::from_rgb_xy([0.7, 0.3], [0.2, 0.7], [0.1, 0.1]);
    assert!(matches!(primaries, RgbPrimaries::Custom(_)));
}

#[test]
fn canonicalize_matching_custom_primaries() {
    // Create a Custom variant with BT.709 values
    let mut primaries = RgbPrimaries::Custom([[0.64, 0.33], [0.30, 0.60], [0.15, 0.06]]);

    let result = primaries.canonicalize();
    assert!(result.is_ok());
    assert!(matches!(primaries, RgbPrimaries::Bt709));
}

#[test]
fn canonicalize_non_matching_custom_primaries() {
    // Create a Custom variant with non-standard values
    let mut primaries = RgbPrimaries::Custom([[0.7, 0.3], [0.2, 0.7], [0.1, 0.1]]);

    let result = primaries.canonicalize();
    assert!(matches!(result, Err(ColorError::CanonicalizationFailed)));
    assert!(matches!(primaries, RgbPrimaries::Custom(_)));
}

#[test]
fn canonicalize_already_canonical() {
    let mut primaries = RgbPrimaries::Bt709;

    let result = primaries.canonicalize();
    assert!(result.is_ok());
    assert!(matches!(primaries, RgbPrimaries::Bt709));
}

#[test]
fn detect_exact_d65_white_point() {
    // D65 standard values: x=0.3127, y=0.3290
    // This converts to XYZ [0.95047, 1.0, 1.08883]
    let wp = WhitePoint::from_xy(0.31271, 0.32902);
    assert!(matches!(wp, WhitePoint::D65));
}

#[test]
fn detect_d65_within_tolerance() {
    // Slightly off D65 values should still be detected
    // Keep the difference small to stay within 1e-4 tolerance in XYZ space
    let wp = WhitePoint::from_xy(0.312715, 0.329025);
    assert!(matches!(wp, WhitePoint::D65));
}

#[test]
fn custom_white_point_for_unknown_values() {
    let wp = WhitePoint::from_xy(0.4, 0.4);
    assert!(matches!(wp, WhitePoint::Custom(_)));
}

#[test]
fn canonicalize_matching_custom_white_point() {
    // Create a Custom variant with D65 XYZ values
    let mut wp = WhitePoint::Custom([0.95047, 1.0, 1.08883]);

    let result = wp.canonicalize();
    assert!(result.is_ok());
    assert!(matches!(wp, WhitePoint::D65));
}

#[test]
fn canonicalize_non_matching_custom_white_point() {
    let mut wp = WhitePoint::Custom([0.9, 1.0, 1.1]);

    let result = wp.canonicalize();
    assert!(matches!(result, Err(ColorError::CanonicalizationFailed)));
    assert!(matches!(wp, WhitePoint::Custom(_)));
}

#[test]
fn aces_ap0_detection() {
    let primaries =
        RgbPrimaries::from_rgb_xy([0.7347, 0.2653], [0.0000, 1.0000], [0.0001, -0.0770]);
    assert!(matches!(primaries, RgbPrimaries::AcesAp0));
}

#[test]
fn aces_ap1_detection() {
    let primaries = RgbPrimaries::from_rgb_xy([0.713, 0.293], [0.165, 0.830], [0.128, 0.044]);
    assert!(matches!(primaries, RgbPrimaries::AcesAp1));
}
