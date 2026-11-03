# 数字の文字列や数字列を整数に変換したいなら、数値列の解析

## こういうときに使う

数字の文字列や数字列を整数に変換したい場合に、**数値列の解析** を用います。

## 考え方と使い方

ASCII数字などの列からオーバーフローを確認して `usize` を構築できます。 `DecimalDigitsExt::parse_usize()` を使い、変換できなければ `None` を処理します。

## 注意・計算量

負数や小数には対応しません。

**計算量**：桁数をDとして O(D)。

**対応API**：[`DecimalDigitsExt`](atcoder::string::DecimalDigitsExt)。
