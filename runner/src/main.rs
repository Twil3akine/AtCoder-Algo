use std::collections::{BTreeMap, HashMap};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, Instant};

use axum::{
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tempfile::tempdir;
use tokio::io::AsyncWriteExt;
use tokio::process::Command;
use tokio::sync::Mutex;
use tokio::time::timeout;
use tower_http::cors::{Any, CorsLayer};

const ATCODER_CARGO_TOML: &str = include_str!("../profiles/atcoder/Cargo.toml");
const ATCODER_CARGO_LOCK: &str = include_str!("../profiles/atcoder/Cargo.lock");
const CODEFORCES_CARGO_TOML: &str = include_str!("../profiles/codeforces/Cargo.toml");
const CODEFORCES_CARGO_LOCK: &str = include_str!("../profiles/codeforces/Cargo.lock");

const COMPILE_TIMEOUT: Duration = Duration::from_secs(60);
const RUN_TIMEOUT: Duration = Duration::from_secs(10);
const MAX_OUTPUT: usize = 1024 * 1024;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
enum Profile {
    Atcoder,
    Codeforces,
}

impl Profile {
    const ALL: [Self; 2] = [Self::Atcoder, Self::Codeforces];

    fn as_str(self) -> &'static str {
        match self {
            Self::Atcoder => "atcoder",
            Self::Codeforces => "codeforces",
        }
    }

    fn env_prefix(self) -> &'static str {
        match self {
            Self::Atcoder => "ATCODER",
            Self::Codeforces => "CODEFORCES",
        }
    }

    fn parse(value: &str) -> Option<Self> {
        match value.to_ascii_lowercase().as_str() {
            "atcoder" => Some(Self::Atcoder),
            "codeforces" => Some(Self::Codeforces),
            _ => None,
        }
    }

    fn manifest(self) -> &'static str {
        match self {
            Self::Atcoder => ATCODER_CARGO_TOML,
            Self::Codeforces => CODEFORCES_CARGO_TOML,
        }
    }

    fn lockfile(self) -> &'static str {
        match self {
            Self::Atcoder => ATCODER_CARGO_LOCK,
            Self::Codeforces => CODEFORCES_CARGO_LOCK,
        }
    }
}

#[derive(Deserialize)]
#[serde(tag = "mode", rename_all = "lowercase")]
enum Request {
    List,
    Run {
        #[serde(default)]
        profile: Option<Profile>,
        #[serde(rename = "compilerName")]
        compiler_name: String,
        #[serde(rename = "sourceCode")]
        source_code: String,
        stdin: String,
    },
}

