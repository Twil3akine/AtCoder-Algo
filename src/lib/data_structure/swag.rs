//! キューの先頭削除・末尾追加・全体集約を償却O(1)で行うSWAGです。

/// 結合則を満たす演算に対応したスライディングウィンドウ集約です。
///
/// `op` は結合則を満たし、`identity` を単位元に持つ必要があります。
/// 非可換な演算にも対応します。各操作の償却計算量は `O(1)` です。
///
/// # Examples
///
/// ```
/// use atcoder::data_structure::swag::Swag;
///
/// let mut q = Swag::new(|a, b| a.min(b), i32::MAX);
/// q.push(3);
/// q.push(1);
/// q.push(4);
/// assert_eq!(q.fold(), 1);
/// assert_eq!(q.pop(), Some(3));
/// assert_eq!(q.fold(), 1);
/// ```
pub struct Swag<T, F> {
    front: Vec<(T, T)>,
    back: Vec<(T, T)>,
    op: F,
    identity: T,
}

impl<T: Clone, F: Fn(T, T) -> T> Swag<T, F> {
    /// 集約演算と単位元を指定して空のキューを作ります。
    pub fn new(op: F, identity: T) -> Self {
        Self {
            front: Vec::new(),
            back: Vec::new(),
            op,
            identity,
        }
    }

    /// キューの末尾に値を追加します。償却計算量は `O(1)` です。
    pub fn push(&mut self, value: T) {
        let aggregate = if let Some((_, prev)) = self.back.last() {
            (self.op)(prev.clone(), value.clone())
        } else {
            value.clone()
        };
        self.back.push((value, aggregate));
    }

    /// キューの先頭から値を取り除きます。空なら `None` です。
    ///
    /// 償却計算量は `O(1)` です。
    pub fn pop(&mut self) -> Option<T> {
        if self.front.is_empty() {
            while let Some((value, _)) = self.back.pop() {
                let aggregate = if let Some((_, prev)) = self.front.last() {
                    (self.op)(value.clone(), prev.clone())
                } else {
                    value.clone()
                };
                self.front.push((value, aggregate));
            }
        }
        self.front.pop().map(|(value, _)| value)
    }

    /// キュー全体の集約結果を返します。空なら単位元です。計算量は `O(1)` です。
    pub fn fold(&self) -> T {
        match (self.front.last(), self.back.last()) {
            (Some((_, left)), Some((_, right))) => (self.op)(left.clone(), right.clone()),
            (Some((_, left)), None) => left.clone(),
            (None, Some((_, right))) => right.clone(),
            (None, None) => self.identity.clone(),
        }
    }

    /// 保持する要素数を返します。
    pub fn len(&self) -> usize {
        self.front.len() + self.back.len()
    }

    /// キューが空なら `true` を返します。
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::Swag;

    #[test]
    fn ordered_noncommutative_aggregation() {
        let mut q = Swag::new(
            |mut a: String, b: String| {
                a.push_str(&b);
                a
            },
            String::new(),
        );
        assert_eq!(q.fold(), "");
        for s in ["a", "b", "c"] {
            q.push(s.to_string());
        }
        assert_eq!(q.fold(), "abc");
        assert_eq!(q.pop(), Some("a".to_string()));
        q.push("d".into());
        assert_eq!(q.fold(), "bcd");
        assert_eq!(q.pop(), Some("b".into()));
        assert_eq!(q.pop(), Some("c".into()));
        assert_eq!(q.pop(), Some("d".into()));
        assert!(q.pop().is_none());
    }
}
