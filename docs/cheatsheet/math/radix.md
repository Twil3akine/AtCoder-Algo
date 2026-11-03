# 整数を2進数・16進数などの各桁へ分解したい、基数変換・桁列への変換

## こういうときに使う

**基数変換・桁列への変換**：整数を2進数・16進数などの各桁へ分解したい場合に適しています。

## 考え方と使い方

整数と任意基数の桁列を相互に変換できます。 `to_digits(value, base)` で桁列を得て、`from_digits(digits, base)` で数に戻します。0を使わない表記には `to_bijective_digits` を使います。

## 注意・計算量

`base` は2以上です。桁DP自体を行う関数ではありません。

**計算量**：桁数をDとして O(D)。

**対応API**：[`to_digits`](atcoder::math::radix::to_digits)、[`from_digits`](atcoder::math::radix::from_digits)、[`to_bijective_digits`](atcoder::math::radix::to_bijective_digits)。
