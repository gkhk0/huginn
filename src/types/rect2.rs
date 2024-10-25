use crate::float;
use crate::types::vectors::Vector2;
use crate::types::{Side, Transform2D};
use auto_ops::impl_op_ex;
use std::fmt::{Display, Formatter};

/// A 2D axis-aligned bounding box using floating-point coordinates.
///
/// **Rect2** represents an axis-aligned rectangle in a 2D space. It is defined by its `position` and `size`, which are [`Vector2`]. It is frequently used for fast overlap tests (see [`intersects`](Rect2::intersects)). Although **Rect2** itself is axis-aligned, it can be combined with [`Transform2D`] to represent a rotated or skewed rectangle.
///
/// For integer coordinates, use [`Rect2i`]. The 3D equivalent to **Rect2** is [`AABB`](crate::types::AABB).
///
/// **Note:** Negative values for `size` are not supported. With negative size, most **Rect2** methods do not work correctly. Use [`abs`](Rect2::abs) to get an equivalent **Rect2** with a non-negative size.
///
/// **Note:** In a boolean context, a **Rect2** evaluates to `false` if both `position` and `size` are zero (equal to [`Vector2::ZERO`]). Otherwise, it always evaluates to `true`.
#[derive(Copy, Clone, Debug, Default)]
pub struct Rect2 {
    position: Vector2,
    size: Vector2,
}

impl Rect2 {
    pub fn get_support(&self, direction: &Vector2) -> Vector2 {
        let mut support = self.position;
        if direction.x > 0.0 {
            support.x += self.size.x;
        }
        if direction.y > 0.0 {
            support.y += self.size.y;
        }
        support
    }
}

impl Rect2 {
    /// Constructs a **Rect2** by `position` and `size`.
    pub const fn new(position: Vector2, size: Vector2) -> Self {
        Self { position, size }
    }

    /// Constructs a **Rect2** by setting its `position` to (`x`, `y`), and its `size` to (`width`, `height`).
    pub const fn new_from_dimension(
        x: float!(),
        y: float!(),
        width: float!(),
        height: float!(),
    ) -> Self {
        Self::new(Vector2::new(x, y), Vector2::new(width, height))
    }

    /// Returns a **Rect2** equivalent to this rectangle, with its width and height modified to be non-negative values, and with its `position` being the top-left corner of the rectangle.
    ///
    /// ```
    /// # use huginn::types::Rect2;
    /// let rect = Rect2::new_from_dimension(25.0, 25.0, -100.0, -50.0);
    /// let absolute = rect.abs(); // absolute is React2(-75, -25, 100, 50
    /// ```
    ///
    /// **Note:** It's recommended to use this method when `size` is negative, as most other methods in Grimm assume that the `position` is the top-left corner, and the `end` is the bottom-right corner.
    pub fn abs(&self) -> Self {
        Self::new(self.position + self.size.min_f(0.0), self.size.abs())
    }

    /// Returns `true` if this rectangle *completely* encloses the `b` rectangle.
    pub fn encloses(&self, b: &Self) -> bool {
        (b.position.x >= self.position.x)
            && (b.position.y >= self.position.y)
            && ((b.position.x + b.size.x) <= (self.position.x + self.size.x))
            && ((b.position.y + b.size.y) <= (self.position.y + self.size.y))
    }

    fn expand_to(&mut self, to: &Vector2) {
        let mut begin = self.position;
        let mut end = self.end();
        begin = begin.min(&to);
        end = end.max(&to);
        self.position = begin;
        self.size = end - begin;
    }

    /// Returns a copy of this rectangle expanded to align the edges with the given `to` point, if necessary.
    ///
    /// ```
    /// use huginn::types::Rect2;
    /// use huginn::types::vectors::Vector2;
    /// let mut rect = Rect2::new_from_dimension(0.0, 0.0, 5.0, 2.0);
    ///
    /// rect = rect.expand(&Vector2::new(10.0, 0.0)); // rect is Rect2(0, 0, 10, 2)
    /// rect = rect.expand(&Vector2::new(-5.0,5.0)); // rect is Rect2(-5, 0, 15, 5)
    /// ```
    pub fn expand(&self, to: &Vector2) -> Self {
        let mut r = *self;
        r.expand_to(to);
        r
    }

