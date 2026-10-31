//! 遅延評価セグメント木です。

/// 区間への作用と区間に対する集約を扱う遅延評価セグメント木です。
///
/// 添字は 0-indexed、区間は `[left, right)` です。3つの関数を与えます。
/// `op(a, b)` は値の結合で、単位元 `identity` と結合則を満たす必要があります。
/// `mapping(action, value, len)` は長さ `len` の区間を表す値 `value` に作用
/// `action` を適用した結果を返します。`composition(first, second)` は作用
/// `first` の後に `second` を適用した合成を返し、単位元は `identity_action`
/// です。更新と取得はいずれも `O(log N)` です。
///
/// # Examples
///
/// ```
/// use atcoder::data_structure::lazy_segment_tree::LazySegmentTree;
///
/// // 区間加算・区間和
/// let mut tree = LazySegmentTree::new(
///     |a, b| a + b,
///     |action, value, len| value + action * len as i64,
///     |first, second| first + second,
///     vec![1, 2, 3, 4],
///     0,
///     0,
/// );
/// tree.apply(1, 3, 10);
/// assert_eq!(tree.get(0, 4), 30);
/// assert_eq!(tree.get(1, 3), 25);
/// ```
#[derive(Clone)]
pub struct LazySegmentTree<T, A, F, G, H> {
    op: F,
    mapping: G,
    composition: H,
    identity: T,
    identity_action: A,
    len: usize,
    size: usize,
    log: usize,
    data: Vec<T>,
    lazy: Vec<A>,
}

