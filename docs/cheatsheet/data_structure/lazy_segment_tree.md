# 区間に一括加算・代入しながら、区間和や区間最小値も求めたいなら、遅延評価セグメント木

## こういうときに使う

区間に一括加算・代入しながら、区間和や区間最小値も求めたい場合に、**遅延評価セグメント木** を用います。

## 考え方と使い方

区間全体への操作を遅延伝播させ、集約結果も保持できます。 集約 `op`、更新の作用 `mapping`、作用の合成 `composition` を指定し、`apply(l, r, action)` と `get(l, r)` を使います。

## 注意・計算量

合成順序と区間長を考慮した `mapping` が必要です。

**計算量**：構築 O(N)、区間更新・取得 O(log N)。

**対応API**：[`LazySegmentTree`](atcoder::data_structure::lazy_segment_tree::LazySegmentTree)。
