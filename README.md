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
自作モジュールと、そのモジュールが依存する自作モジュールだけを再帰的に収集します。正規表現で
Rust構文を置換せず、`syn`で解析・変換しているため、グループ化された`use`やマクロ呼び出しも
扱えます。

```sh
cargo bundle
```

生成コードは`target/bundle.rs`へ保存されます。rustdocコメントと`#[cfg(test)]`の項目を除去し、
単一ファイルへ展開した後、現在のpackageと同じeditionを指定して`rustc`でコンパイルします。
コンパイル成功時だけmacOSの`pbcopy`でクリップボードへコピーします。

```text
Bundled 2 modules:
geometry
io

Bundle size: 12.3 KiB -> 8.1 KiB

✓ rustc compilation succeeded
✓ copied to clipboard
✓ wrote target/bundle.rs
```

クリップボードを変更せず検証だけ行う場合や、出力先を変える場合は次を使います。

```sh
cargo bundle --no-clipboard
cargo bundle --output /tmp/submission.rs
```

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