#[derive(Serialize)]
struct CompilerInfo {
    language: String,
    #[serde(rename = "compilerName")]
    compiler_name: String,
    label: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct RunResponse {
    status: String,
    profile: Profile,
    #[serde(skip_serializing_if = "Option::is_none")]
    exit_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memory: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stdout: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stderr: Option<String>,
}

#[derive(Clone, Serialize)]
struct Versions {
    rust: BTreeMap<String, String>,
    python: String,
    pypy: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct HealthResponse {
    status: &'static str,
    default_profile: Profile,
    profiles: [Profile; 2],
    versions: Versions,
}

struct RustProject {
    path: PathBuf,
    cargo: String,
    rustc: String,
    lock: Mutex<()>,
}

#[derive(Clone)]
struct AppState {
    versions: Arc<Versions>,
    projects: Arc<HashMap<Profile, Arc<RustProject>>>,
    default_profile: Profile,
}

impl AppState {
    fn project(&self, profile: Profile) -> &Arc<RustProject> {
        self.projects
            .get(&profile)
            .expect("all supported profiles are initialized")
    }
}

fn profile_tool(profile: Profile, tool: &str) -> String {
    let key = format!(
        "RUNNER_{}_{}",
        profile.env_prefix(),
        tool.to_ascii_uppercase()
    );
    std::env::var(key).unwrap_or_else(|_| tool.to_string())
}

fn cache_root() -> PathBuf {
    if let Some(path) = std::env::var_os("XDG_CACHE_HOME") {
        return PathBuf::from(path).join("atcoder-runner");
    }
    let home = std::env::var_os("HOME").unwrap_or_else(|| ".".into());
    PathBuf::from(home).join(".cache/atcoder-runner")
}

async fn setup_rust_project(profile: Profile) -> std::io::Result<RustProject> {
    let project_dir = cache_root().join(profile.as_str()).join("rust");
    let src_dir = project_dir.join("src");
    tokio::fs::create_dir_all(&src_dir).await?;
    tokio::fs::write(project_dir.join("Cargo.toml"), profile.manifest()).await?;
    tokio::fs::write(project_dir.join("Cargo.lock"), profile.lockfile()).await?;

    let main_rs = src_dir.join("main.rs");
    if !main_rs.exists() {
        tokio::fs::write(main_rs, "fn main() {}\n").await?;
    }

    Ok(RustProject {
        path: project_dir,
        cargo: profile_tool(profile, "cargo"),
        rustc: profile_tool(profile, "rustc"),
        lock: Mutex::new(()),
    })
}

async fn detect_version(command: &str, args: &[&str]) -> String {
    let Ok(output) = Command::new(command).args(args).output().await else {
        return "?".into();
    };
    let raw = if output.stdout.is_empty() {
        String::from_utf8_lossy(&output.stderr).into_owned()
    } else {
        String::from_utf8_lossy(&output.stdout).into_owned()
    };
    let first = raw.lines().next().unwrap_or("?").trim().to_string();
    first
        .split_whitespace()
        .nth(1)
        .unwrap_or(&first)
        .to_string()
}

impl Versions {
    async fn detect(projects: &HashMap<Profile, Arc<RustProject>>) -> Self {
        let mut rust = BTreeMap::new();
        for profile in Profile::ALL {
            let project = projects.get(&profile).unwrap();
            rust.insert(
                profile.as_str().to_string(),
                detect_version(&project.rustc, &["--version"]).await,
            );
        }
        Self {
            rust,
            python: detect_version("python3", &["--version"]).await,
            pypy: detect_version("pypy3", &["--version"]).await,
        }
    }
}

async fn health(
    axum::extract::State(state): axum::extract::State<AppState>,
) -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        default_profile: state.default_profile,
        profiles: Profile::ALL,
        versions: (*state.versions).clone(),
    })
}

async fn handle(
    axum::extract::State(state): axum::extract::State<AppState>,
    Json(request): Json<Request>,
) -> Json<serde_json::Value> {
    match request {
        Request::List => {
            let rust_version = state
                .versions
                .rust
                .get(state.default_profile.as_str())
                .map(String::as_str)
                .unwrap_or("?");
            let list = vec![
                CompilerInfo {
                    language: "Rust".into(),
                    compiler_name: "rust".into(),
                    label: format!("Rust ({rust_version})"),
                },
                CompilerInfo {
                    language: "Python3".into(),
                    compiler_name: "python".into(),
                    label: format!("Python (CPython {})", state.versions.python),
                },
                CompilerInfo {
                    language: "Python3".into(),
                    compiler_name: "pypy".into(),
                    label: format!("Python (PyPy {})", state.versions.pypy),
                },
            ];
            Json(serde_json::to_value(list).unwrap())
        }
        Request::Run {
            profile,
            compiler_name,
            source_code,
            stdin,
        } => {
            let profile = profile.unwrap_or(state.default_profile);
            let started = Instant::now();
            let response = run(&compiler_name, &source_code, &stdin, profile, &state).await;
            eprintln!(
                "[run] profile={} compiler={} status={} time={}ms exit_code={:?}",
                profile.as_str(),
                compiler_name,
                response.status,
                started.elapsed().as_millis(),
                response.exit_code,
            );
            Json(serde_json::to_value(response).unwrap())
        }
    }
}

