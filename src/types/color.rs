use crate::utils::{is_equal_approx, FloatExt};
use crate::{float, int};
use auto_ops::{impl_op_ex, impl_op_ex_commutative};
use log::error;
use okhsl::{oklab_to_linear_srgb, Okhsl, Rgb};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::ops::{Neg, Not};

#[derive(Copy, Clone, Debug)]
pub struct Color {
    r: float!(),
    g: float!(),
    b: float!(),
    a: float!(),
}

impl Color {
    // Alice blue color.
    pub const ALICE_BLUE: Color = Color::rgba(0.941176, 0.972549, 1.0, 1.0);

    // Antique white color.
    pub const ANTIQUE_WHITE: Color = Color::rgba(0.980392, 0.921569, 0.843137, 1.0);

    // Aqua color.
    pub const AQUA: Color = Color::rgba(0.0, 1.0, 1.0, 1.0);

    // Aquamarine color.
    pub const AQUAMARINE: Color = Color::rgba(0.498039, 1.0, 0.831373, 1.0);

    // Azure color.
    pub const AZURE: Color = Color::rgba(0.941176, 1.0, 1.0, 1.0);

    // Beige color.
    pub const BEIGE: Color = Color::rgba(0.960784, 0.960784, 0.862745, 1.0);

    // Bisque color.
    pub const BISQUE: Color = Color::rgba(1.0, 0.894118, 0.768627, 1.0);

    // Black color. In Grimm, this is the default value of any color.
    pub const BLACK: Color = Color::rgba(0.0, 0.0, 0.0, 1.0);

    // Blanched almond color.
    pub const BLANCHED_ALMOND: Color = Color::rgba(1.0, 0.921569, 0.803922, 1.0);

    // Blue color.
    pub const BLUE: Color = Color::rgba(0.0, 0.0, 1.0, 1.0);

    // Blue violet color.
    pub const BLUE_VIOLET: Color = Color::rgba(0.541176, 0.168627, 0.886275, 1.0);

    // Brown color.
    pub const BROWN: Color = Color::rgba(0.647059, 0.164706, 0.164706, 1.0);

    // Burlywood color.
    pub const BURLYWOOD: Color = Color::rgba(0.870588, 0.721569, 0.529412, 1.0);

    // Cadet blue color.
    pub const CADET_BLUE: Color = Color::rgba(0.372549, 0.619608, 0.627451, 1.0);

    // Chartreuse color.
    pub const CHARTREUSE: Color = Color::rgba(0.498039, 1.0, 0.0, 1.0);

    // Chocolate color.
    pub const CHOCOLATE: Color = Color::rgba(0.823529, 0.411765, 0.117647, 1.0);

    // Coral color.
    pub const CORAL: Color = Color::rgba(1.0, 0.498039, 0.313726, 1.0);

    // Cornflower blue color.
    pub const CORNFLOWER_BLUE: Color = Color::rgba(0.392157, 0.584314, 0.929412, 1.0);

    // Cornsilk color.
    pub const CORNSILK: Color = Color::rgba(1.0, 0.972549, 0.862745, 1.0);

    // Crimson color.
    pub const CRIMSON: Color = Color::rgba(0.862745, 0.0784314, 0.235294, 1.0);

    // Cyan color.
    pub const CYAN: Color = Color::rgba(0.0, 1.0, 1.0, 1.0);

    // Dark blue color.
    pub const DARK_BLUE: Color = Color::rgba(0.0, 0.0, 0.545098, 1.0);

    // Dark cyan color.
    pub const DARK_CYAN: Color = Color::rgba(0.0, 0.545098, 0.545098, 1.0);

    // Dark goldenrod color.
    pub const DARK_GOLDENROD: Color = Color::rgba(0.721569, 0.52549, 0.0431373, 1.0);

    // Dark gray color.
    pub const DARK_GRAY: Color = Color::rgba(0.662745, 0.662745, 0.662745, 1.0);

    // Dark green color.
    pub const DARK_GREEN: Color = Color::rgba(0.0, 0.392157, 0.0, 1.0);

    // Dark khaki color.
    pub const DARK_KHAKI: Color = Color::rgba(0.741176, 0.717647, 0.419608, 1.0);

    // Dark magenta color.
    pub const DARK_MAGENTA: Color = Color::rgba(0.545098, 0.0, 0.545098, 1.0);

    // Dark olive green color.
    pub const DARK_OLIVE_GREEN: Color = Color::rgba(0.333333, 0.419608, 0.184314, 1.0);

    // Dark orange color.
    pub const DARK_ORANGE: Color = Color::rgba(1.0, 0.54902, 0.0, 1.0);

    // Dark orchid color.
    pub const DARK_ORCHID: Color = Color::rgba(0.6, 0.196078, 0.8, 1.0);

    // Dark red color.
    pub const DARK_RED: Color = Color::rgba(0.545098, 0.0, 0.0, 1.0);

    // Dark salmon color.
    pub const DARK_SALMON: Color = Color::rgba(0.913725, 0.588235, 0.478431, 1.0);

    // Dark sea green color.
    pub const DARK_SEA_GREEN: Color = Color::rgba(0.560784, 0.737255, 0.560784, 1.0);

    // Dark slate blue color.
    pub const DARK_SLATE_BLUE: Color = Color::rgba(0.282353, 0.239216, 0.545098, 1.0);

    // Dark slate gray color.
    pub const DARK_SLATE_GRAY: Color = Color::rgba(0.184314, 0.309804, 0.309804, 1.0);

    // Dark turquoise color.
    pub const DARK_TURQUOISE: Color = Color::rgba(0.0, 0.807843, 0.819608, 1.0);

    // Dark violet color.
    pub const DARK_VIOLET: Color = Color::rgba(0.580392, 0.0, 0.827451, 1.0);

    // Deep pink color.
    pub const DEEP_PINK: Color = Color::rgba(1.0, 0.0784314, 0.576471, 1.0);

    // Deep sky blue color.
    pub const DEEP_SKY_BLUE: Color = Color::rgba(0.0, 0.74902, 1.0, 1.0);

    // Dim gray color.
    pub const DIM_GRAY: Color = Color::rgba(0.411765, 0.411765, 0.411765, 1.0);

    // Dodger blue color.
    pub const DODGER_BLUE: Color = Color::rgba(0.117647, 0.564706, 1.0, 1.0);

    // Firebrick color.
    pub const FIREBRICK: Color = Color::rgba(0.698039, 0.133333, 0.133333, 1.0);

    // Floral white color.
    pub const FLORAL_WHITE: Color = Color::rgba(1.0, 0.980392, 0.941176, 1.0);

    // Forest green color.
    pub const FOREST_GREEN: Color = Color::rgba(0.133333, 0.545098, 0.133333, 1.0);

    // Fuchsia color.
    pub const FUCHSIA: Color = Color::rgba(1.0, 0.0, 1.0, 1.0);

    // Gainsboro color.
    pub const GAINSBORO: Color = Color::rgba(0.862745, 0.862745, 0.862745, 1.0);

    // Ghost white color.
    pub const GHOST_WHITE: Color = Color::rgba(0.972549, 0.972549, 1.0, 1.0);

