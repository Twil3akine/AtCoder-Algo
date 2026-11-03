# n個からr個を選ぶ方法や並べる方法を何度も数えたい、組合せ・順列の階乗前計算

## こういうときに使う

**組合せ・順列の階乗前計算**：n個からr個を選ぶ方法や並べる方法を何度も数えたい場合に適しています。

## 考え方と使い方

階乗と逆階乗を前計算し、各問い合わせを定数時間で処理します。 `Combinatorics::<998_244_353>::new(max_n)` を構築し、`combination(n, r)` や `permutation(n, r)` を呼びます。

## 注意・計算量

法 `MOD` は素数で、前計算範囲 `max_n` は `MOD` より小さくする必要があります。

**計算量**：構築 O(max_n + log MOD)、各問い合わせ O(1)。

**対応API**：[`Combinatorics`](atcoder::math::combinatorics::Combinatorics)。
