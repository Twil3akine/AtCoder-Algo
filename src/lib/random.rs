//! 外部 crate を使わない軽量な疑似乱数生成器です。

/// Xorshift による64 bit疑似乱数生成器です。
///
/// 競技中のランダムテストやヒューリスティック用途を想定しています。暗号用途には適しません。
/// 各生成操作は `O(1)` です。同じ seed からは同じ列を生成します。
#[derive(Debug, Clone)]
pub struct Xorshift {
    state: u64,
}

impl Xorshift {
    /// `seed` を初期状態として作ります。`0` は固定の非ゼロ値へ置き換えます。
    pub const fn new(seed: u64) -> Self {
        Self {
            state: if seed == 0 {
                88_172_645_463_325_252
            } else {
                seed
            },
        }
    }

    /// 次の `u64` を返します。
    pub fn next_u64(&mut self) -> u64 {
        self.state ^= self.state << 13;
        self.state ^= self.state >> 7;
        self.state ^= self.state << 17;
        self.state
    }

    /// 閉区間 `[min, max]` の `usize` を返します。
    ///
    /// `min > max` の場合と、区間幅が `usize` で表せない場合は panic します。剰余による
    /// ごく小さな偏りがあるため、厳密な一様分布が必要な用途には適しません。
    pub fn range_inclusive(&mut self, min: usize, max: usize) -> usize {
        assert!(min <= max, "empty range");
        let width = max.checked_sub(min).unwrap().checked_add(1).unwrap();
        min + self.next_u64() as usize % width
    }

    /// 半開区間 `[0.0, 1.0)` の `f64` を返します。
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