    // Gold color.
    pub const GOLD: Color = Color::rgba(1.0, 0.843137, 0.0, 1.0);

    // Goldenrod color.
    pub const GOLDENROD: Color = Color::rgba(0.854902, 0.647059, 0.12549, 1.0);

    // Gray color.
    pub const GRAY: Color = Color::rgba(0.745098, 0.745098, 0.745098, 1.0);

    // Green color.
    pub const GREEN: Color = Color::rgba(0.0, 1.0, 0.0, 1.0);

    // Green yellow color.
    pub const GREEN_YELLOW: Color = Color::rgba(0.678431, 1.0, 0.184314, 1.0);

    // Honeydew color.
    pub const HONEYDEW: Color = Color::rgba(0.941176, 1.0, 0.941176, 1.0);

    // Hot pink color.
    pub const HOT_PINK: Color = Color::rgba(1.0, 0.411765, 0.705882, 1.0);

    // Indian red color.
    pub const INDIAN_RED: Color = Color::rgba(0.803922, 0.360784, 0.360784, 1.0);

    // Indigo color.
    pub const INDIGO: Color = Color::rgba(0.294118, 0.0, 0.509804, 1.0);

    // Ivory color.
    pub const IVORY: Color = Color::rgba(1.0, 1.0, 0.941176, 1.0);

    // Khaki color.
    pub const KHAKI: Color = Color::rgba(0.941176, 0.901961, 0.54902, 1.0);

    // Lavender color.
    pub const LAVENDER: Color = Color::rgba(0.901961, 0.901961, 0.980392, 1.0);

    // Lavender blush color.
    pub const LAVENDER_BLUSH: Color = Color::rgba(1.0, 0.941176, 0.960784, 1.0);

    // Lawn green color.
    pub const LAWN_GREEN: Color = Color::rgba(0.486275, 0.988235, 0.0, 1.0);

    // Lemon chiffon color.
    pub const LEMON_CHIFFON: Color = Color::rgba(1.0, 0.980392, 0.803922, 1.0);

    // Light blue color.
    pub const LIGHT_BLUE: Color = Color::rgba(0.678431, 0.847059, 0.901961, 1.0);

    // Light coral color.
    pub const LIGHT_CORAL: Color = Color::rgba(0.941176, 0.501961, 0.501961, 1.0);

    // Light cyan color.
    pub const LIGHT_CYAN: Color = Color::rgba(0.878431, 1.0, 1.0, 1.0);

    // Light goldenrod color.
    pub const LIGHT_GOLDENROD: Color = Color::rgba(0.980392, 0.980392, 0.823529, 1.0);

    // Light gray color.
    pub const LIGHT_GRAY: Color = Color::rgba(0.827451, 0.827451, 0.827451, 1.0);

    // Light green color.
    pub const LIGHT_GREEN: Color = Color::rgba(0.564706, 0.933333, 0.564706, 1.0);

    // Light pink color.
    pub const LIGHT_PINK: Color = Color::rgba(1.0, 0.713726, 0.756863, 1.0);

    // Light salmon color.
    pub const LIGHT_SALMON: Color = Color::rgba(1.0, 0.627451, 0.478431, 1.0);

    // Light sea green color.
    pub const LIGHT_SEA_GREEN: Color = Color::rgba(0.12549, 0.698039, 0.666667, 1.0);

    // Light sky blue color.
    pub const LIGHT_SKY_BLUE: Color = Color::rgba(0.529412, 0.807843, 0.980392, 1.0);

    // Light slate gray color.
    pub const LIGHT_SLATE_GRAY: Color = Color::rgba(0.466667, 0.533333, 0.6, 1.0);

    // Light steel blue color.
    pub const LIGHT_STEEL_BLUE: Color = Color::rgba(0.690196, 0.768627, 0.870588, 1.0);

    // Light yellow color.
    pub const LIGHT_YELLOW: Color = Color::rgba(1.0, 1.0, 0.878431, 1.0);

    // Lime color.
    pub const LIME: Color = Color::rgba(0.0, 1.0, 0.0, 1.0);

    // Lime green color.
    pub const LIME_GREEN: Color = Color::rgba(0.196078, 0.803922, 0.196078, 1.0);

    // Linen color.
    pub const LINEN: Color = Color::rgba(0.980392, 0.941176, 0.901961, 1.0);

    // Magenta color.
    pub const MAGENTA: Color = Color::rgba(1.0, 0.0, 1.0, 1.0);

    // Maroon color.
    pub const MAROON: Color = Color::rgba(0.690196, 0.188235, 0.376471, 1.0);

    // Medium aquamarine color.
    pub const MEDIUM_AQUAMARINE: Color = Color::rgba(0.4, 0.803922, 0.666667, 1.0);

    // Medium blue color.
    pub const MEDIUM_BLUE: Color = Color::rgba(0.0, 0.0, 0.803922, 1.0);

    // Medium orchid color.
    pub const MEDIUM_ORCHID: Color = Color::rgba(0.729412, 0.333333, 0.827451, 1.0);

    // Medium purple color.
    pub const MEDIUM_PURPLE: Color = Color::rgba(0.576471, 0.439216, 0.858824, 1.0);

    // Medium sea green color.
    pub const MEDIUM_SEA_GREEN: Color = Color::rgba(0.235294, 0.701961, 0.443137, 1.0);

    // Medium slate blue color.
    pub const MEDIUM_SLATE_BLUE: Color = Color::rgba(0.482353, 0.407843, 0.933333, 1.0);

    // Medium spring green color.
    pub const MEDIUM_SPRING_GREEN: Color = Color::rgba(0.0, 0.980392, 0.603922, 1.0);

    // Medium turquoise color.
    pub const MEDIUM_TURQUOISE: Color = Color::rgba(0.282353, 0.819608, 0.8, 1.0);

    // Medium violet red color.
    pub const MEDIUM_VIOLET_RED: Color = Color::rgba(0.780392, 0.0823529, 0.521569, 1.0);

    // Midnight blue color.
    pub const MIDNIGHT_BLUE: Color = Color::rgba(0.0980392, 0.0980392, 0.439216, 1.0);

    // Mint cream color.
    pub const MINT_CREAM: Color = Color::rgba(0.960784, 1.0, 0.980392, 1.0);

    // Misty rose color.
    pub const MISTY_ROSE: Color = Color::rgba(1.0, 0.894118, 0.882353, 1.0);

    // Moccasin color.
    pub const MOCCASIN: Color = Color::rgba(1.0, 0.894118, 0.709804, 1.0);

    // Navajo white color.
    pub const NAVAJO_WHITE: Color = Color::rgba(1.0, 0.870588, 0.678431, 1.0);

    // Navy blue color.
    pub const NAVY_BLUE: Color = Color::rgba(0.0, 0.0, 0.501961, 1.0);

    // Old lace color.
    pub const OLD_LACE: Color = Color::rgba(0.992157, 0.960784, 0.901961, 1.0);

    // Olive color.
    pub const OLIVE: Color = Color::rgba(0.501961, 0.501961, 0.0, 1.0);

