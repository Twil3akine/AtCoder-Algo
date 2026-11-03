# 個数の累積和から、k番目の要素がある位置を探したいなら、Fenwick Treeの累積頻度二分探索

## こういうときに使う

個数の累積和から、k番目の要素がある位置を探したい場合に、**Fenwick Treeの累積頻度二分探索** を用います。

## 考え方と使い方

各位置の個数を管理して、累積値が閾値に初めて達する位置を求められます。 個数をBITに加算し、`lower_bound(k)` で累積頻度が `k` 以上になる最初の位置を取得します。

## 注意・計算量

各位置の頻度が非負で、累積和が単調である必要があります。範囲外の `k` の扱いも確認してください。

**計算量**：更新・位置検索 O(log N)。

**対応API**：[`FenwickTree::lower_bound`](atcoder::data_structure::fenwick_tree::FenwickTree::lower_bound)。
