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