    // Olive drab color.
    pub const OLIVE_DRAB: Color = Color::rgba(0.419608, 0.556863, 0.137255, 1.0);

    // Orange color.
    pub const ORANGE: Color = Color::rgba(1.0, 0.647059, 0.0, 1.0);

    // Orange red color.
    pub const ORANGE_RED: Color = Color::rgba(1.0, 0.270588, 0.0, 1.0);

    // Orchid color.
    pub const ORCHID: Color = Color::rgba(0.854902, 0.439216, 0.839216, 1.0);

    // Pale goldenrod color.
    pub const PALE_GOLDENROD: Color = Color::rgba(0.933333, 0.909804, 0.666667, 1.0);

    // Pale green color.
    pub const PALE_GREEN: Color = Color::rgba(0.596078, 0.984314, 0.596078, 1.0);

    // Pale turquoise color.
    pub const PALE_TURQUOISE: Color = Color::rgba(0.686275, 0.933333, 0.933333, 1.0);

    // Pale violet red color.
    pub const PALE_VIOLET_RED: Color = Color::rgba(0.858824, 0.439216, 0.576471, 1.0);

    // Papaya whip color.
    pub const PAPAYA_WHIP: Color = Color::rgba(1.0, 0.937255, 0.835294, 1.0);

    // Peach puff color.
    pub const PEACH_PUFF: Color = Color::rgba(1.0, 0.854902, 0.72549, 1.0);

    // Peru color.
    pub const PERU: Color = Color::rgba(0.803922, 0.521569, 0.247059, 1.0);

    // Pink color.
    pub const PINK: Color = Color::rgba(1.0, 0.752941, 0.796078, 1.0);

    // Plum color.
    pub const PLUM: Color = Color::rgba(0.866667, 0.627451, 0.866667, 1.0);

    // Powder blue color.
    pub const POWDER_BLUE: Color = Color::rgba(0.690196, 0.878431, 0.901961, 1.0);

    // Purple color.
    pub const PURPLE: Color = Color::rgba(0.627451, 0.12549, 0.941176, 1.0);

    // Rebecca purple color.
    pub const REBECCA_PURPLE: Color = Color::rgba(0.4, 0.2, 0.6, 1.0);

    // Red color.
    pub const RED: Color = Color::rgba(1.0, 0.0, 0.0, 1.0);

    // Rosy brown color.
    pub const ROSY_BROWN: Color = Color::rgba(0.737255, 0.560784, 0.560784, 1.0);

    // Royal blue color.
    pub const ROYAL_BLUE: Color = Color::rgba(0.254902, 0.411765, 0.882353, 1.0);

    // Saddle brown color.
    pub const SADDLE_BROWN: Color = Color::rgba(0.545098, 0.270588, 0.0745098, 1.0);

    // Salmon color.
    pub const SALMON: Color = Color::rgba(0.980392, 0.501961, 0.447059, 1.0);

    // Sandy brown color.
    pub const SANDY_BROWN: Color = Color::rgba(0.956863, 0.643137, 0.376471, 1.0);

    // Sea green color.
    pub const SEA_GREEN: Color = Color::rgba(0.180392, 0.545098, 0.341176, 1.0);

    // Seashell color.
    pub const SEASHELL: Color = Color::rgba(1.0, 0.960784, 0.933333, 1.0);

    // Sienna color.
    pub const SIENNA: Color = Color::rgba(0.627451, 0.321569, 0.176471, 1.0);

    // Silver color.
    pub const SILVER: Color = Color::rgba(0.752941, 0.752941, 0.752941, 1.0);

    // Sky blue color.
    pub const SKY_BLUE: Color = Color::rgba(0.529412, 0.807843, 0.921569, 1.0);

    // Slate blue color.
    pub const SLATE_BLUE: Color = Color::rgba(0.415686, 0.352941, 0.803922, 1.0);

    // Slate gray color.
    pub const SLATE_GRAY: Color = Color::rgba(0.439216, 0.501961, 0.564706, 1.0);

    // Snow color.
    pub const SNOW: Color = Color::rgba(1.0, 0.980392, 0.980392, 1.0);

    // Spring green color.
    pub const SPRING_GREEN: Color = Color::rgba(0.0, 1.0, 0.498039, 1.0);

    // Steel blue color.
    pub const STEEL_BLUE: Color = Color::rgba(0.27451, 0.509804, 0.705882, 1.0);

    // Tan color.
    pub const TAN: Color = Color::rgba(0.823529, 0.705882, 0.54902, 1.0);

    // Teal color.
    pub const TEAL: Color = Color::rgba(0.0, 0.501961, 0.501961, 1.0);

    // Thistle color.
    pub const THISTLE: Color = Color::rgba(0.847059, 0.74902, 0.847059, 1.0);

    // Tomato color.
    pub const TOMATO: Color = Color::rgba(1.0, 0.388235, 0.278431, 1.0);

    // Transparent color (white with zero alpha).
    pub const TRANSPARENT: Color = Color::rgba(1.0, 1.0, 1.0, 0.0);

    // Turquoise color.
    pub const TURQUOISE: Color = Color::rgba(0.25098, 0.878431, 0.815686, 1.0);

    // Violet color.
    pub const VIOLET: Color = Color::rgba(0.933333, 0.509804, 0.933333, 1.0);

    // Web gray color.
    pub const WEB_GRAY: Color = Color::rgba(0.501961, 0.501961, 0.501961, 1.0);

    // Web green color.
    pub const WEB_GREEN: Color = Color::rgba(0.0, 0.501961, 0.0, 1.0);

    // Web maroon color.
    pub const WEB_MAROON: Color = Color::rgba(0.501961, 0.0, 0.0, 1.0);

    // Web purple color.
    pub const WEB_PURPLE: Color = Color::rgba(0.501961, 0.0, 0.501961, 1.0);

    // Wheat color.
    pub const WHEAT: Color = Color::rgba(0.960784, 0.870588, 0.701961, 1.0);

    // White color.
    pub const WHITE: Color = Color::rgba(1.0, 1.0, 1.0, 1.0);

    // White smoke color.
    pub const WHITE_SMOKE: Color = Color::rgba(0.960784, 0.960784, 0.960784, 1.0);

    // Yellow color.
    pub const YELLOW: Color = Color::rgba(1.0, 1.0, 0.0, 1.0);

    // Yellow green color.
    pub const YELLOW_GREEN: Color = Color::rgba(0.603922, 0.803922, 0.196078, 1.0);

    /// Constructs a **Color** from RGB values, typically between `0.0` and `1.0`. `a` is set to `1.0`.
    pub const fn rgb(r: float!(), g: float!(), b: float!()) -> Self {
        Self::rgba(r, g, b, 1.0)
    }

    /// Constructs a Color from RGBA values, typically between `0.0` and `1.0`.
    pub const fn rgba(r: float!(), g: float!(), b: float!(), a: float!()) -> Self {
        Self { r, g, b, a }
    }