    /// Returns the rectangle's area. This is equivalent to `size.x * size.y`. See also [`has_area`](Rect2::has_area).
    pub fn get_area(&self) -> float!() {
        self.size.x * self.size.y
    }

    /// Returns the center point of the rectangle. This is the same as `position + (size / 2.0)`.
    pub fn get_center(&self) -> Vector2 {
        self.position + (self.size / 2.0)
    }

    fn grow_by(&mut self, amount: float!()) {
        self.position.x -= amount;
        self.position.y -= amount;
        self.size.x += amount * 2.0;
        self.size.y += amount * 2.0
    }

    /// Returns a copy of this rectangle extended on all sides by the given `amount`. A negative `amount` shrinks the rectangle instead. See also [`grow_individual`](Rect2::grow_individual) and [`grow_side`](Rect2::grow_side).
    ///
    /// ```
    /// # use huginn::types::Rect2;
    /// let a = Rect2::new_from_dimension(4.0, 4.0, 8.0, 8.0).grow(4.0); // a is Rect2(0, 0, 16, 16)
    /// let b = Rect2::new_from_dimension(0.0, 0.0, 8.0, 4.0).grow(2.0); // b is Rect2(-2, -2, 12, 8)
    /// ```
    pub fn grow(&self, amount: float!()) -> Self {
        let mut g = *self;
        g.grow_by(amount);
        g
    }

    /// Returns a copy of this rectangle with its `left`, `top`, `right`, and `bottom` sides extended by the given amounts. Negative values shrink the sides, instead. See also [`grow`](Rect2::grow) and [`grow_side`](Rect2::grow_side).
    pub fn grow_individual(
        &self,
        left: float!(),
        top: float!(),
        right: float!(),
        bottom: float!(),
    ) -> Self {
        let mut g = *self;
        g.position.x -= left;
        g.position.y -= top;
        g.size.x += left + right;
        g.size.y += top + bottom;

        g
    }

    /// Returns a copy of this rectangle with its `side` extended by the given `amount` (see [`Side`]). A negative `amount` shrinks the rectangle, instead. See also [`grow`](Rect2::grow) and [`grow_individual`](Rect2::grow_individual).
    pub fn grow_side(&self, side: Side, amount: float!()) -> Self {
        let (left, top, right, bottom) = match side {
            Side::Left => (amount, 0.0, 0.0, 0.0),
            Side::Top => (0.0, amount, 0.0, 0.0),
            Side::Right => (0.0, 0.0, amount, 0.0),
            Side::Bottom => (0.0, 0.0, 0.0, amount),
            _ => (0.0, 0.0, 0.0, 0.0),
        };
        self.grow_individual(left, top, right, bottom)
    }

    /// Returns `true` if this rectangle has positive width and height. See also [`get_area`](Rect2::get_area).
    pub fn has_area(&self) -> bool {
        self.size.x > 0.0 && self.size.y > 0.0
    }

    /// Returns `true` if the rectangle contains the given `point`. By convention, points on the right and bottom edges are not included.
    ///
    /// **Note:** This method is not reliable for **Rect2** with a *negative* `size`. Use [`abs`](Rect2::abs) first to get a valid rectangle.
    pub fn has_point(&self, point: &Vector2) -> bool {
        point.x >= self.position.x
            && point.y >= self.position.y
            && point.x < (self.position.x + self.size.x)
            && point.y < (self.position.y + self.size.y)
    }

    /// Returns the intersection between this rectangle and `b`. If the rectangles do not intersect, returns an empty **Rect2**.
    ///
    /// ```
    /// # use huginn::types::Rect2;
    /// let rect1 = Rect2::new_from_dimension(0.0, 0.0, 5.0, 10.0);
    /// let rect2 = Rect2::new_from_dimension(2.0, 0.0, 8.0, 4.0);
    ///
    /// let a = rect1.intersection(&rect2); // a is Rect2(2, 0, 3, 4)
    /// ```
    ///
    /// **Note:** If you only need to know whether two rectangles are overlapping, use [`intersects`](Rect2::intersects), instead.
    pub fn intersection(&self, b: &Self) -> Self {
        let mut new_rect = *b;
        if !self.intersects(&new_rect, false) {
            return Rect2::default();
        }

        new_rect.position = b.position.max(&self.position);

        let b_end = b.position + b.size;
        let end = self.position + self.size;

        new_rect.size = b_end.min(&end) - new_rect.position;

        new_rect
    }