async fn run(
    compiler_name: &str,
    source_code: &str,
    stdin: &str,
    profile: Profile,
    state: &AppState,
) -> RunResponse {
    match compiler_name {
        "rust" => run_rust(source_code, stdin, profile, state).await,
        "python" | "pypy" => {
            let interpreter = if compiler_name == "python" {
                "python3"
            } else {
                "pypy3"
            };
            let dir = match tempdir() {
                Ok(dir) => dir,
                Err(error) => return internal_error(profile, format!("tempdir: {error}")),
            };
            run_interpreted(interpreter, source_code, stdin, dir, profile).await
        }
        other => internal_error(profile, format!("unknown compilerName: {other}")),
    }
}

async fn run_rust(
    source_code: &str,
    stdin: &str,
    profile: Profile,
    state: &AppState,
) -> RunResponse {
    let project = state.project(profile);
    let _guard = project.lock.lock().await;

    if let Err(error) = tokio::fs::write(project.path.join("src/main.rs"), source_code).await {
        return internal_error(profile, format!("write source: {error}"));
    }

    let mut command = Command::new(&project.cargo);
    command
        .args(["build", "--release", "--locked"])
        .env("RUSTC", &project.rustc)
        .current_dir(&project.path)
        .kill_on_drop(true);

    match timeout(COMPILE_TIMEOUT, command.output()).await {
        Err(_) => return compile_error(profile, "コンパイルがタイムアウトしました"),
        Ok(Err(error)) => {
            return internal_error(profile, format!("cargo 起動失敗: {error}"));
        }
        Ok(Ok(output)) if !output.status.success() => {
            return compile_error(profile, String::from_utf8_lossy(&output.stderr).trim());
        }
        Ok(Ok(_)) => {}
    }

    execute(
        &project.path.join("target/release/solution"),
        &[],
        stdin,
        profile,
    )
    .await
}

async fn run_interpreted(
    interpreter: &str,
    source_code: &str,
    stdin: &str,
    dir: tempfile::TempDir,
    profile: Profile,
) -> RunResponse {
    let source = dir.path().join("solution.py");
    if let Err(error) = tokio::fs::write(&source, source_code).await {
        return internal_error(profile, format!("write source: {error}"));
    }

    let mut command = Command::new(interpreter);
    command
        .args(["-m", "py_compile", source.to_str().unwrap()])
        .kill_on_drop(true);
    match timeout(Duration::from_secs(10), command.output()).await {
        Err(_) => return compile_error(profile, "構文チェックがタイムアウトしました"),
        Ok(Err(error)) => {
            return internal_error(profile, format!("{interpreter} 起動失敗: {error}"));
        }
        Ok(Ok(output)) if !output.status.success() => {
            return compile_error(profile, String::from_utf8_lossy(&output.stderr).trim());
        }
        Ok(Ok(_)) => {}
    }

    execute(
        Path::new(interpreter),
        &[source.to_str().unwrap()],
        stdin,
        profile,
    )
    .await
}

async fn execute(command: &Path, args: &[&str], stdin_data: &str, profile: Profile) -> RunResponse {
    let mut process = Command::new(command);
    process
        .args(args)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .kill_on_drop(true);

    let mut child = match process.spawn() {
        Ok(child) => child,
        Err(error) => {
            return internal_error(profile, format!("実行ファイル起動失敗: {error}"));
        }
    };

    if let Some(mut child_stdin) = child.stdin.take() {
        if let Err(error) = child_stdin.write_all(stdin_data.as_bytes()).await {
            return internal_error(profile, format!("stdin書き込み失敗: {error}"));
        }
    }

    let started = Instant::now();
    match timeout(RUN_TIMEOUT, child.wait_with_output()).await {
        Err(_) => RunResponse {
            status: "timeLimitExceeded".into(),
            profile,
            exit_code: None,
            time: Some(RUN_TIMEOUT.as_millis() as u64),
            memory: None,
            stdout: None,
            stderr: Some("実行時間制限 (10s) を超えました".into()),
        },
        Ok(Err(error)) => internal_error(profile, format!("wait失敗: {error}")),
        Ok(Ok(output)) => {
            let status = if output.status.success() {
                "ok"
            } else {
                "runtimeError"
            };
            RunResponse {
                status: status.into(),
                profile,
                exit_code: output.status.code(),
                time: Some(started.elapsed().as_millis() as u64),
                memory: None,
                stdout: Some(truncate(
                    String::from_utf8_lossy(&output.stdout).into_owned(),
                )),
                stderr: Some(truncate(
                    String::from_utf8_lossy(&output.stderr).into_owned(),
                )),
            }
        }
    }
}