    /// Returns a new color resulting from overlaying this color over the given color. In a painting program, you can imagine it as the `over` color painted over this color (including alpha).
    pub fn blend(&self, over: &Self) -> Self {
        let mut res = Self::default();
        let sa = 1.0 - over.a;
        res.a = self.a * sa + over.a;
        if res.a == 0.0 {
            Self::rgba(0.0, 0.0, 0.0, 0.0)
        } else {
            res.r = (self.r * self.a * sa + over.r * over.a) / res.a;
            res.g = (self.g * self.a * sa + over.g * over.a) / res.a;
            res.b = (self.b * self.a * sa + over.b * over.a) / res.a;
            res
        }
    }

    /// Returns a new color with all components clamped between the components of `min` and `max`, by running `clamp` on each component.
    pub fn clamp(&self, min: &Self, max: &Self) -> Self {
        Self::rgba(
            self.r.clamp(min.r, max.r),
            self.g.clamp(min.g, max.g),
            self.b.clamp(min.b, max.b),
            self.a.clamp(min.a, max.a),
        )
    }

    /// Returns a new color resulting from making this color darker by the specified `amount` (ratio from `0.0` to `1.0`). See also [`Color::lightened`].
    pub fn darkened(&self, amount: float!()) -> Self {
        let mut res = *self;
        res.r = res.r * (1.0 - amount);
        res.g = res.g * (1.0 - amount);
        res.b = res.b * (1.0 - amount);
        res
    }

    /// Constructs a color from an [HSV profile](https://en.wikipedia.org/wiki/HSL_and_HSV). The hue (`h`), saturation (`s`), and value (`v`) are typically between `0.0` and `1.0`.
    pub fn hsv(h: float!(), s: float!(), v: float!()) -> Self {
        Self::hsva(h, s, v, 1.0)
    }

    fn set_hsva(&mut self, h: float!(), s: float!(), v: float!(), a: float!()) {
        self.a = a;

        if s == 0.0 {
            // Achromatic (gray)
            self.r = v;
            self.g = v;
            self.b = v;
            return;
        }

        let mut h = h * 6.0;
        h = h % 6.0;
        let i = h.floor() as int!();

        let f = h - i as float!();
        let p = v * (1.0 - s);
        let q = v * (1.0 - s * f);
        let t = v * (1.0 - s * (1.0 - f));

        match i {
            0 => {
                // Red is the dominant color
                self.r = v;
                self.g = t;
                self.b = p;
            }
            1 => {
                // Green is the dominant color
                self.r = q;
                self.g = v;
                self.b = p;
            }
            2 => {
                // Green is the dominant color
                self.r = p;
                self.g = v;
                self.b = t;
            }
            3 => {
                // Blue is the dominant color
                self.r = p;
                self.g = q;
                self.b = v;
            }
            4 => {
                // Blue is the dominant color
                self.r = t;
                self.g = p;
                self.b = v;
            }
            _ => {
                // (5) Red is the dominant color
                self.r = v;
                self.g = p;
                self.b = q;
            }
        }
    }

    /// Constructs a color from an [HSV profile](https://en.wikipedia.org/wiki/HSL_and_HSV). The hue (`h`), saturation (`s`), and value (`v`) are typically between `0.0` and `1.0`.
    pub fn hsva(h: float!(), s: float!(), v: float!(), a: float!()) -> Self {
        let mut c = Color::default();
        c.set_hsva(h, s, v, a);
        c
    }

    /// Constructs a color from an [OK HSL profile](https://bottosson.github.io/posts/colorpicker/). The hue (`h`), saturation (`s`), and lightness (`l`) are typically between `0.0` and `1.0`.
    pub fn ok_hsl(h: float!(), s: float!(), l: float!()) -> Self {
        Self::ok_hsla(h, s, l, 1.0)
    }

    fn set_ok_hsla(&mut self, h: float!(), s: float!(), l: float!(), a: float!()) {
        let mut hsl = Okhsl::from(Rgb::new(0, 0, 0));
        hsl.h = h as f64;
        hsl.s = s as f32;
        hsl.l = l as f32;
        let rgb = hsl.to_srgb();
        self.r = rgb.r as float!() / 255.0;
        self.g = rgb.g as float!() / 255.0;
        self.b = rgb.b as float!() / 255.0;
        self.a = a;
    }

    fn get_ok_hsla(&self) -> Okhsl {
        Okhsl::from(Rgb::new(self.r8(), self.g8(), self.b8()))
    }

    /// Constructs a color from an [OK HSL profile](https://bottosson.github.io/posts/colorpicker/). The hue (`h`), saturation (`s`), and lightness (`l`) are typically between `0.0` and `1.0`.
    pub fn ok_hsla(h: float!(), s: float!(), l: float!(), a: float!()) -> Self {
        let mut c = Color::default();
        c.set_ok_hsla(h, s, l, a);
        c
    }

    /// Decodes a **Color** from an RGBE9995 format integer.
    pub fn rgbe9995(rgbe: int!()) -> Self {
        let r = (rgbe & 0x1ff) as float!();
        let g = ((rgbe >> 9) & 0x1ff) as float!();
        let b = ((rgbe >> 18) & 0x1ff) as float!();
        let e = (rgbe >> 27) as float!();
        let m = (2.0 as float!()).powf(e - 15.0 - 9.0);

        let rd = r * m;
        let gd = g * m;
        let bd = b * m;

        Self::rgb(rd, gd, bd)
    }

    pub fn named(string: &str, default: Option<&Self>) -> Self {
        let default = if let Some(default) = default {
            default
        } else {
            &Self::default()
        };
        color_name(string.replace(" ", "_").to_uppercase().as_str(), default)
    }

    /// Creates a **Color** from the given string, which can be either an HTML color code or a named color (case-insensitive). Returns `default` if the color cannot be inferred from the string.
    ///
    /// See also [`Color::from`].
    pub fn from_string(string: &str, default: &Self) -> Self {
        if Self::html_is_valid(&string) {
            Self::html(string)
        } else {
            Self::named(string, Some(default))
        }
    }

    /// Returns the light intensity of the color, as a value between `0.0` and `1.0` (inclusive). This is useful when determining light or dark color. Colors with a luminance smaller than `0.5` can be generally considered dark.
    ///
    /// **Note:** `get_luminance` relies on the color being in the linear color space to return an accurate relative luminance value. If the color is in the sRGB color space, use srgb_to_linear to convert it to the linear color space first.
    pub fn get_luminance(&self) -> float!() {
        0.2126 * self.r + 0.7152 * self.g + 0.0722 * self.b
    }

    /// Returns the Color associated with the provided hex integer in 32-bit RGBA format (8 bits per channel). This method is the inverse of to_rgba32.
    ///
    /// **Note:** The int is best visualized with hexadecimal notation (`0x` prefix, making it `0xRRGGBBAA`).
    pub fn hex(hex: u32) -> Self {
        let mut hex = hex;
        let a = (hex & 0xff) as float!() / 255.0;
        hex >>= 8;
        let b = (hex & 0xff) as float!() / 255.0;
        hex >>= 8;
        let g = (hex & 0xff) as float!() / 255.0;
        hex >>= 8;
        let r = (hex & 0xff) as float!() / 255.0;

        Self::rgba(r, g, b, a)
    }

