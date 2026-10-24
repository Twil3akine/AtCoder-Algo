#![cfg(unix)]

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use tempfile::TempDir;

const LIBRARY: &str = "\
//! Library inner docs.
// Root library comment.
// Comment before external module declaration.
/// Public helper docs.
pub mod helper;
// fn commented_out_library_item() {}
#[cfg(test)]
mod tests { fn test_only_marker() {} }
fn unused_library_item_marker() {}
";
const HELPER: &str = "\
//! Helper module inner docs.
/// Answer docs.
pub fn answer() -> i32 {
    // Function body comment.
    macro_rules! answer_value {
        // Comment inside macro definition.
        () => { /* block comment in macro body */ 42 };
    }
    answer_value!()
}
// fn commented_out_helper_item() {}
#[cfg(test)]
mod tests { fn test_only_helper_marker() {} }
fn unused_helper_item_marker() {}
";
const MAIN: &str = r####"
//! Submission docs.
// Main source comment.
use solution::helper::answer;

// fn commented_out_main_item() {}
fn main() {
    // Trailing-expression comment.
    let comment_markers = ("// string marker", r###"raw // string /* marker */"###);
    println!("{} {}", answer(), comment_markers.0);
}
// Final standalone main comment.
#[cfg(test)]
mod tests { fn test_only_main_marker() {} }
fn unused_main_item_marker() {}
"####;

struct Fixture {
    directory: TempDir,
    reference: String,
    bin_directory: PathBuf,
    clipboard: PathBuf,
}

impl Fixture {
    fn new() -> Self {
        let directory = tempfile::tempdir().unwrap();
        let root = directory.path();
        fs::create_dir_all(root.join("src")).unwrap();
        fs::write(
            root.join("Cargo.toml"),
            "[package]\nname = \"solution\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\n",
        )
        .unwrap();
        fs::write(root.join("src/lib.rs"), LIBRARY).unwrap();
        fs::write(root.join("src/helper.rs"), HELPER).unwrap();
        fs::write(root.join("src/main.rs"), MAIN).unwrap();

        git(root, &["init", "-q"]);
        git(root, &["config", "user.name", "Bundler tests"]);
        git(
            root,
            &["config", "user.email", "bundler-tests@example.invalid"],
        );
        git(
            root,
            &[
                "add",
                "Cargo.toml",
                "src/lib.rs",
                "src/helper.rs",
                "src/main.rs",
            ],
        );
        git(root, &["commit", "-qm", "reference"]);
        let reference = String::from_utf8(git(root, &["rev-parse", "HEAD"]).stdout)
            .unwrap()
            .trim()
            .to_owned();

        let bin_directory = root.join("mock-bin");
        fs::create_dir(&bin_directory).unwrap();
        executable(
            &bin_directory.join("curl"),
            "#!/bin/sh\n[ \"$1\" = --disable ] || exit 90\nprintf '%s\\n' \"$@\" > \"$MOCK_CURL_ARGUMENTS\"\n[ \"$MOCK_HTTP_CODE\" != network-error ] || exit 6\nprintf '%s' \"$MOCK_HTTP_CODE\"\n",
        );
        executable(
            &bin_directory.join("pbcopy"),
            "#!/bin/sh\ncat > \"$MOCK_CLIPBOARD_FILE\"\n",
        );

        Self {
            clipboard: root.join("clipboard.txt"),
            directory,
            reference,
            bin_directory,
        }
    }

    fn root(&self) -> &Path {
        self.directory.path()
    }

    fn run(&self, arguments: &[&str], http_code: &str) -> Output {
        let path = std::env::var_os("PATH").unwrap_or_default();
        let mut paths = vec![self.bin_directory.clone()];
        paths.extend(std::env::split_paths(&path));
        Command::new(env!("CARGO_BIN_EXE_cargo-bundle"))
            .current_dir(self.root())
            .args(arguments)
            .env("PATH", std::env::join_paths(paths).unwrap())
            .env("CARGO_NET_OFFLINE", "true")
            .env("MOCK_HTTP_CODE", http_code)
            .env(
                "MOCK_CURL_ARGUMENTS",
                self.root().join("curl-arguments.txt"),
            )
            .env("MOCK_CLIPBOARD_FILE", &self.clipboard)
            .output()
            .unwrap()
    }
}

fn executable(path: &Path, contents: &str) {
    fs::write(path, contents).unwrap();
    let mut permissions = fs::metadata(path).unwrap().permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(path, permissions).unwrap();
}

fn git(directory: &Path, arguments: &[&str]) -> Output {
    let output = Command::new("git")
        .arg("-C")
        .arg(directory)
        .args(arguments)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "git {arguments:?} failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    output
}

#[test]
fn bundles_verified_library_and_copies_the_header_and_code() {
    let fixture = Fixture::new();
    // The program can change after the reference commit; reusable library files cannot.
    fs::write(
        fixture.root().join("src/main.rs"),
        r####"//! Changed submission docs.
// Changed main comment.
use solution::helper::answer;

// fn commented_out_main_item() {}
fn main() {
    // Main body comment.
    let comment_markers = ("// string marker", r###"raw // string /* marker */"###);
    println!("updated {} {}", answer(), comment_markers.0);
}
// Changed trailing comment.
"####,
    )
    .unwrap();

    let output = fixture.run(&["--reference-commit", &fixture.reference], "200");
    assert!(
        output.status.success(),
        "bundle failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let bundled = fs::read(fixture.root().join("target/bundle.rs")).unwrap();
    let bundled = String::from_utf8(bundled).unwrap();
    let header = format!(
        "// https://github.com/Twil3akine/AtCoder-Algo/commit/{}\n\n",
        fixture.reference
    );
    assert!(bundled.starts_with(&header));
    assert!(bundled.contains("pub fn answer() -> i32"));
    assert!(bundled.contains("Public helper docs."));
    assert!(bundled.contains("Helper module inner docs."));
    assert!(bundled.contains("Comment before external module declaration."));
    assert!(bundled.contains("Answer docs."));
    assert!(bundled.contains("Function body comment."));
    assert!(bundled.contains("Comment inside macro definition."));
    assert!(bundled.contains("block comment in macro body"));
    assert!(bundled.contains("Changed submission docs."));
    assert!(bundled.contains("Changed main comment."));
    assert!(bundled.contains("Main body comment."));
    assert!(bundled.contains("Changed trailing comment."));
    assert!(bundled.contains("commented_out_main_item"));
    assert!(bundled.contains("commented_out_helper_item"));
    assert!(bundled.contains("commented_out_library_item"));
    assert!(bundled.contains("updated {}"));
    assert!(bundled.contains("// string marker"));
    assert!(bundled.contains("raw // string /* marker */"));
    for excluded in [
        "test_only_marker",
        "test_only_helper_marker",
        "test_only_main_marker",
        "unused_library_item_marker",
        "unused_helper_item_marker",
        "unused_main_item_marker",
    ] {
        assert!(
            !bundled.contains(excluded),
            "unexpected retained item: {excluded}"
        );
    }
    assert_eq!(fs::read(&fixture.clipboard).unwrap(), bundled.as_bytes());
    let request = fs::read_to_string(fixture.root().join("curl-arguments.txt")).unwrap();
    assert!(request.starts_with("--disable\n"));
    assert!(request.contains(&format!(
        "https://api.github.com/repos/Twil3akine/AtCoder-Algo/commits/{}\n",
        fixture.reference
    )));
}

#[test]
fn keeps_original_main_comments_after_bundled_modules() {
    let fixture = Fixture::new();
    fs::write(
        fixture.root().join("src/main.rs"),
        r#"//! Submission docs stay at the top.
#![allow(unused_imports)]
use solution::{helper::answer}; // grouped import comment

// Main function line comment.
/* Main function block comment. */
// fn commented_out_before_main() {}
fn main() {
    println!("{}", answer());
}
"#,
    )
    .unwrap();

    let output = fixture.run(
        &["--reference-commit", &fixture.reference, "--no-clipboard"],
        "200",
    );
    assert!(
        output.status.success(),
        "bundle failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let bundled = fs::read_to_string(fixture.root().join("target/bundle.rs")).unwrap();
    let docs = bundled.find("Submission docs stay at the top.").unwrap();
    let attribute = bundled.find("#![allow(unused_imports)]").unwrap();
    let import = bundled.find("use crate::helper::answer;").unwrap();
    let import_comment = bundled.find("// grouped import comment").unwrap();
    let helper_module = bundled.find("mod helper {").unwrap();
    let line_comment = bundled.find("// Main function line comment.").unwrap();
    let block_comment = bundled.find("/* Main function block comment. */").unwrap();
    let commented_out_code = bundled
        .find("// fn commented_out_before_main() {}")
        .unwrap();
    let main = bundled.find("fn main() {").unwrap();

    assert!(docs < attribute && attribute < import && import < import_comment);
    assert!(
        import_comment < helper_module,
        "the original grouped import should precede the bundled helper module"
    );
    assert!(
        helper_module < line_comment
            && line_comment < block_comment
            && block_comment < commented_out_code
            && commented_out_code < main,
        "main's leading comments and commented-out code should stay immediately before fn main"
    );
}

#[test]
fn preserves_comments_in_nested_modules_and_rewritten_paths() {
    let mut fixture = Fixture::new();
    fs::create_dir_all(fixture.root().join("src/helper")).unwrap();
    fs::write(
        fixture.root().join("src/helper.rs"),
        "//! Parent module docs.\n// Child declaration comment.\npub mod child;\n",
    )
    .unwrap();
    fs::write(
        fixture.root().join("src/helper/child.rs"),
        r#"//! Child module docs.
/// Answer docs.
pub fn answer() -> i32 {
    let value = 42; // 日本語の行末コメント。
    /* 外側 /* 入れ子 */ のコメント。 */
    value
}
"#,
    )
    .unwrap();
    git(
        fixture.root(),
        &["add", "src/helper.rs", "src/helper/child.rs"],
    );
    git(
        fixture.root(),
        &["commit", "-qm", "nested library reference"],
    );
    fixture.reference = String::from_utf8(git(fixture.root(), &["rev-parse", "HEAD"]).stdout)
        .unwrap()
        .trim()
        .to_owned();
    fs::write(
        fixture.root().join("src/main.rs"),
        "fn main() {\n    // Path comment.\n    let answer = /* 値の説明 */ solution::helper::child::answer();\n    println!(\"{}\", answer);\n}\n",
    )
    .unwrap();

    let output = fixture.run(
        &["--reference-commit", &fixture.reference, "--no-clipboard"],
        "200",
    );
    assert!(
        output.status.success(),
        "bundle failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let bundled = fs::read_to_string(fixture.root().join("target/bundle.rs")).unwrap();
    for comment in [
        "Parent module docs.",
        "Child declaration comment.",
        "Child module docs.",
        "Answer docs.",
        "// 日本語の行末コメント。",
        "/* 外側 /* 入れ子 */ のコメント。 */",
        "Path comment.",
        "/* 値の説明 */",
    ] {
        assert!(bundled.contains(comment), "missing comment: {comment}");
    }
    assert!(bundled.contains("crate::helper::child::answer()"));
}

#[test]
fn formats_bundle_without_collapsing_blank_lines_in_raw_strings_or_block_comments() {
    let fixture = Fixture::new();
    fs::write(
        fixture.root().join("src/main.rs"),
        r####"fn main() {
    // This standalone comment must survive formatting.



    let raw = r###"first line


third line"###;
    /* This block comment keeps its intentional spacing:


       second paragraph.
    */
    println!("{}", raw);
}
"####,
    )
    .unwrap();

    let output = fixture.run(
        &["--reference-commit", &fixture.reference, "--no-clipboard"],
        "200",
    );
    assert!(
        output.status.success(),
        "bundle failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let bundled = fs::read_to_string(fixture.root().join("target/bundle.rs")).unwrap();
    assert!(bundled.contains("// This standalone comment must survive formatting."));
    assert!(bundled.contains("first line\n\n\nthird line"));
    assert!(bundled.contains(
        "This block comment keeps its intentional spacing:\n\n\n       second paragraph."
    ));
    assert!(
        !bundled.contains("// This standalone comment must survive formatting.\n\n\n\n    let raw"),
        "long blank-line run outside comments and literals was retained"
    );
    assert!(bundled.contains("// This standalone comment must survive formatting.\n\n    let raw"));
}

#[test]
fn rejects_changed_selected_library_without_replacing_output_or_copying() {
    let fixture = Fixture::new();
    let output_path = fixture.root().join("target/bundle.rs");
    fs::create_dir_all(output_path.parent().unwrap()).unwrap();
    fs::write(&output_path, "keep this output").unwrap();
    fs::write(
        fixture.root().join("src/helper.rs"),
        "pub fn answer() -> i32 { 99 }\n",
    )
    .unwrap();

    let output = fixture.run(&["--reference-commit", &fixture.reference], "200");
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("differs from reference commit"));
    assert_eq!(fs::read(output_path).unwrap(), b"keep this output");
    assert!(!fixture.clipboard.exists());
}

#[test]
fn rejects_invalid_commit_and_nonpublic_reference() {
    let fixture = Fixture::new();

    let invalid = fixture.run(&["--reference-commit", "not-a-commit"], "200");
    assert!(!invalid.status.success());
    assert!(String::from_utf8_lossy(&invalid.stderr).contains("full 40-character commit SHA"));

    let nonexistent = fixture.run(
        &[
            "--reference-commit",
            "0000000000000000000000000000000000000000",
        ],
        "200",
    );
    assert!(!nonexistent.status.success());
    assert!(String::from_utf8_lossy(&nonexistent.stderr)
        .contains("pre-contest reference is not confirmed as publicly available"));

    let private = fixture.run(&["--reference-commit", &fixture.reference], "404");
    assert!(!private.status.success());
    assert!(String::from_utf8_lossy(&private.stderr).contains("HTTP 404"));
    assert!(!fixture.root().join("target/bundle.rs").exists());
    assert!(!fixture.clipboard.exists());
}

#[test]
fn rejects_network_failure_and_api_limits() {
    let fixture = Fixture::new();
    for response in ["network-error", "429"] {
        let output = fixture.run(&["--reference-commit", &fixture.reference], response);
        assert!(!output.status.success());
        assert!(String::from_utf8_lossy(&output.stderr)
            .contains("pre-contest reference is not confirmed as publicly available"));
        assert!(!fixture.root().join("target/bundle.rs").exists());
        assert!(!fixture.clipboard.exists());
    }
}

#[test]
fn rejects_changed_library_root() {
    let fixture = Fixture::new();
    fs::write(
        fixture.root().join("src/lib.rs"),
        format!("{LIBRARY}\n// Local edit\n"),
    )
    .unwrap();
    let output = fixture.run(&["--reference-commit", &fixture.reference], "200");
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("src/lib.rs differs"));
}

#[test]
fn resolves_default_origin_master_reference() {
    let fixture = Fixture::new();
    git(
        fixture.root(),
        &[
            "update-ref",
            "refs/remotes/origin/master",
            &fixture.reference,
        ],
    );

    let output = fixture.run(&["--no-clipboard"], "200");
    assert!(
        output.status.success(),
        "bundle failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let bundled = fs::read_to_string(fixture.root().join("target/bundle.rs")).unwrap();
    assert!(bundled.starts_with(&format!(
        "// https://github.com/Twil3akine/AtCoder-Algo/commit/{}\n\n",
        fixture.reference
    )));
    assert!(!fixture.clipboard.exists());
}
