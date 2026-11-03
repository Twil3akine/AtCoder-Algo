# 各位置を中心とした最長回文や、回文部分列の長さを求めたいなら、Manacher法

## こういうときに使う

各位置を中心とした最長回文や、回文部分列の長さを求めたい場合に、**Manacher法** を用います。

## 考え方と使い方

中心を動かす際に、既知の回文半径を使い回せます。 `manacher(sequence)` で奇数長と偶数長の中心を含む回文半径列を得ます。単発の区間回文判定なら `SequenceExt::is_palindrome()` も使えます。

## 注意・計算量

返り値の添字は区切り位置を含む長さ `2N+1` の変換済み列に対応します。回文長の読み替えに注意します。

**計算量**：Manacher O(N)、単発区間判定 O(区間長)。

**対応API**：[`manacher`](atcoder::string::manacher)、[`SequenceExt`](atcoder::string::SequenceExt)。
