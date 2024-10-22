use huginn::types::Color;
use huginn::utils::CMP_EPSILON;

macro_rules! assert_approx_eq {
    ($x:expr, $y:expr) => {
        assert!(($x - $y).abs() < CMP_EPSILON);
    };
    ($x:expr, $y:expr, $msg:expr) => {
        assert!(($x - $y).abs() < CMP_EPSILON, $msg);
    };
}
macro_rules! assert_approx_eq_with_tolerance {
    ($x:expr, $y:expr, $z:expr) => {
        assert!(($x - $y).abs() < $z);
    };
    ($x:expr, $y:expr, $z:expr, $msg:expr) => {
        assert!(($x - $y).abs() < $z, $msg);
    };
}

#[test]
fn constructor_methods() {
    let blue_rgba = Color::rgba(0.25098, 0.376471, 1.0, 0.501961);
    let blue_html = Color::html("#4060ff80");
    let blue_hex = Color::hex(0x4060ff80);
    let blue_hex64 = Color::hex64(0x4040_6060_ffff_8080);

    let html_invalid = Color::html("invalid");

    let green_rgba = Color::rgba(0.0, 1.0, 0.0, 0.25);
    let green_hsva = Color::hsva(120.0 / 360.0, 1.0, 1.0, 0.25);

    assert!(
        blue_rgba.is_equal_approx(&blue_html),
        "Creation with HTML notation should result in components approximately equal to the default constructor.");
    assert!(
        blue_rgba.is_equal_approx(&blue_hex),
        "Creation with a 32-bit hexadecimal number should result in components approximately equal to the default constructor.");
    assert!(
        blue_rgba.is_equal_approx(&blue_hex64),
        "Creation with a 64-bit hexadecimal number should result in components approximately equal to the default letructor.");
    assert!(
        html_invalid.is_equal_approx(&Color::default()),
        "Creation with invalid HTML notation should result in a Color with the default values."
    );
    assert!(
        green_rgba.is_equal_approx(&green_hsva),
        "Creation with HSV notation should result in components approximately equal to the default letructor.");
}

#[test]
fn operators() {
    let blue = Color::rgb(0.2, 0.2, 1.0);
    let dark_red = Color::rgb(0.3, 0.1, 0.1);

    // Color components may be negative. Also, the alpha component may be greater than 1.0.

    assert!(
        (blue + dark_red).is_equal_approx(&Color::rgba(0.5, 0.3, 1.1, 2.0)),
        "Color addition should behave as expected."
    );
    assert!(
        (blue - dark_red).is_equal_approx(&Color::rgba(-0.1, 0.1, 0.9, 0.0)),
        "Color subtraction should behave as expected."
    );
    assert!(
        (blue * 2).is_equal_approx(&Color::rgba(0.4, 0.4, 2.0, 2.0)),
        "Color multiplication with a scalar should behave as expected."
    );
    assert!(
        (blue / 2).is_equal_approx(&Color::rgba(0.1, 0.1, 0.5, 0.5)),
        "Color division with a scalar should behave as expected."
    );
    assert!(
        (blue * dark_red).is_equal_approx(&Color::rgb(0.06, 0.02, 0.1)),
        "Color multiplication with another Color should behave as expected."
    );
    assert!(
        (blue / dark_red).is_equal_approx(&Color::rgb(0.666667, 2.0, 10.0)),
        "Color division with another Color should behave as expected."
    );
    assert!(
        (-blue).is_equal_approx(&Color::rgba(0.8, 0.8, 0.0, 0.0)),
        "Color negation should behave as expected (affecting the alpha channel, unlike `invert()`).");
}

#[test]
fn reading_methods() {
    let dark_blue = Color::rgba(0.0, 0.0, 0.5, 0.4);

    assert_approx_eq!(
        dark_blue.h(),
        240.0 / 360.0,
        "The returned HSV hue should match the expected value."
    );
    assert_approx_eq!(
        dark_blue.s(),
        1.0,
        "The returned HSV saturation should match the expected value."
    );
    assert_approx_eq!(
        dark_blue.v(),
        0.5,
        "The returned HSV value should match the expected value."
    );
}

