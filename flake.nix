{
  description = "AtCoder / Codeforces development environments + shared local runner";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };

        mkRustToolchain = channel:
          channel.default.override {
            extensions = [ "rust-src" "rust-analyzer" "clippy" "rustfmt" ];
          };

        # AtCoderの2025/10ジャッジ更新（Rust 1.89.0）に合わせる。
        atcoderRustToolchain = mkRustToolchain pkgs.rust-bin.stable."1.89.0";
        # Codeforces側は将来バージョンを固定しやすいよう独立定義にする。
        codeforcesRustToolchain = mkRustToolchain pkgs.rust-bin.stable.latest;

        runnerBin = pkgs.rustPlatform.buildRustPackage {
          pname = "runner";
          version = "0.2.0";
          src = ./runner;
          cargoLock.lockFile = ./runner/Cargo.lock;
        };

        # 1つのrunnerからprofileごとに異なるRustツールチェーンを呼び分ける。
        runner = pkgs.writeShellScriptBin "runner" ''
          export RUNNER_PROFILE="''${RUNNER_PROFILE:-atcoder}"
          export RUNNER_ATCODER_CARGO="${atcoderRustToolchain}/bin/cargo"
          export RUNNER_ATCODER_RUSTC="${atcoderRustToolchain}/bin/rustc"
          export RUNNER_CODEFORCES_CARGO="${codeforcesRustToolchain}/bin/cargo"
          export RUNNER_CODEFORCES_RUSTC="${codeforcesRustToolchain}/bin/rustc"
          export PATH="${pkgs.python3}/bin:${pkgs.pypy3}/bin:$PATH"
          exec ${runnerBin}/bin/runner "$@"
        '';

        runnerStop = pkgs.writeShellScriptBin "runner-stop" ''
          PORT="''${RUNNER_PORT:-4000}"
          PID=$(${pkgs.lsof}/bin/lsof -iTCP:"$PORT" -sTCP:LISTEN -t 2>/dev/null | tr '\n' ' ')
          if [ -n "$PID" ]; then
            kill $PID
            echo "Runner stopped on port $PORT (PID $PID)"
          else
            echo "Runner is not running on port $PORT"
          fi
        '';

        commonPackages = [
          pkgs.python3
          pkgs.pypy3
          pkgs.lsof
          runner
          runnerStop
        ];

        mkDevShell = { profile, rustToolchain, extraPackages ? [ ] }:
          pkgs.mkShell {
            packages = [ rustToolchain ] ++ commonPackages ++ extraPackages;
            RUNNER_PROFILE = profile;
            RUNNER_PORT = "4000";

            shellHook = ''
              if ${pkgs.lsof}/bin/lsof -iTCP:"$RUNNER_PORT" -sTCP:LISTEN -t >/dev/null 2>&1; then
                echo "Local Runner already running on http://127.0.0.1:$RUNNER_PORT"
              else
                runner > /tmp/atcoder-runner.log 2>&1 &
                echo "Shared Local Runner started on http://127.0.0.1:$RUNNER_PORT"
              fi
            '';
          };

        atcoderShell = mkDevShell {
          profile = "atcoder";
          rustToolchain = atcoderRustToolchain;
          extraPackages = [ pkgs.cargo-watch ];
        };

        codeforcesShell = mkDevShell {
          profile = "codeforces";
          rustToolchain = codeforcesRustToolchain;
        };
      in {
        packages.runner = runner;

        devShells = {
          default = atcoderShell;
          atcoder = atcoderShell;
          codeforces = codeforcesShell;
        };
      }
    );
}
