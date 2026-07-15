# Local runner API

local runnerは`http://127.0.0.1:4000`で待ち受けます。既存互換の`POST /`に加えて、
接続確認用の`GET /health`を提供します。

## Health

```http
GET /health
```

レスポンス例:

```json
{
  "status": "ok",
  "defaultProfile": "atcoder",
  "profiles": ["atcoder", "codeforces"],
  "versions": {
    "rust": {
      "atcoder": "1.89.0",
      "codeforces": "1.x.x"
    },
    "python": "3.x.x",
    "pypy": "3.x.x"
  }
}
```

## コンパイラ一覧

既存APIとの互換性を維持しています。

```json
{ "mode": "list" }
```

## コード実行

```json
{
  "mode": "run",
  "profile": "codeforces",
  "compilerName": "rust",
  "sourceCode": "fn main() { println!(\"hello\"); }",
  "stdin": ""
}
```

`profile`は`atcoder`または`codeforces`です。省略時はrunner起動時の
`RUNNER_PROFILE`、それも未指定なら`atcoder`を使用します。`compilerName`は
`rust`、`python`、`pypy`に対応します。

成功レスポンス例:

```json
{
  "status": "ok",
  "profile": "codeforces",
  "exitCode": 0,
  "time": 12,
  "stdout": "hello\n",
  "stderr": ""
}
```

`status`は次のいずれかです。

| status | 意味 |
| --- | --- |
| `ok` | 正常終了 |
| `compileError` | コンパイル・構文エラー |
| `runtimeError` | 非ゼロ終了 |
| `timeLimitExceeded` | 実行時間超過 |
| `internalError` | runner内部エラー |

サンプルのAC / WA判定はクライアント側でExpectedと`stdout`を比較して行います。