    /// Returns the Color associated with the provided hex integer in 64-bit RGBA format (16 bits per channel). This method is the inverse of to_rgba64.
    ///
    /// **Note:** the int is best visualized with hexadecimal notation (`0x` prefix, making it `0xRRRRGGGGBBBBAAAA`).
    pub fn hex64(hex: u64) -> Self {
        let mut hex = hex;
        let a = (hex & 0xFFFF) as float!() / 65535.0;
        hex >>= 16;
        let b = (hex & 0xFFFF) as float!() / 65535.0;
        hex >>= 16;
        let g = (hex & 0xFFFF) as float!() / 65535.0;
        hex >>= 16;
        let r = (hex & 0xFFFF) as float!() / 65535.0;

        Self::rgba(r, g, b, a)
    }

    /// Returns a new color from `rgba`, an HTML hexadecimal color string. `rgba` is not case-sensitive, and may be prefixed by a hash sign (`#`).
    ///
    /// `rgba` must be a valid three-digit or six-digit hexadecimal color string, and may contain an alpha channel value. If `rgba` does not contain an alpha channel value, an alpha channel value of `1.0` is applied. If `rgba` is invalid, returns an empty color.
    pub fn html(rgba: &str) -> Self {
        let mut color = rgba.to_string();
        if color.len() == 0 {
            return Self::default();
        }
        if color.starts_with('#') {
            color = color.replacen('#', "", 1);
        }

        // If enabled, use 1 hex digit per channel instead of 2.
        // Other sizes aren't in the HTML/CSS spec, but we could add them if desired.
        let is_shorthand = color.len() < 5;

        let alpha = if color.len() == 8 {
            true
        } else if color.len() == 6 {
            false
        } else if color.len() == 4 {
            true
        } else if color.len() == 3 {
            false
        } else {
            error!("Invalid color code: #{}.", color);
            return Self::default();
        };

        let (r, g, b, a) = if is_shorthand {
            (
                <int!()>::from_str_radix(&color[0..1], 16),
                <int!()>::from_str_radix(&color[1..2], 16),
                <int!()>::from_str_radix(&color[2..3], 16),
                if alpha {
                    <int!()>::from_str_radix(&color[3..4], 16)
                } else {
                    Ok(15)
                },
            )
        } else {
            (
                <int!()>::from_str_radix(&color[0..2], 16),
                <int!()>::from_str_radix(&color[2..4], 16),
                <int!()>::from_str_radix(&color[4..6], 16),
                if alpha {
                    <int!()>::from_str_radix(&color[6..8], 16)
                } else {
                    Ok(255)
                },
            )
        };
        match (r, g, b, a) {
            (Ok(r), Ok(g), Ok(b), Ok(a)) => {
                if is_shorthand {
                    Self::rgba(
                        r as float!() / 15.0,
                        g as float!() / 15.0,
                        b as float!() / 15.0,
                        a as float!() / 15.0,
                    )
                } else {
                    Self::rgba(
                        r as float!() / 255.0,
                        g as float!() / 255.0,
                        b as float!() / 255.0,
                        a as float!() / 255.0,
                    )
                }
            }
            _ => {
                error!("Invalid color code: #{}.", color);
                Self::default()
            }
        }
    }

    /// Returns `true` if `color` is a valid HTML hexadecimal color string. The string must be a hexadecimal value (case-insensitive) of either 3, 4, 6 or 8 digits, and may be prefixed by a hash sign (`#`).
    pub fn html_is_valid(color: &str) -> bool {
        let mut color = color.to_string();

        if color.len() == 0 {
            return false;
        }

        if color.starts_with('#') {
            color = color.replacen('#', "", 1);
        }

        let len = color.len();
        // Check if the amount of hex digits is valid.
        if !(len == 3 || len == 4 || len == 6 || len == 8) {
            return false;
        }

        // Check if each hex digit is valid.
        for c in color.chars() {
            if ![
                '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
                'A', 'B', 'C', 'D', 'E', 'F',
            ]
            .contains(&c)
            {
                return false;
            }
        }
        true
    }

    /// Returns the color with its `r`, `g`, and `b` components inverted (`(1 - r, 1 - g, 1 - b, a)`).
    pub fn inverted(&self) -> Self {
        Self::rgba(1.0 - self.r, 1.0 - self.g, 1.0 - self.b, self.a)
    }

    /// Returns `true` if this color and `to` are approximately equal, by running `is_equal_approx` on each component.
    pub fn is_equal_approx(&self, to: &Self) -> bool {
        is_equal_approx(self.r, to.r)
            && is_equal_approx(self.g, to.g)
            && is_equal_approx(self.b, to.b)
            && is_equal_approx(self.a, to.a)
    }

    /// Returns the linear interpolation between this color's components and `to`'s components. The interpolation factor `weight` should be between `0.0` and `1.0` (inclusive).
    pub fn lerp(&self, to: &Self, weight: float!()) -> Self {
        Self::rgba(
            self.r.lerp(to.r, weight),
            self.g.lerp(to.g, weight),
            self.b.lerp(to.b, weight),
            self.a.lerp(to.a, weight),
        )
    }

    /// Returns a new color resulting from making this color lighter by the specified `amount`, which should be a ratio from `0.0` to `1.0`. See also [`Color::darkened`].
    pub fn lightened(&self, amount: float!()) -> Self {
        let mut res = *self;
        res.r = res.r + (1.0 - res.r) * amount;
        res.g = res.g + (1.0 - res.g) * amount;
        res.b = res.b + (1.0 - res.b) * amount;
        res
    }

    /// Returns the color converted to the [sRGB](https://en.wikipedia.org/wiki/SRGB) color space. This method assumes the original color is in the linear color space. See also [`Color::srgb_to_linear`] which performs the opposite operation.
    pub fn linear_to_srgb(&self) -> Self {
        Self::rgba(
            if self.r < 0.0031308 {
                12.92 * self.r
            } else {
                (1.0 + 0.055) * self.r.powf(1.0 / 2.4) - 0.055
            },
            if self.g < 0.0031308 {
                12.92 * self.g
            } else {
                (1.0 + 0.055) * self.g.powf(1.0 / 2.4) - 0.055
            },
            if self.b < 0.0031308 {
                12.92 * self.b
            } else {
                (1.0 + 0.055) * self.b.powf(1.0 / 2.4) - 0.055
            },
            self.a,
        )
    }

    /// Returns the color converted to the linear color space. This method assumes the original color already is in the sRGB color space. See also [`Color::linear_to_srgb`] which performs the opposite operation.
    pub fn srgb_to_linear(&self) -> Self {
        Self::rgba(
            if self.r < 0.04045 {
                self.r * (1.0 / 12.92)
            } else {
                ((self.r + 0.055) * (1.0 / (1.0 + 0.055))).powf(2.4)
            },
            if self.g < 0.04045 {
                self.g * (1.0 / 12.92)
            } else {
                ((self.g + 0.055) * (1.0 / (1.0 + 0.055))).powf(2.4)
            },
            if self.b < 0.04045 {
                self.b * (1.0 / 12.92)
            } else {
                ((self.b + 0.055) * (1.0 / (1.0 + 0.055))).powf(2.4)
            },
            self.a,
        )
    }

