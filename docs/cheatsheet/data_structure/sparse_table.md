# 配列が更新されず、区間の最小値・最大値を大量に求めたいなら、Sparse Table

## こういうときに使う

配列が更新されず、区間の最小値・最大値を大量に求めたい場合に、**Sparse Table** を用います。

## 考え方と使い方

変更されない配列なら、長さが2の累乗の区間結果を組み合わせて定数時間で答えられます。 `SparseTable::new(values, |a, b| a.min(b))` などで構築し、`get(l, r)` を呼びます。

## 注意・計算量

演算には結合則とべき等性（`op(x, x) = x`）が必要です。一般の区間和には使えません。

**計算量**：前計算 O(N log N)、各問い合わせ O(1)。

**対応API**：[`SparseTable`](atcoder::data_structure::sparse_table::SparseTable)。
