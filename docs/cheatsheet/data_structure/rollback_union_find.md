# 探索で仮に辺を追加し、前の連結状態へ戻したいなら、Rollback Union-Find

## こういうときに使う

探索で仮に辺を追加し、前の連結状態へ戻したい場合に、**Rollback Union-Find** を用います。

## 考え方と使い方

併合履歴を持ち、過去の状態まで戻せます。 分岐前に `snapshot()` を保存し、試行中に `merge()`、戻るときに `rollback(snapshot)` を実行します。

## 注意・計算量

辺の削除クエリをそのままオンラインで処理するものではなく、オフライン分割統治などと組み合わせます。

**計算量**：併合・検索 O(log N)、1操作の取り消し O(1)。

**対応API**：[`RollbackUnionFind`](atcoder::data_structure::rollback_union_find::RollbackUnionFind)。