#[test]
fn conversion_methods() {
    let cyan = Color::rgb(0.0, 1.0, 1.0);
    let cyan_transparent = Color::rgba(0.0, 1.0, 1.0, 0.0);

    assert_eq!(
        cyan.to_html(),
        "00ffffff",
        "The returned RGB HTML color code should match the expected value."
    );
    assert_eq!(
        cyan_transparent.to_html(),
        "00ffff00",
        "The returned RGBA HTML color code should match the expected value."
    );
    assert_eq!(
        cyan.to_argb32(),
        0xff00ffff,
        "The returned 32-bit RGB number should match the expected value."
    );
    assert_eq!(
        cyan.to_abgr32(),
        0xffffff00,
        "The returned 32-bit BGR number should match the expected value."
    );
    assert_eq!(
        cyan.to_rgba32(),
        0x00ffffff,
        "The returned 32-bit BGR number should match the expected value."
    );
    assert_eq!(
        cyan.to_argb64(),
        0xffff_0000_ffff_ffff,
        "The returned 64-bit RGB number should match the expected value."
    );
    assert_eq!(
        cyan.to_abgr64(),
        0xffff_ffff_ffff_0000,
        "The returned 64-bit BGR number should match the expected value."
    );
    assert_eq!(
        cyan.to_rgba64(),
        0x0000_ffff_ffff_ffff,
        "The returned 64-bit BGR number should match the expected value."
    );
    assert_eq!(
        cyan.to_string(),
        "Color(0, 1, 1, 1)",
        "The string representation should match the expected value."
    );
}

#[test]
fn linear_srgb_conversion() {
    let color = Color::rgba(0.35, 0.5, 0.6, 0.7);
    let color_linear = color.srgb_to_linear();
    let color_srgb = color.linear_to_srgb();

    assert!(
        color_linear.is_equal_approx(&Color::rgba(0.100481, 0.214041, 0.318547, 0.7)),
        "The color converted to linear color space should match the expected value."
    );
    assert!(
        color_srgb.is_equal_approx(&Color::rgba(0.62621, 0.735357, 0.797738, 0.7)),
        "The color converted to sRGB color space should match the expected value."
    );
    assert!(
        color_linear
            .linear_to_srgb()
            .is_equal_approx(&Color::rgba(0.35, 0.5, 0.6, 0.7)),
        "The linear color converted back to sRGB color space should match the expected value."
    );
    assert!(
        color_srgb
            .srgb_to_linear()
            .is_equal_approx(&Color::rgba(0.35, 0.5, 0.6, 0.7)),
        "The sRGB color converted back to linear color space should match the expected value."
    );
}

#[test]
fn named_colors() {
    // Named colors have their names automatically normalized.

    assert!(
        Color::named("red", None).is_equal_approx(&Color::hex(0xFF0000FF)),
        "The named color \"red\" should match the expected value."
    );
    assert!(
        Color::named("white_smoke", None).is_equal_approx(&Color::hex(0xF5F5F5FF)),
        "The named color \"white_smoke\" should match the expected value."
    );
    assert!(
        Color::named("Slate Blue", None).is_equal_approx(&Color::hex(0x6A5ACDFF)),
        "The named color \"Slate Blue\" should match the expected value."
    );
    assert!(
        Color::named("doesn't exist", None).is_equal_approx(&Color::default()),
        "The invalid named color \"doesn't exist\" should result in a Color with the default values.");
}

#[test]
fn validation_methods() {
    assert!(
        Color::html_is_valid("#4080ff"),
        "Valid HTML color (with leading #) should be considered valid."
    );
    assert!(
        Color::html_is_valid("4080ff"),
        "Valid HTML color (without leading #) should be considered valid."
    );
    assert!(
        !Color::html_is_valid("12345"),
        "Invalid HTML color should be considered invalid."
    );
    assert!(
        !Color::html_is_valid("#fuf"),
        "Invalid HTML color should be considered invalid."
    );
}

#[test]
fn manipulation_methods() {
    let blue = Color::rgba(0.0, 0.0, 1.0, 0.4);

    let purple = Color::rgba(0.5, 0.2, 0.5, 0.25);

    let red = Color::rgba(1.0, 0.0, 0.0, 0.2);
    let yellow = Color::rgba(1.0, 1.0, 0.0, 0.8);

    assert!(
        blue.inverted()
            .is_equal_approx(&Color::rgba(1.0, 1.0, 0.0, 0.4)),
        "Inverted color should have its red, green and blue components inverted."
    );
    assert!(
        purple
            .lightened(0.2)
            .is_equal_approx(&Color::rgba(0.6, 0.36, 0.6, 0.25)),
        "Color should be lightened by the expected amount."
    );
    assert!(
        purple
            .darkened(0.2)
            .is_equal_approx(&Color::rgba(0.4, 0.16, 0.4, 0.25)),
        "Color should be darkened by the expected amount."
    );
    assert!(
        red.lerp(&yellow, 0.5)
            .is_equal_approx(&Color::rgba(1.0, 0.5, 0.0, 0.5)),
        "Red interpolated with yellow should be orange (with interpolated alpha)."
    );
}
