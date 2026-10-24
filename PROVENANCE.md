# 競技プログラミング用コードの出所記録

## 対象と前提

この記録は、2026年10月24日の調査開始時点で、ABC・ARC・AGCで使う可能性がある再利用コードについて、作業ツリー、README、ソース内のコメント、参照可能なGit履歴を確認した結果です。基準はAtCoderの[生成AI対策ルール（2026年10月14日版）](https://info.atcoder.jp/entry/llm-rules-ja)、[ルール変更のお知らせ](https://atcoder.jp/posts/algorithm_rules_20261014_ja)、[利用規約](https://atcoder.jp/tos?lang=ja)です。

「UNKNOWN」は、確認できる証拠から判定できないことを表します。「UNKNOWN_ORIGIN」は、出所を特定できていないことを表します。コミットの著者表示やコードの書き方だけでは本人の著作やAIの使用を確定できないため、それらを根拠に「はい」とは判定していません。

## 再利用ライブラリ

各ファイルの外部出所とライセンスは、確認できる記録がないため `UNKNOWN` としています。Git履歴やコメントから外部コードのコピーを確認できたファイルはありませんが、これは外部由来でないことの証明ではありません。AIによる生成・改変の証拠も確認できないため、該当欄は `UNKNOWN` です。

| ファイル／モジュール | 自己著作の確認 | 外部由来の確認 | 外部出典URL | ライセンス | AI生成・改変の証拠 |
| --- | --- | --- | --- | --- | --- |
| `src/lib.rs` | UNKNOWN | UNKNOWN_ORIGIN | UNKNOWN | UNKNOWN | UNKNOWN |
| `src/lib/algorithm/mod.rs` | UNKNOWN | UNKNOWN_ORIGIN | UNKNOWN | UNKNOWN | UNKNOWN |
| `src/lib/algorithm/compress.rs` | UNKNOWN | UNKNOWN_ORIGIN | UNKNOWN | UNKNOWN | UNKNOWN |
| `src/lib/algorithm/rle.rs` | UNKNOWN | UNKNOWN_ORIGIN | UNKNOWN | UNKNOWN | UNKNOWN |
| `src/lib/algorithm/sort.rs` | UNKNOWN | UNKNOWN_ORIGIN | UNKNOWN | UNKNOWN | UNKNOWN |
| `src/lib/data_structure/mod.rs` | UNKNOWN | UNKNOWN_ORIGIN | UNKNOWN | UNKNOWN | UNKNOWN |
| `src/lib/data_structure/dual_segment_tree.rs` | UNKNOWN | UNKNOWN_ORIGIN | UNKNOWN | UNKNOWN | UNKNOWN |
| `src/lib/data_structure/heap.rs` | UNKNOWN | UNKNOWN_ORIGIN | UNKNOWN | UNKNOWN | UNKNOWN |
| `src/lib/data_structure/segment_tree.rs` | UNKNOWN | UNKNOWN_ORIGIN | UNKNOWN | UNKNOWN | UNKNOWN |
| `src/lib/data_structure/union_find.rs` | UNKNOWN | UNKNOWN_ORIGIN | UNKNOWN | UNKNOWN | UNKNOWN |
| `src/lib/debug.rs` | UNKNOWN | UNKNOWN_ORIGIN | UNKNOWN | UNKNOWN | UNKNOWN |
| `src/lib/floor_sum.rs` | UNKNOWN | UNKNOWN_ORIGIN | UNKNOWN | UNKNOWN | UNKNOWN |
| `src/lib/geometry.rs` | UNKNOWN | UNKNOWN_ORIGIN | UNKNOWN | UNKNOWN | UNKNOWN |
| `src/lib/graph/mod.rs` | UNKNOWN | UNKNOWN_ORIGIN | UNKNOWN | UNKNOWN | UNKNOWN |
| `src/lib/grid.rs` | UNKNOWN | UNKNOWN_ORIGIN | UNKNOWN | UNKNOWN | UNKNOWN |
| `src/lib/io.rs` | UNKNOWN | UNKNOWN_ORIGIN | UNKNOWN | UNKNOWN | UNKNOWN |
| `src/lib/math/mod.rs` | UNKNOWN | UNKNOWN_ORIGIN | UNKNOWN | UNKNOWN | UNKNOWN |
| `src/lib/math/combinatorics.rs` | UNKNOWN | UNKNOWN_ORIGIN | UNKNOWN | UNKNOWN | UNKNOWN |
| `src/lib/math/fast_math.rs` | UNKNOWN | UNKNOWN_ORIGIN | UNKNOWN | UNKNOWN | UNKNOWN |
| `src/lib/math/integer.rs` | UNKNOWN | UNKNOWN_ORIGIN | UNKNOWN | UNKNOWN | UNKNOWN |
| `src/lib/math/mod_int.rs` | UNKNOWN | UNKNOWN_ORIGIN | UNKNOWN | UNKNOWN | UNKNOWN |
| `src/lib/math/radix.rs` | UNKNOWN | UNKNOWN_ORIGIN | UNKNOWN | UNKNOWN | UNKNOWN |
| `src/lib/math/saturating_usize.rs` | UNKNOWN | UNKNOWN_ORIGIN | UNKNOWN | UNKNOWN | UNKNOWN |
| `src/lib/math/sieve.rs` | UNKNOWN | UNKNOWN_ORIGIN | UNKNOWN | UNKNOWN | UNKNOWN |
| `src/lib/random.rs` | UNKNOWN | UNKNOWN_ORIGIN | UNKNOWN | UNKNOWN | UNKNOWN |
| `src/lib/string.rs` | UNKNOWN | UNKNOWN_ORIGIN | UNKNOWN | UNKNOWN | UNKNOWN |
| `src/lib/types.rs` | UNKNOWN | UNKNOWN_ORIGIN | UNKNOWN | UNKNOWN | UNKNOWN |

## 2026年10月24日追記：`geometry::argcmp`

`src/main.rs`にあった偏角の比較関数を、比較処理を変えずに`src/lib/geometry.rs`へ移設しました。
移設前のコメントに記載されていた[ngtkanaさんの記事](https://blog.ngtkana.com/entry/2021/11/13/202103)で、
半平面による分類と座標の積による比較を確認し、関数のドキュメントにも参考元を残しています。

参照した記事ではコードのライセンス表記を確認できなかったため、利用許諾の確認状況は`UNKNOWN`です。
今回の移設とドキュメント整備には生成AIを使用しています。移設前の実装に生成AIが使われたかは`UNKNOWN`です。
この記録は上表の調査開始時点の記録に対する追記であり、geometry内の他の関数の出所を確定するものではありません。

ABC・ARC・AGCの開催中に利用する場合は、[20261014版の公式ルール](https://info.atcoder.jp/entry/llm-rules-ja)に従い、
この変更を含む版をコンテスト開始前に公開し、提出コード内に参照URLを含める必要があります。

## Python エントリーファイル

`main.py` は共有ライブラリではなく、問題ごとの解答を記述するためのエントリーテンプレートとして別に記録します。現状の内容と履歴から、自己著作、外部出所、ライセンス、AIによる生成・改変はいずれも確認できません。

| ファイル | 区分 | 自己著作の確認 | 外部由来の確認 | 外部出典URL | ライセンス | AI生成・改変の証拠 |
| --- | --- | --- | --- | --- | --- | --- |
| `main.py` | 問題別解答用テンプレート | UNKNOWN | UNKNOWN_ORIGIN | UNKNOWN | UNKNOWN | UNKNOWN |

## 確認した履歴と限界

- 調査開始時点のローカル履歴は118コミットです。公開リモートの`master`は[`7577e5b08664aaf3bfd4486f13aeef5f7af5ed05`](https://github.com/Twil3akine/AtCoder-Algo/commit/7577e5b08664aaf3bfd4486f13aeef5f7af5ed05)で、ローカルHEADは`ef7677a2e715c7b110478c8e6172954dc6710ec5`です。後者には`src/lib/types.rs`の`B = bool`追加があり、今回の匿名GitHub API確認ではHTTP 422となりました。これを公開済みの根拠には使いません。
- 上記公開コミットには、GitHub側の[push起因のActions記録](https://github.com/Twil3akine/AtCoder-Algo/actions/runs/31891305113)があり、作成時刻は2026年8月15日23:56:26 JSTです。これはGitHub側に存在した記録であり、当時の公開設定を証明するものではありません。publicへ切り替えた時刻は`UNKNOWN`です。
- 2026年7月19日の[`97f0f0531208706d91346a5e939e22c4a23394bf`](https://github.com/Twil3akine/AtCoder-Algo/commit/97f0f0531208706d91346a5e939e22c4a23394bf)はライブラリの分割を記録していますが、分割以前の実装の作者や由来を証明するものではありません。
- [Claudeの共同著者表示がある変更](https://github.com/Twil3akine/AtCoder-Algo/commit/753e3a946400fa0410f859bd0026683ec391cef0)を含む10コミットはrunnerまたはNix関連で、ここに記録した提出用ライブラリの作者・AI利用を示す証拠にはなりません。
- README、コメント、参照可能な履歴から、提出用ライブラリに含まれる外部コピーの出典は確認できませんでした。確認範囲外の資料や履歴がある可能性は残ります。
- Cargo の通常の依存crateは、リポジトリにコピーされた実装とは分けて扱い、この表には含めていません。依存関係のライセンス評価もこの記録の対象外です。
- `src/main.rs` は作業時点で未コミット変更があるため編集せず、この再利用ライブラリの登録対象にも含めていません。