    /// Returns the color converted to a 32-bit integer in ABGR format (each component is 8 bits). ABGR is the reversed version of the default RGBA format.
    pub fn to_abgr32(&self) -> u32 {
        let mut c = (self.a * 255.0).round() as u32;
        c <<= 8;
        c |= (self.b * 255.0).round() as u32;
        c <<= 8;
        c |= (self.g * 255.0).round() as u32;
        c <<= 8;
        c |= (self.r * 255.0).round() as u32;

        c
    }

    pub fn to_abgr64(&self) -> u64 {
        let mut c = (self.a * 65535.0).round() as u64;
        c <<= 16;
        c |= (self.b * 65535.0).round() as u64;
        c <<= 16;
        c |= (self.g * 65535.0).round() as u64;
        c <<= 16;
        c |= (self.r * 65535.0).round() as u64;

        c
    }

    /// Returns the color converted to a 32-bit integer in ARGB format (each component is 8 bits). ARGB is more compatible with DirectX.
    pub fn to_argb32(&self) -> u32 {
        let mut c = (self.a * 255.0).round() as u32;
        c <<= 8;
        c |= (self.r * 255.0).round() as u32;
        c <<= 8;
        c |= (self.g * 255.0).round() as u32;
        c <<= 8;
        c |= (self.b * 255.0).round() as u32;

        c
    }

    /// Returns the color converted to a 64-bit integer in ARGB format (each component is 16 bits). ARGB is more compatible with DirectX.
    pub fn to_argb64(&self) -> u64 {
        let mut c = (self.a * 65535.0).round() as u64;
        c <<= 16;
        c |= (self.r * 65535.0).round() as u64;
        c <<= 16;
        c |= (self.g * 65535.0).round() as u64;
        c <<= 16;
        c |= (self.b * 65535.0).round() as u64;

        c
    }

    /// Returns the color converted to an HTML hexadecimal color String in RGBA format, without the hash (`#`) prefix.
    pub fn to_html(&self) -> String {
        format!(
            "{:0>2x}{:0>2x}{:0>2x}{:0>2x}",
            (self.r * 255.0).round() as u8,
            (self.g * 255.0).round() as u8,
            (self.b * 255.0).round() as u8,
            (self.a * 255.0).round() as u8
        )
    }

    /// Returns the color converted to an HTML hexadecimal color String in RGB format, without the hash (`#`) prefix.
    pub fn to_html_without_alpha(&self) -> String {
        format!(
            "{:0>2x}{:0>2x}{:0>2x}",
            (self.r * 255.0).round() as u8,
            (self.g * 255.0).round() as u8,
            (self.b * 255.0).round() as u8
        )
    }

    /// Returns the color converted to a 32-bit integer in RGBA format (each component is 8 bits). RGBA is Grimm's default format. This method is the inverse of [`Color::hex`].
    pub fn to_rgba32(&self) -> u32 {
        let mut c = (self.r * 255.0).round() as u32;
        c <<= 8;
        c |= (self.g * 255.0).round() as u32;
        c <<= 8;
        c |= (self.b * 255.0).round() as u32;
        c <<= 8;
        c |= (self.a * 255.0).round() as u32;

        c
    }

    /// Returns the color converted to a 64-bit integer in RGBA format (each component is 16 bits). RGBA is Grimm's default format. This method is the inverse of [`Color::hex64`].
    pub fn to_rgba64(&self) -> u64 {
        let mut c = (self.r * 65535.0).round() as u64;
        c <<= 16;
        c |= (self.g * 65535.0).round() as u64;
        c <<= 16;
        c |= (self.b * 65535.0).round() as u64;
        c <<= 16;
        c |= (self.a * 65535.0).round() as u64;

        c
    }

    pub const fn a(&self) -> float!() {
        self.a
    }
    pub fn set_a(&mut self, a: float!()) {
        self.a = a;
    }

    pub fn a8(&self) -> u8 {
        (self.a * 255.0).round() as u8
    }

    pub fn set_a8(&mut self, a8: u8) {
        self.a = a8 as float!() / 255.0
    }

    pub const fn r(&self) -> float!() {
        self.r
    }
    pub fn set_r(&mut self, r: float!()) {
        self.r = r;
    }

    pub fn r8(&self) -> u8 {
        (self.r * 255.0).round() as u8
    }

    pub fn set_r8(&mut self, r8: u8) {
        self.r = r8 as float!() / 255.0
    }

    pub const fn g(&self) -> float!() {
        self.g
    }
    pub fn set_g(&mut self, g: float!()) {
        self.g = g;
    }

    pub fn g8(&self) -> u8 {
        (self.g * 255.0).round() as u8
    }

    pub fn set_g8(&mut self, g8: u8) {
        self.g = g8 as float!() / 255.0
    }

    pub const fn b(&self) -> float!() {
        self.b
    }
    pub fn set_b(&mut self, b: float!()) {
        self.b = b;
    }

    pub fn b8(&self) -> u8 {
        (self.b * 255.0).round() as u8
    }

    pub fn set_b8(&mut self, b8: u8) {
        self.b = b8 as float!() / 255.0
    }

    pub fn h(&self) -> float!() {
        let min = self.r.min(self.g).min(self.b);
        let max = self.r.max(self.g).max(self.b);

        let delta = max - min;

        if delta == 0.0 {
            return 0.0;
        }

        let mut h = if self.r == max {
            (self.g - self.b) / delta // between yellow & magenta
        } else if self.g == max {
            2.0 + (self.b - self.r) / delta // between cyan & yellow
        } else {
            4.0 + (self.r - self.g) / delta // between magenta & cyan
        };

        h /= 6.0;
        if h < 0.0 {
            h += 1.0;
        }

        h
    }
    pub fn set_h(&mut self, h: float!()) {
        let c = Self::hsva(h, self.s(), self.v(), self.a());
        self.r = c.r;
        self.g = c.g;
        self.b = c.g;
        self.a = c.a;
    }

    pub fn s(&self) -> float!() {
        let min = self.r.min(self.g).min(self.b);
        let max = self.r.max(self.g).max(self.b);

        let delta = max - min;

        if max != 0.0 {
            delta / max
        } else {
            0.0
        }
    }

    pub fn set_s(&mut self, s: float!()) {
        let c = Self::hsva(self.h(), s, self.v(), self.a());
        self.r = c.r;
        self.g = c.g;
        self.b = c.g;
        self.a = c.a;
    }

    pub fn l(&self) -> float!() {
        self.get_ok_hsla().l as float!()
    }

    pub fn set_l(&mut self, l: float!()) {
        let c = Self::ok_hsla(self.h(), self.s(), l, self.a());
        self.r = c.r;
        self.g = c.g;
        self.b = c.g;
        self.a = c.a;
    }

    pub fn v(&self) -> float!() {
        self.r.max(self.g).max(self.b)
    }

    pub fn set_v(&mut self, v: float!()) {
        let c = Self::hsva(self.h(), self.s(), v, self.a());
        self.r = c.r;
        self.g = c.g;
        self.b = c.g;
        self.a = c.a;
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r && self.g == other.g && self.b == other.b && self.a == other.a
    }
}

