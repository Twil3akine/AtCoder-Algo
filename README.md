# AtCoder-Algo

AtCoder / Codeforces向けのRust・Python・PyPy開発環境と、ブラウザから利用できる
共通local runnerです。Nix flakeと
[direnv](https://direnv.net/) / [nix-direnv](https://github.com/nix-community/nix-direnv)
を利用します。

## 初回セットアップ

Nixが導入済みでflakesが有効になっていることを前提とします。macOS / Linux共通で
direnvとnix-direnvを導入できます。

```sh
nix profile install nixpkgs#direnv nixpkgs#nix-direnv
mkdir -p ~/.config/direnv
echo 'source $HOME/.nix-profile/share/nix-direnv/direnvrc' >> ~/.config/direnv/direnvrc
```

[direnvのshell hook](https://direnv.net/docs/hook.html)をシェル設定へ追加してください。

```sh
# ~/.zshrc
eval "$(direnv hook zsh)"

# ~/.bashrc
eval "$(direnv hook bash)"
```

リポジトリでは初回だけ`.envrc`を許可します。

```sh
cd AtCoder-Algo
direnv allow
```

以後は`cd AtCoder-Algo`だけで`devShells.default`（AtCoder環境）がロードされ、共通runnerが
<http://127.0.0.1:4000>で起動します。AtCoder / Codeforcesの切替にrunner再起動は不要です。

## profile

runnerはAPIリクエストの`profile`ごとに、独立したCargo環境を使用します。

```text
~/.cache/atcoder-runner/atcoder/rust
~/.cache/atcoder-runner/codeforces/rust
```

profileの選択順はAPIの`profile`、`RUNNER_PROFILE`、`atcoder`の順です。実行用依存関係は
`runner/profiles/atcoder`と`runner/profiles/codeforces`で管理します。AtCoder profileは
[現行の公式Rust環境](https://img.atcoder.jp/file/language-update/2025-10/language-list.html)
に合わせ、Rust 1.89.0、`itertools 0.14.0`、`rand 0.9.2`を使用します。Codeforces profileは
外部crateなしです。

補助的に用途別devShellも利用できます。

```sh
nix develop .#atcoder
nix develop .#codeforces
```

runnerを停止する場合は次を実行します。

```sh
runner-stop
```

ログは`/tmp/atcoder-runner.log`です。APIの詳細は
[docs/runner-api.md](docs/runner-api.md)を参照してください。

## Rust競プロライブラリ

`src/main.rs`には問題固有の解答だけを書き、共通実装は`atcoder`ライブラリから読み込みます。

```rust
use atcoder::data_structure::union_find::UnionFind;
use atcoder::geometry::{segments_intersect, Point};
use atcoder::io::Writer;

fn main() {
    atcoder::input! { n: usize }
    let mut dsu = UnionFind::new(n);
    // ...
}
```

ライブラリは次の単位で分割されています。カテゴリ内も実装単位のファイルに分かれているため、
提出時に使っていないセグメント木などはバンドルされません。

```text
src/
├── lib.rs                         公開モジュールの入口
├── main.rs                        問題固有の解答
└── lib/
    ├── algorithm/                 座標圧縮・RLE・ソート
    ├── data_structure/            DSU・ヒープ・セグメント木
    ├── graph/                     グラフアルゴリズム
    ├── math/                      整数演算・ModInt
    ├── debug.rs                   debug!マクロ
    ├── geometry.rs                2次元整数幾何
    ├── grid.rs                    グリッドの範囲・近傍
    ├── io.rs                      input!・Writer
    ├── random.rs                  Xorshift
    └── string.rs                  英字変換・Manacher法
```

APIドキュメントは次のコマンドで生成できます。

```sh
cargo doc --no-deps --open
```

## 提出用バンドル

`cargo bundle`は`src/main.rs`をRustの構文木として解析し、`atcoder::...`で実際に参照した
自作モジュールと、そのモジュールが依存する自作モジュールだけを再帰的に収集します。明示的に
項目を指定して読み込んだ場合は、同じモジュール内の未使用項目も除外します。型、トレイト、
マクロなど依存先を構文だけでは特定できない項目は、安全のため関連する実装をまとめて残します。
正規表現でRust構文を置換せず、`syn`で解析・変換しているため、グループ化された`use`や
マクロ呼び出しも扱えます。

```sh
cargo bundle
```

生成コードは`target/bundle.rs`へ保存されます。ソースコメントとrustdocコメントを保ったまま
`#[cfg(test)]`の項目を除去して単一ファイルへ展開し、現在のpackageと同じedition・依存関係で
`rustfmt`による整形と`cargo check`を行います。
検証成功時だけmacOSの`pbcopy`でクリップボードへコピーします。

```text
Bundled 2 modules:
geometry
io

Bundle size: 12.3 KiB -> 8.1 KiB

✓ cargo check succeeded
✓ copied to clipboard
✓ wrote target/bundle.rs
```

クリップボードを変更せず検証だけ行う場合や、出力先を変える場合は次を使います。

```sh
cargo bundle --no-clipboard
cargo bundle --output /tmp/submission.rs
```

### ABC・ARC・AGCの事前作成コードへの参照

最終的な提出文字列の1行目には、参照URLのコメントを必ず追加します。保存するファイルと
クリップボードには同じ文字列を渡します。

```rust
// https://github.com/Twil3akine/AtCoder-Algo/commit/<40文字のSHA>
```

既定の参照はローカルの`origin/master`を固定SHAへ解決したものです。コンテスト前に
公開した版を指定する場合は、完全なSHAを渡してください。

```sh
cargo bundle --reference-commit <40文字のSHA> --no-clipboard
```

バンドラーはGitとcurlを使い、次の両方を確認してから出力します。

- `src/lib.rs`と、今回展開する各ライブラリファイルの内容が参照コミットと一致すること。
  ファイル単位で照合するため、未使用項目やコメントだけの変更でも不一致になります。
  `src/main.rs`の問題固有コードは照合対象に含めません。
- 指定SHAを、このリポジトリのGitHub APIから認証なしで取得できること。
  ローカルコミットの存在や`origin/master`だけでは公開済みと認定しません。

GitHubへの接続失敗、API制限、参照不能、ライブラリの不一致では
`pre-contest reference is not confirmed as publicly available`というエラーで停止し、
既存の出力ファイルとクリップボードを更新しません。通常のCargo依存はソースへ展開しません。

この確認で分かるのは実行時点の公開状態です。コンテスト開始前の公開を自動証明する機能は
ありません。利用するライブラリは開始前に公開し、第三者が内容と公開時刻を確認できる記録を
用意してください。コミット日時だけを公開時刻の証明には使えません。
[AtCoder公式ルール（20261014版）](https://info.atcoder.jp/entry/llm-rules-ja)と
[変更告知](https://atcoder.jp/posts/algorithm_rules_20261014_ja)に基づく対応です。

由来とライセンスの確認状況は[PROVENANCE.md](PROVENANCE.md)に記録しています。
参照URLの付加は、出所不明コードの利用許諾や、必要な著作権表示の保持を代替しません。

バンドラーはローカルpackageのソースを直接処理します。ブラウザのUserscriptがrunnerへ送る
単一ファイルの`sourceCode`形式とrunner APIには変更を加えていないため、既存のAtCoder / Codeforces
実行フローはそのまま利用できます。

## AtCoder Userscript

[userscripts/atcoder-local-runner.user.js](userscripts/atcoder-local-runner.user.js)を
TampermonkeyまたはViolentmonkeyへ追加してください。AtCoderの提出ボタンの横に`実行`と
`実行して提出`、その下にサンプル結果とカスタムテストを追加します。`実行して提出`では
全サンプルがACならそのまま提出します。runnerへは常に
`profile=atcoder`を指定します。

AtCoder Easy Test v2とはUIとキーボード操作が重複するため、置き換える場合はAtCoder Easy Test v2を
無効化してください。

## Codeforces Userscript

[userscripts/codeforces-local-runner.user.js](userscripts/codeforces-local-runner.user.js)を
TampermonkeyまたはViolentmonkeyへ追加してください。次の問題ページにコード入力欄、サンプル実行、
カスタムテスト、結果表示、Submit導線を追加します。`Run Samples`で全サンプルがACになると、
Codeforcesの提出ページへ移動して、選択中の言語とコードをそのまま自動提出します。

```text
https://codeforces.com/problemset/problem/*/*
https://codeforces.com/contest/*/problem/*
```

コードと言語選択はcontest ID・problem index・language単位でlocalStorageへ保存されます。
