use std::fmt::{Display, Formatter};
use std::ops::Not;
use crate::int;
use crate::types::{Rect2, Side};
use crate::types::vectors::{Vector2, Vector2i};

/// A 2D axis-aligned bounding box using integer coordinates.
///
/// **Rect2i** represents an axis-aligned rectangle in a 2D space, using integer coordinates. It is defined by its `position` and `size`, which are [`Vector2i`]. Because it does not rotate, it is frequently used for fast overlap tests (see [`intersects`](Rect2i::intersects)).
///
/// For floating-point coordinates, see [`Rect2`].
///
/// **Note:** Negative values for `size` are not supported. With negative size, most **Rect2i** methods do not work correctly. Use [`abs`](Rect2i::abs) to get an equivalent **Rect2i** with a non-negative size.
///
/// **Note:** In a boolean context, a **Rect2i** evaluates to `false` if both `position` and `size` are zero (equal to [`Vector2i::ZERO`]). Otherwise, it always evaluates to `true`.
#[derive(Copy, Clone, Debug, Default)]
pub struct Rect2i {
    position: Vector2i,
    size: Vector2i,
}

impl Rect2i {
    /// Constructs a **Rect2i** by `position` and `size`.
    pub const fn new(position: Vector2i, size: Vector2i) -> Self {
        Self {position, size}
    }
    /// Constructs a **Rect2i** by setting its `position` to (`x`, `y`), and its `size` to (`width`, `height`).
    pub const fn new_from_dimension(x: int!(), y: int!(), width: int!(), height: int!()) -> Self {
        Self::new(Vector2i::new(x,y), Vector2i::new(width,height))
    }

    /// Returns a **Rect2i** equivalent to this rectangle, with its width and height modified to be non-negative values, and with its `position` being the top-left corner of the rectangle.
    ///
    /// **Note:** It's recommended to use this method when `size` is negative, as most other methods in Grimm assume that the `position` is the top-left corner, and the `end` is the bottom-right corner.
    pub fn abs(&self) -> Self {
        Self::new(self.position + self.size.min_i(0), self.size.abs())
    }

    /// Returns `true` if this **Rect2i** completely encloses another one.
    pub fn encloses(&self, b: &Self) -> bool {
        (b.position.x >= self.position.x) && (b.position.y >= self.position.y) &&
            ((b.position.x + b.size.x) <= (self.position.x + self.size.x)) &&
            ((b.position.y + b.size.y) <= (self.position.y + self.size.y))
    }

    fn expand_to(&mut self, to: &Vector2i) {
        let mut begin = self.position;
        let mut end = self.position + self.size;

        if (to.x < begin.x) {
            begin.x = to.x;
        }
        if (to.y < begin.y) {
            begin.y = to.y;
        }

        if (to.x > end.x) {
            end.x = to.x;
        }
        if (to.y > end.y) {
            end.y = to.y;
        }

        self.position = begin;
        self.size = end - begin;
    }

    /// Returns a copy of this rectangle expanded to align the edges with the given `to` point, if necessary.
    pub fn expand(&self, to: &Vector2i) -> Self {
        let mut r = *self;
        r.expand_to(to);
        r
    }

    /// Returns the rectangle's area. This is equivalent to `size.x * size.y`. See also [`has_area`](Rect2i::has_area).
    pub fn get_area(&self) -> int!() {
        self.size.x * self.size.y
    }

    /// Returns the center point of the rectangle. This is the same as `position + (size / 2)`.
    ///
    /// **Note:** If the `size` is odd, the result will be rounded towards `position`.
    pub fn get_center(&self) -> Vector2i {
        self.position + (self.size / 2)
    }

    /// Returns a copy of this rectangle extended on all sides by the given `amount`. A negative `amount` shrinks the rectangle instead. See also [`grow_individual`](Rect2i::grow_individual) and [`grow_side`](Rect2i::grow_side).
    pub fn grow(&self, amount: int!()) -> Self {
        let mut g = *self;
        g.position.x -= amount;
        g.position.y -= amount;
        g.size.x += amount * 2;
        g.size.y += amount * 2;
        g

    }

    /// Returns a copy of this rectangle with its `left`, `top`, `right`, and `bottom` sides extended by the given amounts. Negative values shrink the sides, instead. See also [`grow`](Rect2i::grow) and [`grow_side`](Rect2i::grow_side).
    pub fn grow_individual(&self, left: int!(), top: int!(), right: int!(), bottom: int!()) -> Self {
        let mut g = *self;
        g.position.x -= left;
        g.position.y -= top;
        g.size.x += left + right;
        g.size.y += top + bottom;
        g
    }