impl_op_ex!(*|a: &Color, b: &Color| -> Color {
    Color::rgba(a.r * b.r, a.g * b.g, a.b * b.b, a.a * b.a)
});

impl_op_ex!(*= |a: &mut Color, b: &Color| {
    a.r = a.r * b.r;
    a.g = a.g * b.g;
    a.b = a.b * b.b;
    a.a = a.a * b.a;
});

impl_op_ex_commutative!(*|a: &Color, b: &float!()| -> Color {
    Color::rgba(a.r * b, a.g * b, a.b * b, a.a * b)
});

impl_op_ex!(*= |a: &mut Color, b: &float!()| {
    a.r = a.r * b;
    a.g = a.g * b;
    a.b = a.b * b;
    a.a = a.a * b;
});

impl_op_ex_commutative!(*|a: &Color, b: int!()| -> Color {
    Color::rgba(
        a.r * b as float!(),
        a.g * b as float!(),
        a.b * b as float!(),
        a.a * b as float!(),
    )
});

impl_op_ex!(*= |a: &mut Color, b: int!()| {
    a.r = a.r * b as float!();
    a.g = a.g * b as float!();
    a.b = a.b * b as float!();
    a.a = a.a * b as float!();
});

impl_op_ex!(+ |a: &Color, b: &Color| -> Color {
    Color::rgba(
        a.r + b.r,
        a.g + b.g,
        a.b + b.b,
        a.a + b.a,
    )
});

impl_op_ex!(+= |a: &mut Color, b: &Color|{
    a.r = a.r + b.r;
    a.g = a.g + b.g;
    a.b = a.b + b.b;
    a.a = a.a + b.a;
});

impl_op_ex!(-|a: &Color, b: &Color| -> Color {
    Color::rgba(a.r - b.r, a.g - b.g, a.b - b.b, a.a - b.a)
});

impl_op_ex!(-= |a: &mut Color, b: &Color| {
    a.r = a.r - b.r;
    a.g = a.g - b.g;
    a.b = a.b - b.b;
    a.a = a.a - b.a;
});

impl_op_ex!(/ |a: &Color, b: &Color| -> Color {
    Color::rgba(
        a.r / b.r,
        a.g / b.g,
        a.b / b.b,
        a.a / b.a,
    )
});

impl_op_ex!(/= |a: &mut Color, b: &Color| {
    a.r = a.r / b.r;
    a.g = a.g / b.g;
    a.b = a.b / b.b;
    a.a = a.a / b.a;
});

impl_op_ex_commutative!(/ |a: &Color, b: &float!()| -> Color {
    Color::rgba(
        a.r / b,
        a.g / b,
        a.b / b,
        a.a / b,
    )
});

impl_op_ex!(/= |a: &mut Color, b: &float!()| {
    a.r = a.r / b;
    a.g = a.g / b;
    a.b = a.b / b;
    a.a = a.a / b;
});

impl_op_ex_commutative!(/ |a: &Color, b: int!()| -> Color {
    Color::rgba(
        a.r / b as float!(),
        a.g / b as float!(),
        a.b / b as float!(),
        a.a / b as float!(),
    )
});

impl_op_ex!(/= |a: &mut Color, b: int!()| {
    a.r = a.r / b as float!();
    a.g = a.g / b as float!();
    a.b = a.b / b as float!();
    a.a = a.a / b as float!();
});

impl Default for Color {
    /// Constructs a default **Color** from opaque black. This is the same as [`Color::BLACK`].
    fn default() -> Self {
        Self {
            a: 1.0,
            b: 0.0,
            g: 0.0,
            r: 0.0,
        }
    }
}

impl From<(Color, float!())> for Color {
    /// Constructs a **Color** from the existing color, with `a` set to the given `alpha` value.
    fn from(value: (Color, float!())) -> Self {
        Color::rgba(value.0.r, value.0.g, value.0.b, value.1)
    }
}

impl From<&str> for Color {
    /// Constructs a **Color** either from an HTML color code or from a standardized color name. The supported color names are the same as the constants.
    fn from(value: &str) -> Self {
        Color::html(value)
    }
}

impl From<(&str, float!())> for Color {
    /// Constructs a **Color** either from an HTML color code or from a standardized color name, with `alpha` on the range of `0.0` to `1.0`. The supported color names are the same as the constants.
    fn from(value: (&str, float!())) -> Self {
        let mut c = Color::html(value.0);
        c.set_a(value.1);
        c
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Color({}, {}, {}, {})",
            self.r, self.g, self.b, self.a
        ))
    }
}

impl Not for Color {
    type Output = bool;
    fn not(self) -> Self::Output {
        self.r == 0.0 && self.g == 0.0 && self.b == 0.0 && self.a == 1.0
    }
}
impl Neg for Color {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::rgba(1.0 - self.r, 1.0 - self.g, 1.0 - self.b, 1.0 - self.a)
    }
}

