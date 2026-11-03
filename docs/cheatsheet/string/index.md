# 文字列 — こういうときに何を使う？

問題文の条件から選べる、実装済みライブラリの逆引きです。

| こういうとき | 用いるアルゴリズム |
| --- | --- |
| 長い文字列から、指定したパターンの出現位置を全て探したい | [KMP法](crate::string::kmp_search) |
| 各位置から始まる文字列と、先頭からの共通部分の長さが知りたい | [Z-algorithm](crate::string::z_algorithm) |
| 各位置を中心とした最長回文や、回文部分列の長さを求めたい | [Manacher法](crate::string::palindrome) |
| 離れた2つの部分文字列が等しいか、何度も比較したい | [Rolling Hash](crate::string::rolling_hash) |
| 英小文字を0〜25の番号に対応させ、配列で数えたい | [英字と添字の変換](crate::string::alphabet_index) |
| 数字の文字列や数字列を整数に変換したい | [数値列の解析](crate::string::parse_digits) |

[チートシートのトップ](crate)
