//! 整数座標の2次元幾何を扱います。

use std::ops::{Add, Mul, Sub};

/// 整数座標上の点またはベクトルです。
///
/// 演算結果が [`isize`] の範囲を超えないことは呼び出し側で保証してください。
///
/// # Examples
///
/// ```
/// use atcoder::geometry::Point;
///
/// let a = Point::new(1, 2);
/// let b = Point::new(4, 6);
/// assert_eq!(a.distance_squared(b), 25);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    /// x 座標です。
    pub x: isize,
    /// y 座標です。
    pub y: isize,
}

impl Point {
    /// 座標 `(x, y)` の点を作ります。
    pub const fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    /// 2つのベクトルの内積を返します。
    pub const fn dot(self, other: Self) -> isize {
        self.x * other.x + self.y * other.y
    }

    /// 2つのベクトルの外積を返します。
    pub const fn cross(self, other: Self) -> isize {
        self.x * other.y - self.y * other.x
    }

    /// 2つのベクトルが平行なら `true` を返します。
    pub const fn is_parallel(self, other: Self) -> bool {
        self.cross(other) == 0
    }

    /// 2つのベクトルが直交するなら `true` を返します。
    pub const fn is_orthogonal(self, other: Self) -> bool {
        self.dot(other) == 0
    }

    /// 原点からの距離の二乗を返します。
    pub const fn norm_squared(self) -> isize {
        self.dot(self)
    }

    /// 原点からの距離の二乗を返します。
    ///
    /// [`norm_squared`](Self::norm_squared) と同じです。
    pub const fn norm2(self) -> isize {
        self.norm_squared()
    }

    /// `other` までの距離の二乗を返します。
    pub fn distance_squared(self, other: Self) -> isize {
        (self - other).norm_squared()
    }

    /// `other` までの距離の二乗を返します。
    ///
    /// [`distance_squared`](Self::distance_squared) と同じです。
    pub fn dist2(self, other: Self) -> isize {
        self.distance_squared(other)
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Mul<isize> for Point {
    type Output = Self;

    fn mul(self, rhs: isize) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

/// 3点 `a`, `b`, `c` の向きを外積で返します。
///
/// 正なら反時計回り、負なら時計回り、`0` なら一直線上です。計算量は `O(1)` です。
pub fn ccw(a: Point, b: Point, c: Point) -> isize {
    (b - a).cross(c - a)
}

/// 点 `point` が閉線分 `ab` 上にあるか判定します。
///
/// 端点を含みます。計算量は `O(1)` です。
pub fn on_segment(a: Point, b: Point, point: Point) -> bool {
    ccw(a, b, point) == 0 && (a - point).dot(b - point) <= 0
}

/// 2つの閉線分 `ab` と `cd` が交差するか判定します。
///
/// 端点での接触や同一直線上の重なりも交差に含めます。計算量は `O(1)` です。
///
/// # Examples
///
/// ```
/// use atcoder::geometry::{segments_intersect, Point};
///
/// assert!(segments_intersect(
///     Point::new(0, 0),
///     Point::new(2, 2),
///     Point::new(0, 2),
///     Point::new(2, 0),
/// ));
/// ```
pub fn segments_intersect(a: Point, b: Point, c: Point, d: Point) -> bool {
    let ab_c = ccw(a, b, c);
    let ab_d = ccw(a, b, d);
    let cd_a = ccw(c, d, a);
    let cd_b = ccw(c, d, b);

    if ab_c == 0 && on_segment(a, b, c) {
        return true;
    }
    if ab_d == 0 && on_segment(a, b, d) {
        return true;
    }
    if cd_a == 0 && on_segment(c, d, a) {
        return true;
    }
    if cd_b == 0 && on_segment(c, d, b) {
        return true;
    }

    (ab_c > 0) != (ab_d > 0) && (cd_a > 0) != (cd_b > 0)
}

/// 多角形の符号付き面積の2倍を返します。
///
/// `points` は周上の順に並べます。反時計回りなら正、時計回りなら負です。
/// 空または頂点が2個以下なら `0` を返します。頂点数を `N` とすると `O(N)` です。
pub fn polygon_area2(points: &[Point]) -> isize {
    if points.is_empty() {
        return 0;
    }
    (0..points.len())
        .map(|index| points[index].cross(points[(index + 1) % points.len()]))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{polygon_area2, segments_intersect, Point};

    #[test]
    fn detects_touching_and_disjoint_segments() {
        assert!(segments_intersect(
            Point::new(0, 0),
            Point::new(2, 0),
            Point::new(2, 0),
            Point::new(3, 1)
        ));
        assert!(!segments_intersect(
            Point::new(0, 0),
            Point::new(1, 0),
            Point::new(2, 0),
            Point::new(3, 0)
        ));
    }

    #[test]
    fn computes_signed_double_area() {
        let triangle = [Point::new(0, 0), Point::new(3, 0), Point::new(0, 2)];
        assert_eq!(polygon_area2(&triangle), 6);
    }
}
