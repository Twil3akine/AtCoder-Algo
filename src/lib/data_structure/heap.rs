//! 最大ヒープと最小ヒープです。

use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// [`Ord`] で比較できる要素を、最大値から順に取り出すヒープです。
///
/// [`BinaryHeap`] の型エイリアスです。要素の追加・最大値の取り出しは `O(log N)`、
/// 最大値の参照は `O(1)` です。空のヒープからの `pop` と `peek` は `None` を返します。
pub type MaxHeap<T> = BinaryHeap<T>;

/// 最小値を優先して取り出すヒープです。
///
/// [`push`](Self::push) と [`pop`](Self::pop) は `O(log N)`、[`peek`](Self::peek) は
/// `O(1)` です。
///
/// # Examples
///
/// ```
/// use atcoder::data_structure::heap::MinHeap;
///
/// let mut heap = MinHeap::new();
/// heap.push(3);
/// heap.push(1);
/// assert_eq!(heap.pop(), Some(1));
/// ```
#[derive(Debug, Clone)]
pub struct MinHeap<T>(BinaryHeap<Reverse<T>>);

impl<T: Ord> Default for MinHeap<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Ord> MinHeap<T> {
    /// 空の最小ヒープを作ります。計算量は `O(1)` です。
    pub fn new() -> Self {
        Self(BinaryHeap::new())
    }

    /// 要素を追加します。計算量は `O(log N)` です。
    pub fn push(&mut self, item: T) {
        self.0.push(Reverse(item));
    }

    /// 最小要素を削除して返します。空なら `None` です。計算量は `O(log N)` です。
    pub fn pop(&mut self) -> Option<T> {
        self.0.pop().map(|Reverse(value)| value)
    }

    /// 最小要素への参照を返します。空なら `None` です。計算量は `O(1)` です。
    pub fn peek(&self) -> Option<&T> {
        self.0.peek().map(|Reverse(value)| value)
    }

    /// 要素数を返します。計算量は `O(1)` です。
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// 空なら `true` を返します。計算量は `O(1)` です。
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
