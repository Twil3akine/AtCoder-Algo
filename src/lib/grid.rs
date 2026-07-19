//! 2次元グリッドの添字と移動方向です。

/// 4近傍を右、上、左、下の順に表します。
pub const DIRECTIONS_4: [(isize, isize); 4] = [(0, 1), (-1, 0), (0, -1), (1, 0)];

/// 8近傍を右、上、左、下、右上、左上、左下、右下の順に表します。
pub const DIRECTIONS_8: [(isize, isize); 8] = [
    (0, 1),
    (-1, 0),
    (0, -1),
    (1, 0),
    (-1, 1),
    (-1, -1),
    (1, -1),
    (1, 1),
];

/// `(row, column)` が `height × width` のグリッド内なら `true` を返します。
///
/// 添字は 0-indexed です。計算量は `O(1)` です。
pub const fn contains(height: usize, width: usize, (row, column): (usize, usize)) -> bool {
    row < height && column < width
}

#[cfg(test)]
mod tests {
    use super::contains;

    #[test]
    fn checks_bounds() {
        assert!(contains(2, 3, (1, 2)));
        assert!(!contains(2, 3, (2, 0)));
    }
}
