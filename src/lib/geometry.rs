//! 整数座標の2次元幾何を扱います。
//!
//! x 軸は右向き、y 軸は上向きの座標系を想定します。画面やグリッドのように
//! y が下向きに増える座標系では、時計回り・反時計回りの見え方が逆になります。
//! 座標だけでなく、差・積・和を含む途中計算も `isize` に収まる必要があります。

use std::cmp::Ordering;
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
    ///
    /// `self.x * other.x + self.y * other.y` です。両方が零ベクトルでない場合、
    /// 正ならなす角は鋭角、`0` なら直角、負なら鈍角または反対向きです。
    /// 点を引き算して作った方向ベクトルの角度判定に使えます。計算量は `O(1)` です。
    pub const fn dot(self, other: Self) -> isize {
        self.x * other.x + self.y * other.y
    }

    /// 2つのベクトルの外積を返します。
    ///
    /// `self.x * other.y - self.y * other.x` です。正なら `other` は `self` の
    /// 反時計回り側、負なら時計回り側です。`0` は平行で、逆向きや零ベクトルも含みます。
    /// 絶対値は2つのベクトルが作る平行四辺形の面積です。計算量は `O(1)` です。
    pub const fn cross(self, other: Self) -> isize {
        self.x * other.y - self.y * other.x
    }

    /// 2つのベクトルが平行なら `true` を返します。
    ///
    /// 外積が `0` かを判定します。逆向きも平行とし、零ベクトルを含む場合も `true` です。
    pub const fn is_parallel(self, other: Self) -> bool {
        self.cross(other) == 0
    }

    /// 2つのベクトルが直交するなら `true` を返します。
    ///
    /// 内積が `0` かを判定します。零ベクトルを含む場合も `true` です。
    pub const fn is_orthogonal(self, other: Self) -> bool {
        self.dot(other) == 0
    }

    /// 原点からの距離の二乗を返します。
    ///
    /// `x * x + y * y` です。平方根を取らずに距離の大小を比較できます。
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
    ///
    /// `(self.x - other.x)^2 + (self.y - other.y)^2` です。実際の距離ではなく
    /// その二乗を返すため、距離の比較にも閾値の二乗を使います。計算量は `O(1)` です。
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

/// 原点から見た2点の偏角を比較します。
///
/// 正のx軸を始点に反時計回りで比較し、同じ向きなら [`Ordering::Equal`] を返します。
/// 原点は含めず、座標同士の積が `isize` に収まる範囲で使用してください。
/// 計算量は `O(1)` です。
/// 同じ向きの点同士は距離では並べ替えません。別の点を中心にする場合は、
/// その中心を両方の点から引いたベクトルを比較してください。
///
/// # Examples
///
/// ```
/// use atcoder::geometry::{argcmp, Point};
///
/// let mut points = [Point::new(0, -1), Point::new(0, 1), Point::new(1, 0)];
/// points.sort_by(|&a, &b| argcmp(a, b));
/// assert_eq!(points, [Point::new(1, 0), Point::new(0, 1), Point::new(0, -1)]);
/// ```
///
/// # 参考
///
/// ngtkanaさんの[整数演算による偏角ソートの記事](https://blog.ngtkana.com/entry/2021/11/13/202103)を参考にしています。
pub fn argcmp(a: Point, b: Point) -> Ordering {
    // 辞書順で (y, x) < (0, 0) は「y < 0、または y == 0 かつ x < 0」。
    // 下半平面と負のx軸を後半に分け、角度が一周する境界を正のx軸に置く。
    ((a.y, a.x) < (0, 0))
        .cmp(&((b.y, b.x) < (0, 0)))
        // 同じ半分の中では外積の符号で順序を決める。a×b > 0 なら a が先。
        .then((b.x * a.y).cmp(&(a.x * b.y)))
}

/// 3点 `a`, `b`, `c` の向きを外積で返します。
///
/// 正なら反時計回り、負なら時計回り、`0` なら一直線上です。計算量は `O(1)` です。
///
/// 有向直線 `a → b` から見て `c` が左側なら正、右側なら負です。
/// 戻り値は `(b - a) × (c - a)` そのもので、`-1 / 0 / 1` に正規化しません。
/// 絶対値は三角形 `abc` の面積の2倍です。`0` だけでは `c` が線分上かは
/// 分からないため、線分上の判定には [`on_segment`] を使ってください。
///
/// ```
/// use atcoder::geometry::{ccw, Point};
/// let a = Point::new(0, 0);
/// let b = Point::new(2, 0);
/// assert_eq!(ccw(a, b, Point::new(1, 3)), 6);  // 左側（上側）
/// assert_eq!(ccw(a, b, Point::new(1, -3)), -6); // 右側（下側）
/// assert_eq!(ccw(a, b, Point::new(3, 0)), 0);  // 直線上だが線分外
/// ```
pub fn ccw(a: Point, b: Point, c: Point) -> isize {
    (b - a).cross(c - a)
}

/// 点 `point` が閉線分 `ab` 上にあるか判定します。
///
/// 端点を含みます。計算量は `O(1)` です。
/// `a == b` のときは、`point == a` の場合だけ `true` です。
pub fn on_segment(a: Point, b: Point, point: Point) -> bool {
    // 同一直線上で、point から両端へのベクトルが反対向き（端点なら一方が0）か調べる。
    ccw(a, b, point) == 0 && (a - point).dot(b - point) <= 0
}

/// 2つの閉線分 `ab` と `cd` が交差するか判定します。
///
/// 端点での接触や同一直線上の重なりも交差に含めます。計算量は `O(1)` です。
/// 長さ0の線分も扱えます。交点の座標や交差の種類は返しません。
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
/// 最後の頂点と最初の頂点は自動的につなぐため、先頭頂点を末尾に再追加する必要はありません。
/// 自己交差しない多角形の通常の面積は、戻り値の絶対値を2で割った値です。
/// 戻り値が奇数なら面積は半整数なので、整数除算による切り捨てに注意してください。
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
    use std::cmp::Ordering;

    use super::{argcmp, polygon_area2, segments_intersect, Point};

    #[test]
    fn compares_polar_angles_counterclockwise_from_positive_x_axis() {
        let points = [
            Point::new(1, 0),
            Point::new(1, 1),
            Point::new(0, 1),
            Point::new(-1, 1),
            Point::new(-1, 0),
            Point::new(-1, -1),
            Point::new(0, -1),
            Point::new(1, -1),
        ];
        for (i, &a) in points.iter().enumerate() {
            for (j, &b) in points.iter().enumerate() {
                assert_eq!(argcmp(a, b), i.cmp(&j));
            }
        }
    }

    #[test]
    fn compares_same_direction_vectors_as_equal() {
        assert_eq!(argcmp(Point::new(1, 2), Point::new(2, 4)), Ordering::Equal);
        assert_eq!(
            argcmp(Point::new(-1, -2), Point::new(-2, -4)),
            Ordering::Equal
        );
        assert_ne!(
            argcmp(Point::new(1, 2), Point::new(-1, -2)),
            Ordering::Equal
        );
    }

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