fn truncate(value: String) -> String {
    if value.len() <= MAX_OUTPUT {
        return value;
    }
    let mut end = MAX_OUTPUT;
    while !value.is_char_boundary(end) {
        end -= 1;
    }
    format!("{}...(出力が長すぎるため切り捨て)", &value[..end])
}

fn compile_error(profile: Profile, message: impl Into<String>) -> RunResponse {
    RunResponse {
        status: "compileError".into(),
        profile,
        exit_code: None,
        time: None,
        memory: None,
        stdout: None,
        stderr: Some(message.into()),
    }
}

fn internal_error(profile: Profile, message: impl Into<String>) -> RunResponse {
    RunResponse {
        status: "internalError".into(),
        profile,
        exit_code: None,
        time: None,
        memory: None,
        stdout: None,
        stderr: Some(message.into()),
    }
}

#[tokio::main]
async fn main() {
    let default_profile = std::env::var("RUNNER_PROFILE")
        .ok()
        .and_then(|value| Profile::parse(&value))
        .unwrap_or(Profile::Atcoder);
    let port = std::env::var("RUNNER_PORT")
        .ok()
        .and_then(|value| value.parse::<u16>().ok())
        .unwrap_or(4000);

    let address = format!("127.0.0.1:{port}");
    let listener = tokio::net::TcpListener::bind(&address)
        .await
        .unwrap_or_else(|error| {
            eprintln!("Failed to listen on http://{address}: {error}");
            std::process::exit(1);
        });

    let mut projects = HashMap::new();
    for profile in Profile::ALL {
        let project = setup_rust_project(profile).await.unwrap_or_else(|error| {
            eprintln!("Failed to initialize {} profile: {error}", profile.as_str());
            std::process::exit(1);
        });
        projects.insert(profile, Arc::new(project));
    }
    let versions = Arc::new(Versions::detect(&projects).await);

    println!("Local Runner");
    println!("  Default profile: {}", default_profile.as_str());
    for (profile, version) in &versions.rust {
        println!("  Rust ({profile}): {version}");
    }
    println!("  CPython: {}", versions.python);
    println!("  PyPy   : {}", versions.pypy);

    let state = AppState {
        versions,
        projects: Arc::new(projects),
        default_profile,
    };
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any)
        .allow_private_network(true);
    let app = Router::new()
        .route("/", post(handle))
        .route("/health", get(health))
        .layer(cors)
        .with_state(state);

    println!("Listening on http://{address}");
    axum::serve(listener, app).await.unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn profile_manifests_are_separated() {
        assert!(ATCODER_CARGO_TOML.contains("itertools = \"=0.14.0\""));
        assert!(ATCODER_CARGO_TOML.contains("rand = \"=0.9.2\""));
        assert!(!CODEFORCES_CARGO_TOML.contains("itertools"));
        assert!(!CODEFORCES_CARGO_TOML.contains("rand ="));
    }

    #[test]
    fn profile_names_are_parsed_case_insensitively() {
        assert_eq!(Profile::parse("atcoder"), Some(Profile::Atcoder));
        assert_eq!(Profile::parse("Codeforces"), Some(Profile::Codeforces));
        assert_eq!(Profile::parse("unknown"), None);
    }

    #[test]
    fn truncate_keeps_utf8_boundary() {
        let value = "あ".repeat(MAX_OUTPUT);
        let truncated = truncate(value);
        assert!(truncated.ends_with("...(出力が長すぎるため切り捨て)"));
    }
}