impl<T, A, F, G, H> LazySegmentTree<T, A, F, G, H>
where
    T: Copy,
    A: Copy,
    F: Fn(T, T) -> T,
    G: Fn(A, T, usize) -> T,
    H: Fn(A, A) -> A,
{
    /// 各要素の初期値を指定して木を構築します。計算量・メモリ量は `O(N)` です。
    pub fn new(
        op: F,
        mapping: G,
        composition: H,
        values: Vec<T>,
        identity: T,
        identity_action: A,
    ) -> Self {
        let len = values.len();
        let size = len.next_power_of_two().max(1);
        let log = size.trailing_zeros() as usize;
        let mut data = vec![identity; 2 * size];
        data[size..size + len].copy_from_slice(&values);
        for i in (1..size).rev() {
            let value = op(data[2 * i], data[2 * i + 1]);
            data[i] = value;
        }
        Self {
            op,
            mapping,
            composition,
            identity,
            identity_action,
            len,
            size,
            log,
            data,
            lazy: vec![identity_action; size],
        }
    }

    fn apply_node(&mut self, node: usize, action: A, segment_len: usize) {
        let value = (self.mapping)(action, self.data[node], segment_len);
        self.data[node] = value;
        if node < self.size {
            let composed = (self.composition)(self.lazy[node], action);
            self.lazy[node] = composed;
        }
    }

    fn push(&mut self, node: usize) {
        let action = self.lazy[node];
        // ノード node の深さは最上位ビットの位置で決まり、その区間長は size >> 深さ。
        // 子の区間長はその半分です。
        let depth = 63 - node.leading_zeros() as usize;
        let child_len = (self.size >> (depth + 1)).max(1);
        self.apply_node(2 * node, action, child_len);
        self.apply_node(2 * node + 1, action, child_len);
        self.lazy[node] = self.identity_action;
    }

    fn push_all(&mut self, left: usize, right: usize) {
        let l = left + self.size;
        let r = right + self.size;
        for i in (1..=self.log).rev() {
            if ((l >> i) << i) != l {
                self.push(l >> i);
            }
            if ((r >> i) << i) != r {
                self.push((r - 1) >> i);
            }
        }
    }

    fn recalc_point(&mut self, index: usize) {
        // 一点だけが変わるので、その葉の祖先だけを根まで再計算します。
        // 範囲の右端側のノードまで再計算すると、作用が未伝播の兄弟部分木を
        // 古い子の値で上書きしてしまいます。
        let mut node = (index + self.size) >> 1;
        while node >= 1 {
            let value = (self.op)(self.data[2 * node], self.data[2 * node + 1]);
            self.data[node] = value;
            if node == 1 {
                break;
            }
            node >>= 1;
        }
    }

    /// 半開区間 `[left, right)` の全要素に作用 `action` を適用します。
    pub fn apply(&mut self, left: usize, right: usize, action: A) {
        assert!(left <= right && right <= self.len);
        if left == right {
            return;
        }
        self.push_all(left, right);
        let mut l = left + self.size;
        let mut r = right + self.size;
        let mut l_len = 1usize;
        let mut r_len = 1usize;
        let mut touched: Vec<usize> = Vec::new();
        while l < r {
            if l % 2 == 1 {
                self.apply_node(l, action, l_len);
                touched.push(l);
                l += 1;
            }
            if r % 2 == 1 {
                r -= 1;
                self.apply_node(r, action, r_len);
                touched.push(r);
            }
            l >>= 1;
            r >>= 1;
            l_len <<= 1;
            r_len <<= 1;
        }
        // 作用を加えたノード自身は正しい値を持つので、再計算するのは
        // その祖先だけです。深い祖先から順に、子の値で親を更新します。
        let mut ancestors: Vec<usize> = Vec::new();
        for &node in &touched {
            let mut parent = node >> 1;
            while parent >= 1 {
                ancestors.push(parent);
                if parent == 1 {
                    break;
                }
                parent >>= 1;
            }
        }
        ancestors.sort_unstable_by_key(|&node| std::cmp::Reverse(63 - node.leading_zeros()));
        ancestors.dedup();
        for node in ancestors {
            let value = (self.op)(self.data[2 * node], self.data[2 * node + 1]);
            self.data[node] = value;
        }
    }

    /// 半開区間 `[left, right)` の値を `op` で集約して返します。
    pub fn get(&mut self, left: usize, right: usize) -> T {
        assert!(left <= right && right <= self.len);
        if left == right {
            return self.identity;
        }
        self.push_all(left, right);
        let mut l = left + self.size;
        let mut r = right + self.size;
        let mut left_value = self.identity;
        let mut right_value = self.identity;
        while l < r {
            if l % 2 == 1 {
                left_value = (self.op)(left_value, self.data[l]);
                l += 1;
            }
            if r % 2 == 1 {
                r -= 1;
                right_value = (self.op)(self.data[r], right_value);
            }
            l >>= 1;
            r >>= 1;
        }
        (self.op)(left_value, right_value)
    }

    /// 添字 `index` に値を代入します。未伝播の作用は先に押し下げます。
    pub fn set(&mut self, index: usize, value: T) {
        self.push_all(index, index + 1);
        self.data[self.size + index] = value;
        self.recalc_point(index);
    }

    /// 添字 `index` の値を返します。
    pub fn at(&mut self, index: usize) -> T {
        self.get(index, index + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn applies_and_queries() {
        let mut tree = LazySegmentTree::new(
            |a, b| a + b,
            |action, value, len| value + action * len as i64,
            |first, second| first + second,
            vec![1, 2, 3, 4, 5],
            0,
            0,
        );
        tree.apply(0, 5, 1);
        assert_eq!(tree.get(0, 5), 20);
        tree.apply(2, 4, 10);
        assert_eq!(tree.get(1, 4), 32);
        assert_eq!(tree.at(4), 6);
        tree.set(0, 100);
        assert_eq!(tree.get(0, 2), 103);
    }

    #[test]
    fn range_assign_range_min() {
        // 区間代入・区間最小。作用は Option<i64>（None が単位元）。
        let mut tree = LazySegmentTree::new(
            |a: i64, b: i64| a.min(b),
            |action: Option<i64>, value, _len| action.unwrap_or(value),
            |first, second| second.or(first),
            vec![5, 3, 8, 1],
            i64::MAX,
            None,
        );
        tree.apply(0, 3, Some(4));
        assert_eq!(tree.get(0, 4), 1);
        assert_eq!(tree.get(0, 3), 4);
    }

    #[test]
    fn matches_naive_on_random_operations() {
        // 決定的な疑似乱数で、素朴な配列シミュレーションと照合します。
        let mut state = 0x2545F4914F6CDD1Du64;
        let mut next = move || {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            state
        };
        for _ in 0..50 {
            let n = (next() % 40 + 1) as usize;
            let mut naive: Vec<i64> = (0..n).map(|_| (next() % 20) as i64).collect();
            let mut tree = LazySegmentTree::new(
                |a, b| a + b,
                |action, value, len| value + action * len as i64,
                |first, second| first + second,
                naive.clone(),
                0,
                0,
            );
            for _ in 0..200 {
                let l = (next() as usize) % (n + 1);
                let r = (next() as usize) % (n + 1);
                let (l, r) = if l <= r { (l, r) } else { (r, l) };
                match next() % 3 {
                    0 => {
                        let v = (next() % 10) as i64;
                        tree.apply(l, r, v);
                        for x in &mut naive[l..r] {
                            *x += v;
                        }
                    }
                    1 => {
                        let expected: i64 = naive[l..r].iter().sum();
                        assert_eq!(tree.get(l, r), expected);
                    }
                    _ => {
                        if l < n {
                            let v = (next() % 30) as i64;
                            tree.set(l, v);
                            naive[l] = v;
                            assert_eq!(tree.at(l), v);
                        }
                    }
                }
            }
            assert_eq!(tree.get(0, n), naive.iter().sum::<i64>());
        }
    }

    #[test]
    fn assign_matches_naive_on_random_operations() {
        let mut state = 0x9E3779B97F4A7C15u64;
        let mut next = move || {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            state
        };
        for _ in 0..30 {
            let n = (next() % 24 + 1) as usize;
            let mut naive: Vec<i64> = (0..n).map(|_| (next() % 50) as i64).collect();
            let mut tree = LazySegmentTree::new(
                |a: i64, b: i64| a.min(b),
                |action: Option<i64>, value, _len| action.unwrap_or(value),
                |first, second| second.or(first),
                naive.clone(),
                i64::MAX,
                None,
            );
            for _ in 0..150 {
                let l = (next() as usize) % (n + 1);
                let r = (next() as usize) % (n + 1);
                let (l, r) = if l <= r { (l, r) } else { (r, l) };
                if next() % 2 == 0 {
                    let v = (next() % 50) as i64;
                    tree.apply(l, r, Some(v));
                    for x in &mut naive[l..r] {
                        *x = v;
                    }
                } else {
                    let expected = naive[l..r].iter().min().copied().unwrap_or(i64::MAX);
                    assert_eq!(tree.get(l, r), expected);
                }
            }
        }
    }
}