    /// Returns a copy of this rectangle with its `side` extended by the given `amount` (see [`Side`]). A negative `amount` shrinks the rectangle, instead. See also [`grow`](Rect2i::grow) and [`grow_individual`](Rect2i::grow_individual).
    pub fn grow_side(&self, side: Side, amount: int!()) -> Self {
        let (left, top, right, bottom) = match side {
            Side::Left => (amount, 0, 0, 0),
            Side::Top => (0, amount, 0, 0),
            Side::Right => (0, 0, amount, 0),
            Side::Bottom => (0, 0, 0, amount),
            _ => (0, 0, 0, 0),
        };
        self.grow_individual(left, top, right, bottom)
    }

    /// Returns `true` if this rectangle has positive width and height. See also [`get_area`](Rect2i::get_area).
    pub fn has_area(&self) -> bool {
        self.size.x > 0 && self.size.y > 0
    }

    /// Returns `true` if the rectangle contains the given `point`. By convention, points on the right and bottom edges are _**not**_ included.
    ///
    /// **Note:** This method is not reliable for **Rect2i** with a negative `size`. Use [`abs`](Rect2i::abs) first to get a valid rectangle.
    pub fn has_point(&self, point: &Vector2i) -> bool {
        if (point.x < self.position.x) {
            return false;
        }
        if (point.y < self.position.y) {
            return false;
        }

        if (point.x >= (self.position.x + self.size.x)) {
            return false;
        }
        if (point.y >= (self.position.y + self.size.y)) {
            return false;
        }

        true
    }

    /// Returns the intersection between this rectangle and `b`. If the rectangles do not intersect, returns an empty **Rect2i**.
    ///
    /// **Note:** If you only need to know whether two rectangles are overlapping, use [`intersects`](Rect2i::intersects), instead.
    pub fn intersection(&self, b: &Self) -> Self {
        let mut new_rect = *b;

        if (!self.intersects(&new_rect)) {
            return Rect2i::default();
        }

        new_rect.position = b.position.max(&self.position);

        let b_end = b.position + b.size;
        let end = self.position + self.size;

        new_rect.size = b_end.min(&end) - new_rect.position;

        new_rect
    }

    /// Returns `true` if this rectangle overlaps with the `b` rectangle. The edges of both rectangles are excluded.
    pub fn intersects(&self, b: &Self) -> bool {
        if (self.position.x >= (b.position.x + b.size.x)) {
            return false;
        }
        if ((self.position.x + self.size.x) <= b.position.x) {
            return false;
        }
        if (self.position.y >= (b.position.y + b.size.y)) {
            return false;
        }
        if ((self.position.y + self.size.y) <= b.position.y) {
            return false;
        }

        true
    }

    /// Returns a **Rect2i** that encloses both this rectangle and `b` around the edges. See also [`encloses`](Rect2i::encloses).
    pub fn merge(&self, b: &Self) -> Self {
        let mut new_rect = Rect2i::default();

        new_rect.position = b.position.min(&self.position);

        new_rect.size = (b.position + b.size).max(&(self.position + self.size));

        new_rect.size = new_rect.size - new_rect.position; // Make relative again.

        new_rect
    }

    pub fn end(&self) -> Vector2i {
        self.position + self.size
    }
    pub fn set_end(&mut self, end: Vector2i) {
        self.size = end - self.position;
    }

    pub fn position(&self) -> Vector2i {
        self.position
    }
    pub fn set_position(&mut self, position: Vector2i) {
        self.position = position;
    }

    pub fn size(&self) -> Vector2i {
        self.size
    }
    pub fn set_size(&mut self, size: Vector2i) {
        self.size = size;
    }
}

impl From<Rect2> for Rect2i {
    fn from(value: Rect2) -> Self {
        Rect2i::new(Vector2i::from(value.position()), Vector2i::from(value.size()))
    }
}

impl PartialEq for Rect2i {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position && self.size == other.size
    }
}
impl Eq for Rect2i {}

impl Display for Rect2i {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "[P: ({}, {}), S: ({}, {})]",
            self.position.x, self.position.y, self.size.x, self.size.y
        ))
    }
}

impl Not for Rect2i {
    type Output = bool;
    fn not(self) -> Self::Output {
        self.position == Vector2i::ZERO && self.size == Vector2i::ZERO
    }
}