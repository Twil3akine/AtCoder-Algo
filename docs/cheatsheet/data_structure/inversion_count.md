# 数列の大小が逆転している添字対を数えたいなら、座標圧縮＋Fenwick Tree

## こういうときに使う

数列の大小が逆転している添字対を数えたい場合に、**座標圧縮＋Fenwick Tree** を用います。

## 考え方と使い方

左から順に走査して、自分より大きい既出要素の数を累積頻度から求めます。 値を座標圧縮し、各値の出現回数をBITで管理します。現在の値以上ではなく**真に大きい**既出要素を数えた後、現在の値を追加します。

## 注意・計算量

等しい値の組は転倒数に含めません。全組数が大きい場合は答えを `u64` や `i64` で持ちます。

**計算量**：O(N log N)。

**対応API**：[`Compress`](atcoder::algorithm::compress::Compress)、[`FenwickTree`](atcoder::data_structure::fenwick_tree::FenwickTree)。