    /// Returns `true` if this rectangle overlaps with the `b` rectangle. The edges of both rectangles are excluded, unless `include_borders` is `true`.
    pub fn intersects(&self, b: &Self, include_borders: bool) -> bool {
        if (include_borders) {
            if self.position.x > (b.position.x + b.size.x) {
                return false;
            }
            if (self.position.x + self.size.x) < b.position.x {
                return false;
            }
            if (self.position.y > (b.position.y + b.size.y)) {
                return false;
            }
            if ((self.position.y + self.size.y) < b.position.y) {
                return false;
            }
        } else {
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
        }

        return true;
    }

    /// Returns `true` if this rectangle and `rect` are approximately equal, by calling [`Vector2::is_equal_approx`] on the `position` and the `size`.
    pub fn is_equal_approx(&self, rect: &Self) -> bool {
        self.position.is_equal_approx(&rect.position) && self.size.is_equal_approx(&rect.size)
    }

    /// Returns `true` if this rectangle's values are finite, by calling [`Vector2::is_finite`] on the `position` and the `size`.
    pub fn is_finite(&self) -> bool {
        self.position.is_finite() && self.size.is_finite()
    }

    /// Returns a **Rect2** that encloses both this rectangle and `b` around the edges. See also [`encloses`](Rect2::encloses).
    pub fn merge(&self, b: &Self) -> Self {
        let mut new_rect = Rect2::default();

        new_rect.position = b.position.min(&self.position);

        new_rect.size = (b.position + b.size).max(&(self.position + self.size));

        new_rect.size = new_rect.size - new_rect.position; // Make relative again.

        new_rect
    }

    /// The ending point. This is usually the bottom-right corner of the rectangle, and is equivalent to `position + size`. Setting this point affects the `size`.
    pub fn end(&self) -> Vector2 {
        self.position + self.size
    }

    pub fn set_end(&mut self, end: Vector2) {
        self.size = end - self.position;
    }

    /// The origin point. This is usually the top-left corner of the rectangle.
    pub fn position(&self) -> Vector2 {
        self.position
    }
    pub fn set_position(&mut self, position: Vector2) {
        self.position = position;
    }

    /// The rectangle's width and height, starting from `position`. Setting this value also affects the `end` point.
    ///
    /// **Note:** It's recommended setting the width and height to non-negative values, as most methods in Grimm assume that the `position` is the top-left corner, and the `end` is the bottom-right corner. To get an equivalent rectangle with non-negative size, use [`abs`](Rect2::abs).
    pub fn size(&self) -> Vector2 {
        self.size
    }
    pub fn set_size(&mut self, size: Vector2) {
        self.size = size;
    }
}

// TODO: implement from Rect2i
//impl From<Rect2i> for Rect {
//    /// Constructs a **Rect2** from a [`Rect2i`].
//    fn from(value: Rect2i) -> Self {
//        todo!()
//    }
//}

impl PartialEq for Rect2 {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position && self.size == other.size
    }
}

impl_op_ex!(*= |a: &mut Rect2, b: &Transform2D| {
    let x = b.x * a.size.x;
    let y = b.y * a.size.y;
    let pos = b.xform(&a.position);

    a.position = pos;
    a.expand_to(&(pos + x));
    a.expand_to(&(pos + y));
    a.expand_to(&(pos + x + y));
});

impl_op_ex!(*|a: &Rect2, b: &Transform2D| -> Rect2 {
    let mut r = *a;
    r *= b;
    r
});

impl Display for Rect2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "[P: ({}, {}), S: ({}, {})]",
            self.position.x, self.position.y, self.size.x, self.size.y
        ))
    }
}
