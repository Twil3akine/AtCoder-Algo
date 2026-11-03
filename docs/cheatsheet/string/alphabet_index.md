# 英小文字を0〜25の番号に対応させ、配列で数えたいなら、英字と添字の変換

## こういうときに使う

英小文字を0〜25の番号に対応させ、配列で数えたい場合に、**英字と添字の変換** を用います。

## 考え方と使い方

英字を頻度配列やDPの添字として扱えます。 英字から添字には `AlphaExt::to_idx()`、添字から小文字へ戻すには `AlphaIndexExt::to_alpha()` を使います。

## 注意・計算量

ASCIIの英字が前提です。Unicode全体には使えません。

**計算量**：1文字 O(1)。

**対応API**：[`AlphaExt`](atcoder::string::AlphaExt)、[`AlphaIndexExt`](atcoder::string::AlphaIndexExt)。
