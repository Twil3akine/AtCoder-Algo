//! 外部 crate を使わない軽量な疑似乱数生成器です。

/// Xorshift による64 bit疑似乱数生成器です。
///
/// 競技中のランダムテストやヒューリスティック用途を想定しています。暗号用途には適しません。
/// 内部状態を更新する各生成操作は `O(1)` で、同じ seed からは同じ列を生成します。
///
/// # Examples
///
/// ```
/// use atcoder::random::Xorshift;
///
/// let mut rng = Xorshift::new(7);
/// let value = rng.range_inclusive(1, 6);
/// assert!((1..=6).contains(&value));
/// ```
#[derive(Debug, Clone)]
pub struct Xorshift {
    state: u64,
}

impl Xorshift {
    /// `seed` を初期状態として生成器を作ります。
    ///
    /// `seed == 0` の場合は固定の非ゼロ値へ置き換えます。同じ seed から作った生成器は同じ値列を生成します。
    pub const fn new(seed: u64) -> Self {
        Self {
            state: if seed == 0 {
                88_172_645_463_325_252
            } else {
                seed
            },
        }
    }

    /// Xorshift 更新を1回行い、次の `u64` を返します。
    pub fn next_u64(&mut self) -> u64 {
        self.state ^= self.state << 13;
        self.state ^= self.state >> 7;
        self.state ^= self.state << 17;
        self.state
    }

    /// [`next_u64`](Self::next_u64) を呼び出し、次の `u64` を返します。
    ///
    /// [`next_u64`](Self::next_u64) と同じです。
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> u64 {
        self.next_u64()
    }

    /// 閉区間 `[min, max]` から疑似乱数の `usize` を返します。
    ///
    /// `min > max` の場合と、区間幅が `usize` で表せない場合は panic します。剰余による
    /// ごく小さな偏りがあるため、厳密な一様分布が必要な用途には適しません。特に `0..=usize::MAX`
    /// は幅が `usize` に収まらないため panic します。
    pub fn range_inclusive(&mut self, min: usize, max: usize) -> usize {
        assert!(min <= max, "empty range");
        let width = max.checked_sub(min).unwrap().checked_add(1).unwrap();
        min + self.next_u64() as usize % width
    }

    /// 閉区間 `[min, max]` から疑似乱数の `usize` を返します。
    ///
    /// [`range_inclusive`](Self::range_inclusive) と同じです。
    pub fn next_range(&mut self, min: usize, max: usize) -> usize {
        self.range_inclusive(min, max)
    }

    /// 半開区間 `[0.0, 1.0)` の `f64` を返します。
    ///
    /// 上位53ビットを使って値を作ります。値が等確率になる暗号学的乱数ではありません。
    pub fn next_f64(&mut self) -> f64 {
        const SCALE: f64 = (1_u64 << 53) as f64;
        (self.next_u64() >> 11) as f64 / SCALE
    }
}

#[cfg(test)]
mod tests {
    use super::Xorshift;

    #[test]
    fn is_deterministic_and_bounded() {
        let mut left = Xorshift::new(1);
        let mut right = Xorshift::new(1);
        assert_eq!(left.next_u64(), right.next_u64());
        assert!((3..=7).contains(&left.range_inclusive(3, 7)));
        assert!((0.0..1.0).contains(&left.next_f64()));
    }
}