fn color_name(name: &str, default: &Color) -> Color {
    match name {
        "ALICE_BLUE" => Color::ALICE_BLUE,
        "ANTIQUE_WHITE" => Color::ANTIQUE_WHITE,
        "AQUA" => Color::AQUA,
        "AQUAMARINE" => Color::AQUAMARINE,
        "AZURE" => Color::AZURE,
        "BEIGE" => Color::BEIGE,
        "BISQUE" => Color::BISQUE,
        "BLACK" => Color::BLACK,
        "BLANCHED_ALMOND" => Color::BLANCHED_ALMOND,
        "BLUE" => Color::BLUE,
        "BLUE_VIOLET" => Color::BLUE_VIOLET,
        "BROWN" => Color::BROWN,
        "BURLYWOOD" => Color::BURLYWOOD,
        "CADET_BLUE" => Color::CADET_BLUE,
        "CHARTREUSE" => Color::CHARTREUSE,
        "CHOCOLATE" => Color::CHOCOLATE,
        "CORAL" => Color::CORAL,
        "CORNFLOWER_BLUE" => Color::CORNFLOWER_BLUE,
        "CORNSILK" => Color::CORNSILK,
        "CRIMSON" => Color::CRIMSON,
        "CYAN" => Color::CYAN,
        "DARK_BLUE" => Color::DARK_BLUE,
        "DARK_CYAN" => Color::DARK_CYAN,
        "DARK_GOLDENROD" => Color::DARK_GOLDENROD,
        "DARK_GRAY" => Color::DARK_GRAY,
        "DARK_GREEN" => Color::DARK_GREEN,
        "DARK_KHAKI" => Color::DARK_KHAKI,
        "DARK_MAGENTA" => Color::DARK_MAGENTA,
        "DARK_OLIVE_GREEN" => Color::DARK_OLIVE_GREEN,
        "DARK_ORANGE" => Color::DARK_ORANGE,
        "DARK_ORCHID" => Color::DARK_ORCHID,
        "DARK_RED" => Color::DARK_RED,
        "DARK_SALMON" => Color::DARK_SALMON,
        "DARK_SEA_GREEN" => Color::DARK_SEA_GREEN,
        "DARK_SLATE_BLUE" => Color::DARK_SLATE_BLUE,
        "DARK_SLATE_GRAY" => Color::DARK_SLATE_GRAY,
        "DARK_TURQUOISE" => Color::DARK_TURQUOISE,
        "DARK_VIOLET" => Color::DARK_VIOLET,
        "DEEP_PINK" => Color::DEEP_PINK,
        "DEEP_SKY_BLUE" => Color::DEEP_SKY_BLUE,
        "DIM_GRAY" => Color::DIM_GRAY,
        "DODGER_BLUE" => Color::DODGER_BLUE,
        "FIREBRICK" => Color::FIREBRICK,
        "FLORAL_WHITE" => Color::FLORAL_WHITE,
        "FOREST_GREEN" => Color::FOREST_GREEN,
        "FUCHSIA" => Color::FUCHSIA,
        "GAINSBORO" => Color::GAINSBORO,
        "GHOST_WHITE" => Color::GHOST_WHITE,
        "GOLD" => Color::GOLD,
        "GOLDENROD" => Color::GOLDENROD,
        "GRAY" => Color::GRAY,
        "GREEN" => Color::GREEN,
        "GREEN_YELLOW" => Color::GREEN_YELLOW,
        "HONEYDEW" => Color::HONEYDEW,
        "HOT_PINK" => Color::HOT_PINK,
        "INDIAN_RED" => Color::INDIAN_RED,
        "INDIGO" => Color::INDIGO,
        "IVORY" => Color::IVORY,
        "KHAKI" => Color::KHAKI,
        "LAVENDER" => Color::LAVENDER,
        "LAVENDER_BLUSH" => Color::LAVENDER_BLUSH,
        "LAWN_GREEN" => Color::LAWN_GREEN,
        "LEMON_CHIFFON" => Color::LEMON_CHIFFON,
        "LIGHT_BLUE" => Color::LIGHT_BLUE,
        "LIGHT_CORAL" => Color::LIGHT_CORAL,
        "LIGHT_CYAN" => Color::LIGHT_CYAN,
        "LIGHT_GOLDENROD" => Color::LIGHT_GOLDENROD,
        "LIGHT_GRAY" => Color::LIGHT_GRAY,
        "LIGHT_GREEN" => Color::LIGHT_GREEN,
        "LIGHT_PINK" => Color::LIGHT_PINK,
        "LIGHT_SALMON" => Color::LIGHT_SALMON,
        "LIGHT_SEA_GREEN" => Color::LIGHT_SEA_GREEN,
        "LIGHT_SKY_BLUE" => Color::LIGHT_SKY_BLUE,
        "LIGHT_SLATE_GRAY" => Color::LIGHT_SLATE_GRAY,
        "LIGHT_STEEL_BLUE" => Color::LIGHT_STEEL_BLUE,
        "LIGHT_YELLOW" => Color::LIGHT_YELLOW,
        "LIME" => Color::LIME,
        "LIME_GREEN" => Color::LIME_GREEN,
        "LINEN" => Color::LINEN,
        "MAGENTA" => Color::MAGENTA,
        "MAROON" => Color::MAROON,
        "MEDIUM_AQUAMARINE" => Color::MEDIUM_AQUAMARINE,
        "MEDIUM_BLUE" => Color::MEDIUM_BLUE,
        "MEDIUM_ORCHID" => Color::MEDIUM_ORCHID,
        "MEDIUM_PURPLE" => Color::MEDIUM_PURPLE,
        "MEDIUM_SEA_GREEN" => Color::MEDIUM_SEA_GREEN,
        "MEDIUM_SLATE_BLUE" => Color::MEDIUM_SLATE_BLUE,
        "MEDIUM_SPRING_GREEN" => Color::MEDIUM_SPRING_GREEN,
        "MEDIUM_TURQUOISE" => Color::MEDIUM_TURQUOISE,
        "MEDIUM_VIOLET_RED" => Color::MEDIUM_VIOLET_RED,
        "MIDNIGHT_BLUE" => Color::MIDNIGHT_BLUE,
        "MINT_CREAM" => Color::MINT_CREAM,
        "MISTY_ROSE" => Color::MISTY_ROSE,
        "MOCCASIN" => Color::MOCCASIN,
        "NAVAJO_WHITE" => Color::NAVAJO_WHITE,
        "NAVY_BLUE" => Color::NAVY_BLUE,
        "OLD_LACE" => Color::OLD_LACE,
        "OLIVE" => Color::OLIVE,
        "OLIVE_DRAB" => Color::OLIVE_DRAB,
        "ORANGE" => Color::ORANGE,
        "ORANGE_RED" => Color::ORANGE_RED,
        "ORCHID" => Color::ORCHID,
        "PALE_GOLDENROD" => Color::PALE_GOLDENROD,
        "PALE_GREEN" => Color::PALE_GREEN,
        "PALE_TURQUOISE" => Color::PALE_TURQUOISE,
        "PALE_VIOLET_RED" => Color::PALE_VIOLET_RED,
        "PAPAYA_WHIP" => Color::PAPAYA_WHIP,
        "PEACH_PUFF" => Color::PEACH_PUFF,
        "PERU" => Color::PERU,
        "PINK" => Color::PINK,
        "PLUM" => Color::PLUM,
        "POWDER_BLUE" => Color::POWDER_BLUE,
        "PURPLE" => Color::PURPLE,
        "REBECCA_PURPLE" => Color::REBECCA_PURPLE,
        "RED" => Color::RED,
        "ROSY_BROWN" => Color::ROSY_BROWN,
        "ROYAL_BLUE" => Color::ROYAL_BLUE,
        "SADDLE_BROWN" => Color::SADDLE_BROWN,
        "SALMON" => Color::SALMON,
        "SANDY_BROWN" => Color::SANDY_BROWN,
        "SEA_GREEN" => Color::SEA_GREEN,
        "SEASHELL" => Color::SEASHELL,
        "SIENNA" => Color::SIENNA,
        "SILVER" => Color::SILVER,
        "SKY_BLUE" => Color::SKY_BLUE,
        "SLATE_BLUE" => Color::SLATE_BLUE,
        "SLATE_GRAY" => Color::SLATE_GRAY,
        "SNOW" => Color::SNOW,
        "SPRING_GREEN" => Color::SPRING_GREEN,
        "STEEL_BLUE" => Color::STEEL_BLUE,
        "TAN" => Color::TAN,
        "TEAL" => Color::TEAL,
        "THISTLE" => Color::THISTLE,
        "TOMATO" => Color::TOMATO,
        "TRANSPARENT" => Color::TRANSPARENT,
        "TURQUOISE" => Color::TURQUOISE,
        "VIOLET" => Color::VIOLET,
        "WEB_GRAY" => Color::WEB_GRAY,
        "WEB_GREEN" => Color::WEB_GREEN,
        "WEB_MAROON" => Color::WEB_MAROON,
        "WEB_PURPLE" => Color::WEB_PURPLE,
        "WHEAT" => Color::WHEAT,
        "WHITE" => Color::WHITE,
        "WHITE_SMOKE" => Color::WHITE_SMOKE,
        "YELLOW" => Color::YELLOW,
        "YELLOW_GREEN" => Color::YELLOW_GREEN,
        _ => *default,
    }
}
