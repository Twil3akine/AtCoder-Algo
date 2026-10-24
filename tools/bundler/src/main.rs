use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fs;
use std::io::Write;
use std::ops::Range;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use anyhow::{bail, Context, Result};
use syn::spanned::Spanned;
use syn::visit::{self, Visit};
use syn::{Attribute, File, Item, ItemMod, Path as SynPath, UseTree};

type ModulePath = Vec<String>;

const REFERENCE_REPOSITORY: &str = "Twil3akine/AtCoder-Algo";
const REFERENCE_ERROR: &str = "pre-contest reference is not confirmed as publicly available";

#[derive(Default)]
struct Selection {
    modules: BTreeSet<ModulePath>,
    items: BTreeMap<ModulePath, Option<BTreeSet<String>>>,
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
struct Dependency {
    module: ModulePath,
    item: Option<String>,
}

#[derive(Clone)]
struct ModuleInfo {
    file: PathBuf,
    source: String,
    syntax: File,
}

struct Library {
    root_source: String,
    root: File,
    modules: BTreeMap<ModulePath, ModuleInfo>,
    exported_macros: BTreeMap<String, ModulePath>,
}

struct Options {
    copy_to_clipboard: bool,
    output: Option<PathBuf>,
    reference_commit: Option<String>,
}

fn main() {
    if let Err(error) = run() {
        eprintln!("error: {error:#}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let options = parse_options()?;
    let project = find_project_root(&std::env::current_dir()?)?;
    let manifest = read_manifest(&project.join("Cargo.toml"))?;
    let crate_name = manifest
        .get("package")
        .and_then(|package| package.get("name"))
        .and_then(toml::Value::as_str)
        .context("Cargo.toml に package.name がありません")?
        .replace('-', "_");
    let edition = manifest
        .get("package")
        .and_then(|package| package.get("edition"))
        .and_then(toml::Value::as_str)
        .unwrap_or("2021");

    let main_file = project.join("src/main.rs");
    let library_file = project.join("src/lib.rs");
    let main_source = fs::read_to_string(&main_file)
        .with_context(|| format!("{} を読み取れません", main_file.display()))?;
    let main_syntax =
        syn::parse_file(&main_source).context("src/main.rs の構文解析に失敗しました")?;
    let library = Library::load(&library_file)?;

    let selected = select_library(&main_syntax, &library, &crate_name);
    let reference_commit = resolve_reference_commit(&project, options.reference_commit.as_deref())?;
    verify_library_sources(&project, &library, &selected, &reference_commit)?;
    confirm_public_reference(&reference_commit)?;
    let bundled = render_submission(
        &main_source,
        &main_syntax,
        &library,
        &selected,
        &crate_name,
        &reference_commit,
    )?;
    let bundled = format_submission(&bundled, edition)?;
    let output = options
        .output
        .map(|path| project.join(path))
        .unwrap_or_else(|| project.join("target/bundle.rs"));
    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&output, &bundled)
        .with_context(|| format!("{} を書き込めません", output.display()))?;

    compile_with_cargo(&output, edition, &manifest, &project)?;
    if options.copy_to_clipboard {
        copy_to_clipboard(&bundled)?;
    }

    let reported = selected
        .modules
        .iter()
        .filter(|path| module_has_code(&library.modules[*path].syntax))
        .map(|path| path.join("::"))
        .collect::<Vec<_>>();
    let source_size = main_source.len()
        + selected
            .modules
            .iter()
            .map(|path| fs::metadata(&library.modules[path].file).map(|meta| meta.len() as usize))
            .collect::<std::io::Result<Vec<_>>>()?
            .into_iter()
            .sum::<usize>();

    println!("Bundled {} modules:", reported.len());
    for module in reported {
        println!("{module}");
    }
    println!();
    println!(
        "Bundle size: {} -> {}",
        format_size(source_size),
        format_size(bundled.len())
    );
    println!();
    println!("✓ cargo check succeeded");
    if options.copy_to_clipboard {
        println!("✓ copied to clipboard");
    }
    println!(
        "✓ wrote {}",
        output.strip_prefix(&project).unwrap_or(&output).display()
    );
    Ok(())
}

fn parse_options() -> Result<Options> {
    let mut options = Options {
        copy_to_clipboard: true,
        output: None,
        reference_commit: None,
    };
    let mut arguments = std::env::args().skip(1);
    while let Some(argument) = arguments.next() {
        match argument.as_str() {
            "--no-clipboard" => options.copy_to_clipboard = false,
            "--output" => {
                options.output = Some(PathBuf::from(
                    arguments.next().context("--output にはパスが必要です")?,
                ));
            }
            "--reference-commit" => {
                options.reference_commit = Some(
                    arguments
                        .next()
                        .context("--reference-commit requires a full commit SHA")?,
                );
            }
            "-h" | "--help" => {
                println!(
                    "Usage: cargo bundle [--no-clipboard] [--output PATH] [--reference-commit SHA]"
                );
                std::process::exit(0);
            }
            _ => bail!("不明な引数です: {argument}"),
        }
    }
    Ok(options)
}

fn find_project_root(start: &Path) -> Result<PathBuf> {
    for directory in start.ancestors() {
        if directory.join("Cargo.toml").is_file()
            && directory.join("src/main.rs").is_file()
            && directory.join("src/lib.rs").is_file()
        {
            return Ok(directory.to_path_buf());
        }
    }
    bail!("src/main.rs と src/lib.rs を持つ Cargo project が見つかりません")
}

fn read_manifest(path: &Path) -> Result<toml::Value> {
    let source = fs::read_to_string(path)?;
    toml::from_str(&source).with_context(|| format!("{} を解析できません", path.display()))
}

fn reference_git(project: &Path, arguments: &[&str]) -> Result<Vec<u8>> {
    let output = Command::new("git")
        .arg("--no-replace-objects")
        .arg("-C")
        .arg(project)
        .args(arguments)
        .output()
        .with_context(|| format!("{REFERENCE_ERROR}: cannot run git"))?;
    if !output.status.success() {
        bail!(
            "{REFERENCE_ERROR}: git failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }
    Ok(output.stdout)
}

fn resolve_reference_commit(project: &Path, requested: Option<&str>) -> Result<String> {
    let commit = match requested {
        Some(commit) => commit.to_owned(),
        None => String::from_utf8(reference_git(
            project,
            &[
                "rev-parse",
                "--verify",
                "refs/remotes/origin/master^{commit}",
            ],
        )?)?
        .trim()
        .to_owned(),
    };
    if commit.len() != 40 || !commit.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        bail!("{REFERENCE_ERROR}: reference must be a full 40-character commit SHA");
    }
    let commit = commit.to_ascii_lowercase();
    let kind = reference_git(project, &["cat-file", "-t", &commit])?;
    if kind != b"commit\n" {
        bail!("{REFERENCE_ERROR}: reference is not a commit");
    }
    Ok(commit)
}

fn verify_library_sources(
    project: &Path,
    library: &Library,
    selected: &Selection,
    commit: &str,
) -> Result<()> {
    let root = String::from_utf8(reference_git(project, &["rev-parse", "--show-toplevel"])?)?;
    let root = Path::new(root.trim());
    let root_file = project.join("src/lib.rs");
    let sources = std::iter::once((root_file.as_path(), library.root_source.as_str())).chain(
        selected.modules.iter().map(|path| {
            let module = &library.modules[path];
            (module.file.as_path(), module.source.as_str())
        }),
    );
    for (file, source) in sources {
        let relative = file.strip_prefix(root).with_context(|| {
            format!(
                "{REFERENCE_ERROR}: library is outside the Git repository: {}",
                file.display()
            )
        })?;
        let object = format!("{commit}:{}", relative.display());
        let published = reference_git(project, &["cat-file", "blob", &object])?;
        if source.as_bytes() != published {
            bail!(
                "{REFERENCE_ERROR}: {} differs from reference commit {commit}; publish the reusable library before the contest",
                relative.display()
            );
        }
    }
    Ok(())
}

fn confirm_public_reference(commit: &str) -> Result<()> {
    // Disable curl's config so a personal Authorization header or netrc option cannot
    // turn an authenticated-only commit into a supposedly public reference.
    let output = Command::new("curl")
        .args([
            "--disable",
            "--fail",
            "--silent",
            "--show-error",
            "--location",
            "--proto",
            "=https",
            "--proto-redir",
            "=https",
            "--max-time",
            "20",
            "--output",
            "/dev/null",
            "--write-out",
            "%{http_code}",
        ])
        .arg(format!(
            "https://api.github.com/repos/{REFERENCE_REPOSITORY}/commits/{commit}"
        ))
        .output()
        .with_context(|| format!("{REFERENCE_ERROR}: cannot run anonymous curl verification"))?;
    if !output.status.success() || output.stdout != b"200" {
        bail!(
            "{REFERENCE_ERROR}: anonymous GitHub request failed (HTTP {}): {}",
            String::from_utf8_lossy(&output.stdout).trim(),
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }
    Ok(())
}

fn format_submission(source: &str, edition: &str) -> Result<String> {
    let mut process = Command::new("rustfmt")
        .args(["--edition", edition, "--emit", "stdout"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .context("rustfmt を起動できません")?;
    process
        .stdin
        .take()
        .context("rustfmt の標準入力を開けません")?
        .write_all(source.as_bytes())?;
    let output = process
        .wait_with_output()
        .context("rustfmt の終了を待機できません")?;
    if !output.status.success() {
        bail!(
            "生成コードの rustfmt に失敗しました:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    String::from_utf8(output.stdout).context("rustfmt の出力が UTF-8 ではありません")
}

fn render_submission(
    source: &str,
    syntax: &File,
    library: &Library,
    selected: &Selection,
    crate_name: &str,
    commit: &str,
) -> Result<String> {
    let mut edits = Vec::new();
    collect_item_edits(source, &syntax.items, &[], None, &mut edits)?;
    for item in &syntax.items {
        collect_path_edits(item, crate_name, &mut edits);
        if let Item::Use(item_use) = item {
            if !collect_macro_import_edits(
                &item_use.tree,
                &mut Vec::new(),
                crate_name,
                &library.exported_macros,
                &mut edits,
            ) {
                edits.push((item.span().byte_range(), String::new()));
            }
        }
    }

    let mut library_edits = library
        .root
        .attrs
        .iter()
        .map(|attr| (attr.span().byte_range(), String::new()))
        .collect::<Vec<_>>();
    collect_item_edits(
        &library.root_source,
        &library.root.items,
        &[],
        Some((library, selected)),
        &mut library_edits,
    )?;
    let modules = apply_source_edits(&library.root_source, library_edits);
    let insertion = module_insertion_offset(source, syntax);
    if !modules.trim().is_empty() {
        edits.push((insertion..insertion, format!("\n{modules}\n")));
    }

    let library_docs = library
        .root
        .attrs
        .iter()
        .filter(|attr| attr.path().is_ident("doc"))
        .map(|attr| format!("{}\n", &library.root_source[attr.span().byte_range()]))
        .collect::<String>();
    let transformed = apply_source_edits(source, edits);
    Ok(format!("// https://github.com/{REFERENCE_REPOSITORY}/commit/{commit}\n\n{library_docs}{transformed}"))
}

fn module_insertion_offset(source: &str, syntax: &File) -> usize {
    let offset = syntax
        .items
        .iter()
        .take_while(|item| matches!(item, Item::ExternCrate(_) | Item::Use(_)))
        .last()
        .map(|item| item.span().byte_range().end)
        .or_else(|| syntax.attrs.last().map(|attr| attr.span().byte_range().end));
    let Some(mut cursor) = offset else { return 0 };
    let bytes = source.as_bytes();
    // Keep trailing import comments with the import, without consuming the
    // following line's comments for the first solution item.
    while cursor < bytes.len() {
        match bytes[cursor] {
            b' ' | b'\t' | b'\r' => cursor += 1,
            b'\n' => return cursor + 1,
            b'/' if bytes.get(cursor + 1) == Some(&b'/') => {
                return source[cursor..]
                    .find('\n')
                    .map_or(source.len(), |end| cursor + end + 1);
            }
            b'/' if bytes.get(cursor + 1) == Some(&b'*') => {
                let mut depth = 1;
                cursor += 2;
                while depth > 0 && cursor < bytes.len() {
                    match bytes.get(cursor..cursor + 2) {
                        Some(b"/*") => {
                            depth += 1;
                            cursor += 2;
                        }
                        Some(b"*/") => {
                            depth -= 1;
                            cursor += 2;
                        }
                        _ => cursor += 1,
                    }
                }
            }
            _ => break,
        }
    }
    cursor
}

fn render_inline_module(
    path: &[String],
    declaration: &ItemMod,
    parent_source: &str,
    library: &Library,
    selected: &Selection,
) -> Result<String> {
    let info = library
        .modules
        .get(path)
        .with_context(|| format!("module {} が index にありません", path.join("::")))?;
    let mut body_edits = Vec::new();
    collect_item_edits(
        &info.source,
        &info.syntax.items,
        path,
        Some((library, selected)),
        &mut body_edits,
    )?;
    let body = apply_source_edits(&info.source, body_edits);
    let range = declaration.span().byte_range();
    let mut edits = Vec::new();
    for attribute in declaration
        .attrs
        .iter()
        .filter(|attr| attr.path().is_ident("path"))
    {
        let attr = attribute.span().byte_range();
        edits.push((
            attr.start - range.start..attr.end - range.start,
            String::new(),
        ));
    }
    let semicolon = declaration
        .semi
        .context("外部モジュールのセミコロンがありません")?
        .span
        .byte_range();
    edits.push((
        semicolon.start - range.start..semicolon.end - range.start,
        format!("{{\n{body}\n}}"),
    ));
    Ok(apply_source_edits(&parent_source[range], edits))
}

fn collect_item_edits(
    source: &str,
    items: &[Item],
    current: &[String],
    library_selection: Option<(&Library, &Selection)>,
    edits: &mut Vec<(Range<usize>, String)>,
) -> Result<()> {
    let mut retained = items.to_vec();
    if let Some((_, selected)) = library_selection {
        if let Some(Some(names)) = selected.items.get(current) {
            prune_unselected_items(&mut retained, names);
        }
    }
    let retained_ranges = retained
        .iter()
        .map(|item| {
            let range = item.span().byte_range();
            (range.start, range.end)
        })
        .collect::<BTreeSet<_>>();
    for item in items {
        let range = item.span().byte_range();
        let is_library_root = library_selection.is_some() && current.is_empty();
        if !retained_ranges.contains(&(range.start, range.end))
            || is_test_only(item_attributes(item))
            || (is_library_root && !matches!(item, Item::Mod(_)))
        {
            edits.push((range, String::new()));
            continue;
        }
        if let Item::Mod(module) = item {
            if let Some((_, children)) = &module.content {
                // Inline modules retain their items, except test-only code.
                collect_item_edits(source, children, current, None, edits)?;
            } else if let Some((library, selected)) = library_selection {
                let mut path = current.to_vec();
                path.push(module.ident.to_string());
                if selected.modules.contains(&path) {
                    let rendered = render_inline_module(&path, module, source, library, selected)?;
                    edits.push((range, rendered));
                } else {
                    edits.push((range, String::new()));
                }
            }
        }
    }
    Ok(())
}

fn collect_macro_import_edits(
    tree: &UseTree,
    prefix: &mut ModulePath,
    crate_name: &str,
    macros: &BTreeMap<String, ModulePath>,
    edits: &mut Vec<(Range<usize>, String)>,
) -> bool {
    match tree {
        UseTree::Path(path) => {
            prefix.push(path.ident.to_string());
            let keep = collect_macro_import_edits(&path.tree, prefix, crate_name, macros, edits);
            prefix.pop();
            keep
        }
        UseTree::Name(name) => {
            !(prefix.len() == 1
                && prefix[0] == crate_name
                && macros.contains_key(&name.ident.to_string()))
        }
        UseTree::Rename(rename) => {
            !(prefix.len() == 1
                && prefix[0] == crate_name
                && macros.contains_key(&rename.ident.to_string()))
        }
        UseTree::Glob(_) => true,
        UseTree::Group(group) => {
            let pairs = group.items.pairs().collect::<Vec<_>>();
            let mut any_retained = false;
            for (index, pair) in pairs.iter().enumerate() {
                let child = pair.value();
                if collect_macro_import_edits(child, prefix, crate_name, macros, edits) {
                    any_retained = true;
                } else {
                    edits.push((child.span().byte_range(), String::new()));
                    let comma = pair
                        .punct()
                        .or_else(|| index.checked_sub(1).and_then(|prev| pairs[prev].punct()));
                    if let Some(comma) = comma {
                        edits.push((comma.span.byte_range(), String::new()));
                    }
                }
            }
            any_retained
        }
    }
}

fn collect_path_edits(item: &Item, crate_name: &str, edits: &mut Vec<(Range<usize>, String)>) {
    let mut collector = CratePathCollector {
        crate_name,
        spans: Vec::new(),
    };
    collector.visit_item(item);
    for span in collector.spans {
        edits.push((span.byte_range(), "crate".to_owned()));
    }
}

struct CratePathCollector<'a> {
    crate_name: &'a str,
    spans: Vec<proc_macro2::Span>,
}

impl<'ast> Visit<'ast> for CratePathCollector<'_> {
    fn visit_path(&mut self, path: &'ast SynPath) {
        if let Some(first) = path.segments.first() {
            if first.ident == self.crate_name {
                self.spans.push(first.ident.span());
            }
        }
        visit::visit_path(self, path);
    }

    fn visit_item_use(&mut self, item: &'ast syn::ItemUse) {
        collect_use_ident_spans(&item.tree, self.crate_name, &mut self.spans);
        visit::visit_item_use(self, item);
    }
}

fn collect_use_ident_spans(tree: &UseTree, crate_name: &str, spans: &mut Vec<proc_macro2::Span>) {
    match tree {
        UseTree::Path(path) => {
            if path.ident == crate_name {
                spans.push(path.ident.span());
            }
            collect_use_ident_spans(&path.tree, crate_name, spans);
        }
        UseTree::Group(group) => {
            for tree in &group.items {
                collect_use_ident_spans(tree, crate_name, spans);
            }
        }
        _ => {}
    }
}

fn apply_source_edits(source: &str, mut edits: Vec<(Range<usize>, String)>) -> String {
    // Apply outer edits before contained edits, using offsets in the original source.
    edits.sort_by(|a, b| {
        a.0.start.cmp(&b.0.start).then_with(|| {
            if a.0.is_empty() || b.0.is_empty() {
                a.0.end.cmp(&b.0.end)
            } else {
                b.0.end.cmp(&a.0.end)
            }
        })
    });
    let mut output = String::new();
    let mut cursor = 0;
    for (range, replacement) in edits {
        if range.start < cursor {
            continue;
        }
        output.push_str(&source[cursor..range.start]);
        output.push_str(&replacement);
        cursor = range.end;
    }
    output.push_str(&source[cursor..]);
    output
}

impl Library {
    fn load(root_file: &Path) -> Result<Self> {
        let source = fs::read_to_string(root_file)?;
        let root = syn::parse_file(&source).context("src/lib.rs の構文解析に失敗しました")?;
        let mut library = Self {
            root_source: source,
            root: root.clone(),
            modules: BTreeMap::new(),
            exported_macros: BTreeMap::new(),
        };
        library.load_children(&[], root_file, &root.items)?;
        Ok(library)
    }

    fn load_children(
        &mut self,
        parent: &[String],
        parent_file: &Path,
        items: &[Item],
    ) -> Result<()> {
        let declarations = items
            .iter()
            .filter_map(|item| match item {
                Item::Mod(module) if module.content.is_none() && !is_test_only(&module.attrs) => {
                    Some(module.clone())
                }
                _ => None,
            })
            .collect::<Vec<_>>();

        for declaration in declarations {
            let mut path = parent.to_vec();
            path.push(declaration.ident.to_string());
            let file = resolve_module_file(parent_file, &declaration)?;
            let source = fs::read_to_string(&file)
                .with_context(|| format!("module {} を読み取れません", path.join("::")))?;
            let syntax = syn::parse_file(&source)
                .with_context(|| format!("module {} の構文解析に失敗しました", path.join("::")))?;

            for item in &syntax.items {
                if let Item::Macro(item_macro) = item {
                    if has_attribute(&item_macro.attrs, "macro_export") {
                        if let Some(identifier) = &item_macro.ident {
                            self.exported_macros
                                .insert(identifier.to_string(), path.clone());
                        }
                    }
                }
            }

            self.modules.insert(
                path.clone(),
                ModuleInfo {
                    file: file.clone(),
                    source,
                    syntax: syntax.clone(),
                },
            );
            self.load_children(&path, &file, &syntax.items)?;
        }
        Ok(())
    }
}

fn resolve_module_file(parent_file: &Path, module: &ItemMod) -> Result<PathBuf> {
    if let Some(path) = module.attrs.iter().find_map(path_attribute) {
        return Ok(parent_file.parent().unwrap().join(path));
    }
    let parent = parent_file.parent().unwrap();
    let module_directory = match parent_file.file_stem().and_then(|name| name.to_str()) {
        Some("lib" | "main" | "mod") => parent.to_path_buf(),
        Some(stem) => parent.join(stem),
        None => parent.to_path_buf(),
    };
    let name = module.ident.to_string();
    let file = module_directory.join(format!("{name}.rs"));
    if file.is_file() {
        return Ok(file);
    }
    let mod_file = module_directory.join(name).join("mod.rs");
    if mod_file.is_file() {
        return Ok(mod_file);
    }
    bail!("module {} のソースファイルが見つかりません", module.ident)
}

fn path_attribute(attribute: &Attribute) -> Option<PathBuf> {
    if !attribute.path().is_ident("path") {
        return None;
    }
    let syn::Meta::NameValue(meta) = &attribute.meta else {
        return None;
    };
    let syn::Expr::Lit(expression) = &meta.value else {
        return None;
    };
    let syn::Lit::Str(path) = &expression.lit else {
        return None;
    };
    Some(PathBuf::from(path.value()))
}

fn select_library(main: &File, library: &Library, crate_name: &str) -> Selection {
    let mut selected = Selection::default();
    let mut queue = VecDeque::new();
    for dependency in dependencies(main, &[], library, crate_name, true) {
        add_dependency(dependency, &mut selected, &mut queue);
    }

    while let Some(module) = queue.pop_front() {
        let Some(info) = library.modules.get(&module) else {
            continue;
        };
        for dependency in dependencies(&info.syntax, &module, library, crate_name, false) {
            add_dependency(dependency, &mut selected, &mut queue);
        }
    }
    selected
}

fn add_dependency(
    dependency: Dependency,
    selected: &mut Selection,
    queue: &mut VecDeque<ModulePath>,
) {
    for length in 1..=dependency.module.len() {
        let ancestor = dependency.module[..length].to_vec();
        if selected.modules.insert(ancestor.clone()) {
            queue.push_back(ancestor);
        }
    }

    let entry = selected
        .items
        .entry(dependency.module)
        .or_insert_with(|| Some(BTreeSet::new()));
    match (entry, dependency.item) {
        (slot @ Some(_), None) => *slot = None,
        (Some(items), Some(item)) => {
            items.insert(item);
        }
        (None, _) => {}
    }
}

fn dependencies(
    syntax: &File,
    current: &[String],
    library: &Library,
    crate_name: &str,
    is_main: bool,
) -> BTreeSet<Dependency> {
    let mut syntax = syntax.clone();
    prune_tests(&mut syntax.items);
    let mut collector = PathCollector::default();
    collector.visit_file(&syntax);
    let mut result = BTreeSet::new();

    for raw in collector.paths {
        if raw.is_empty() {
            continue;
        }
        if !is_main && raw.len() == 1 && raw[0] == "self" {
            continue;
        }
        let candidate = if is_main {
            if raw[0] != crate_name {
                continue;
            }
            raw[1..].to_vec()
        } else if raw[0] == "crate" {
            raw[1..].to_vec()
        } else if raw[0] == "self" {
            current
                .iter()
                .cloned()
                .chain(raw[1..].iter().cloned())
                .collect()
        } else if raw[0] == "super" {
            let count = raw
                .iter()
                .take_while(|part| part.as_str() == "super")
                .count();
            let keep = current.len().saturating_sub(count);
            current[..keep]
                .iter()
                .cloned()
                .chain(raw[count..].iter().cloned())
                .collect()
        } else {
            continue;
        };

        if let Some(module) = longest_module_prefix(&candidate, &library.modules) {
            let item = candidate
                .get(module.len())
                .filter(|name| name.as_str() != "self")
                .cloned();
            result.insert(Dependency { module, item });
        } else if is_main {
            if let Some(name) = candidate.last() {
                if let Some(module) = library.exported_macros.get(name) {
                    result.insert(Dependency {
                        module: module.clone(),
                        item: None,
                    });
                }
            }
        }
    }
    result
}

fn longest_module_prefix(
    path: &[String],
    modules: &BTreeMap<ModulePath, ModuleInfo>,
) -> Option<ModulePath> {
    (1..=path.len())
        .rev()
        .map(|length| path[..length].to_vec())
        .find(|prefix| modules.contains_key(prefix))
}

#[derive(Default)]
struct PathCollector {
    paths: Vec<ModulePath>,
}

impl<'ast> Visit<'ast> for PathCollector {
    fn visit_item_use(&mut self, item: &'ast syn::ItemUse) {
        collect_use_tree(&item.tree, &mut Vec::new(), &mut self.paths);
    }

    fn visit_path(&mut self, path: &'ast SynPath) {
        self.paths.push(
            path.segments
                .iter()
                .map(|part| part.ident.to_string())
                .collect(),
        );
        visit::visit_path(self, path);
    }
}

fn collect_use_tree(tree: &UseTree, prefix: &mut ModulePath, output: &mut Vec<ModulePath>) {
    match tree {
        UseTree::Path(path) => {
            prefix.push(path.ident.to_string());
            collect_use_tree(&path.tree, prefix, output);
            prefix.pop();
        }
        UseTree::Name(name) => {
            prefix.push(name.ident.to_string());
            output.push(prefix.clone());
            prefix.pop();
        }
        UseTree::Rename(rename) => {
            prefix.push(rename.ident.to_string());
            output.push(prefix.clone());
            prefix.pop();
        }
        UseTree::Glob(_) => output.push(prefix.clone()),
        UseTree::Group(group) => {
            for tree in &group.items {
                collect_use_tree(tree, prefix, output);
            }
        }
    }
}

fn prune_unselected_items(items: &mut Vec<Item>, selected: &BTreeSet<String>) {
    let named_items = items
        .iter()
        .enumerate()
        .filter_map(|(index, item)| item_name(item).map(|name| (name, index)))
        .collect::<BTreeMap<_, _>>();
    let mut retained_names = selected.clone();
    let mut retained = vec![false; items.len()];

    loop {
        let mut changed = false;

        for name in retained_names.clone() {
            if let Some(&index) = named_items.get(&name) {
                if !retained[index] {
                    retained[index] = true;
                    changed = true;
                }
            }
        }

        for (index, item) in items.iter().enumerate() {
            let keep_unnamed = matches!(item, Item::Use(_) | Item::ExternCrate(_) | Item::Macro(_));
            let keep_impl = match item {
                Item::Impl(item_impl) => impl_names(item_impl)
                    .iter()
                    .any(|name| retained_names.contains(name)),
                _ => false,
            };
            if (keep_unnamed || keep_impl) && !retained[index] {
                retained[index] = true;
                changed = true;
            }
        }

        for (index, item) in items.iter().enumerate() {
            if !retained[index] {
                continue;
            }
            let mut collector = PathCollector::default();
            collector.visit_item(item);
            for path in collector.paths {
                let Some(name) = path.first() else {
                    continue;
                };
                if named_items.contains_key(name) && retained_names.insert(name.clone()) {
                    changed = true;
                }
            }
        }

        if !changed {
            break;
        }
    }

    let mut index = 0;
    items.retain(|_| {
        let keep = retained[index];
        index += 1;
        keep
    });
}

fn item_name(item: &Item) -> Option<String> {
    let identifier = match item {
        Item::Const(item) => &item.ident,
        Item::Enum(item) => &item.ident,
        Item::Fn(item) => &item.sig.ident,
        Item::Mod(item) => &item.ident,
        Item::Static(item) => &item.ident,
        Item::Struct(item) => &item.ident,
        Item::Trait(item) => &item.ident,
        Item::TraitAlias(item) => &item.ident,
        Item::Type(item) => &item.ident,
        Item::Union(item) => &item.ident,
        _ => return None,
    };
    Some(identifier.to_string())
}

fn impl_names(item: &syn::ItemImpl) -> BTreeSet<String> {
    let mut names = BTreeSet::new();
    if let Some((_, path, _)) = &item.trait_ {
        if let Some(segment) = path.segments.last() {
            names.insert(segment.ident.to_string());
        }
    }
    let mut collector = PathCollector::default();
    collector.visit_type(&item.self_ty);
    for path in collector.paths {
        if let Some(name) = path.last() {
            names.insert(name.clone());
        }
    }
    names
}

fn prune_tests(items: &mut Vec<Item>) {
    items.retain(|item| !is_test_only(item_attributes(item)));
    for item in items {
        if let Item::Mod(module) = item {
            if let Some((_, items)) = &mut module.content {
                prune_tests(items);
            }
        }
    }
}

fn item_attributes(item: &Item) -> &[Attribute] {
    match item {
        Item::Const(item) => &item.attrs,
        Item::Enum(item) => &item.attrs,
        Item::ExternCrate(item) => &item.attrs,
        Item::Fn(item) => &item.attrs,
        Item::ForeignMod(item) => &item.attrs,
        Item::Impl(item) => &item.attrs,
        Item::Macro(item) => &item.attrs,
        Item::Mod(item) => &item.attrs,
        Item::Static(item) => &item.attrs,
        Item::Struct(item) => &item.attrs,
        Item::Trait(item) => &item.attrs,
        Item::TraitAlias(item) => &item.attrs,
        Item::Type(item) => &item.attrs,
        Item::Union(item) => &item.attrs,
        Item::Use(item) => &item.attrs,
        _ => &[],
    }
}

fn is_test_only(attributes: &[Attribute]) -> bool {
    has_attribute(attributes, "test")
        || attributes.iter().any(|attribute| {
            if !attribute.path().is_ident("cfg") {
                return false;
            }
            let syn::Meta::List(list) = &attribute.meta else {
                return false;
            };
            let mut test = false;
            let _ = list.parse_nested_meta(|meta| {
                if meta.path.is_ident("test") {
                    test = true;
                }
                Ok(())
            });
            test
        })
}

fn has_attribute(attributes: &[Attribute], name: &str) -> bool {
    attributes
        .iter()
        .any(|attribute| attribute.path().is_ident(name))
}

fn module_has_code(syntax: &File) -> bool {
    syntax.items.iter().any(|item| {
        !matches!(item, Item::Mod(module) if module.content.is_none())
            && !is_test_only(item_attributes(item))
    })
}

fn compile_with_cargo(
    source: &Path,
    edition: &str,
    manifest: &toml::Value,
    project: &Path,
) -> Result<()> {
    let directory = tempfile::tempdir()?;
    let source_directory = directory.path().join("src");
    fs::create_dir(&source_directory)?;
    fs::copy(source, source_directory.join("main.rs"))?;

    let mut package = toml::map::Map::new();
    package.insert(
        "name".into(),
        manifest
            .get("package")
            .and_then(|package| package.get("name"))
            .and_then(toml::Value::as_str)
            .unwrap_or("bundle-check")
            .into(),
    );
    package.insert(
        "version".into(),
        manifest
            .get("package")
            .and_then(|package| package.get("version"))
            .and_then(toml::Value::as_str)
            .unwrap_or("0.0.0")
            .into(),
    );
    package.insert("edition".into(), edition.into());

    let mut check_manifest = toml::map::Map::new();
    check_manifest.insert("package".into(), toml::Value::Table(package));
    for key in ["dependencies", "features", "target", "patch"] {
        if let Some(value) = manifest.get(key) {
            check_manifest.insert(key.into(), value.clone());
        }
    }
    fs::write(
        directory.path().join("Cargo.toml"),
        toml::to_string(&check_manifest)?,
    )?;

    let lock_file = project.join("Cargo.lock");
    if lock_file.is_file() {
        fs::copy(&lock_file, directory.path().join("Cargo.lock"))?;
    }

    let mut command = Command::new("cargo");
    command.arg("check").arg("--quiet");
    if lock_file.is_file() {
        command.arg("--locked");
    }
    let output = command
        .arg("--manifest-path")
        .arg(directory.path().join("Cargo.toml"))
        .arg("--target-dir")
        .arg(project.join("target"))
        .output()
        .context("cargo check を実行できません")?;
    if !output.status.success() {
        bail!(
            "生成コードの cargo check に失敗しました:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    Ok(())
}

fn copy_to_clipboard(source: &str) -> Result<()> {
    let mut process = Command::new("pbcopy")
        .stdin(Stdio::piped())
        .spawn()
        .context("pbcopy を起動できません")?;
    process
        .stdin
        .take()
        .context("pbcopy の標準入力を開けません")?
        .write_all(source.as_bytes())?;
    let status = process.wait()?;
    if !status.success() {
        bail!("pbcopy が失敗しました");
    }
    Ok(())
}

fn format_size(bytes: usize) -> String {
    if bytes < 1024 {
        format!("{bytes} B")
    } else {
        format!("{:.1} KiB", bytes as f64 / 1024.0)
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::{
        apply_source_edits, collect_macro_import_edits, collect_use_tree, compile_with_cargo,
        format_size, item_name, render_inline_module, select_library, Library, ModulePath,
    };

    #[test]
    fn flattens_grouped_use_paths() {
        let item: syn::ItemUse = syn::parse_quote!(
            use atcoder::geometry::{Point, ccw};
        );
        let mut paths = Vec::new();
        collect_use_tree(&item.tree, &mut ModulePath::new(), &mut paths);
        assert_eq!(
            paths,
            [
                vec!["atcoder", "geometry", "Point"],
                vec!["atcoder", "geometry", "ccw"]
            ]
        );
    }

    #[test]
    fn formats_kibibytes() {
        assert_eq!(format_size(512), "512 B");
        assert_eq!(format_size(1536), "1.5 KiB");
    }

    #[test]
    fn resolves_transitive_module_dependencies() {
        let directory = tempfile::tempdir().unwrap();
        let source = directory.path().join("src");
        fs::create_dir_all(source.join("lib")).unwrap();
        fs::write(
            source.join("lib.rs"),
            "#[path = \"lib/a.rs\"] pub mod a;\n#[path = \"lib/b.rs\"] pub mod b;\n",
        )
        .unwrap();
        fs::write(
            source.join("lib/a.rs"),
            "use crate::b::B; pub fn make() -> B { B }\n",
        )
        .unwrap();
        fs::write(source.join("lib/b.rs"), "pub struct B;\n").unwrap();

        let library = Library::load(&source.join("lib.rs")).unwrap();
        let main = syn::parse_file("use sample::a::make; fn main() { let _ = make(); }").unwrap();
        let selected = select_library(&main, &library, "sample");
        assert!(selected.modules.contains(&vec!["a".into()]));
        assert!(selected.modules.contains(&vec!["b".into()]));
    }

    #[test]
    fn keeps_only_reachable_items_for_an_explicit_function_import() {
        let directory = tempfile::tempdir().unwrap();
        let source = directory.path().join("src");
        fs::create_dir_all(source.join("lib")).unwrap();
        fs::write(
            source.join("lib.rs"),
            "#[path = \"lib/string.rs\"] pub mod string;\n",
        )
        .unwrap();
        fs::write(
            source.join("lib/string.rs"),
            "\
pub fn manacher() -> usize { helper() }
fn helper() -> usize { 1 }
pub trait SequenceExt { fn is_palindrome(&self) -> bool; }
impl SequenceExt for str { fn is_palindrome(&self) -> bool { true } }
pub trait DecimalDigitsExt { fn parse_usize(&self) -> Option<usize>; }
",
        )
        .unwrap();

        let library = Library::load(&source.join("lib.rs")).unwrap();
        let main =
            syn::parse_file("use sample::string::manacher; fn main() { let _ = manacher(); }")
                .unwrap();
        let selected = select_library(&main, &library, "sample");
        let path = vec!["string".into()];
        let declaration = library
            .root
            .items
            .iter()
            .find_map(|item| match item {
                syn::Item::Mod(module) => Some(module),
                _ => None,
            })
            .unwrap();
        let rendered = render_inline_module(
            &path,
            declaration,
            &library.root_source,
            &library,
            &selected,
        )
        .unwrap();
        let module = syn::parse_str::<syn::ItemMod>(&rendered).unwrap();
        let names = module
            .content
            .unwrap()
            .1
            .iter()
            .filter_map(item_name)
            .collect::<Vec<_>>();

        assert_eq!(names, ["manacher", "helper"]);
    }

    #[test]
    fn excludes_string_extension_traits_when_only_manacher_is_imported() {
        let project = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let library = Library::load(&project.join("src/lib.rs")).unwrap();
        let main = syn::parse_file(
            "use atcoder::string::manacher; fn main() { let _ = manacher(b\"\"); }",
        )
        .unwrap();
        let selected = select_library(&main, &library, "atcoder");
        let path = vec!["string".into()];
        let declaration = library.root.items.iter().find_map(|item| match item {
            syn::Item::Mod(module) if module.ident == "string" => Some(module),
            _ => None,
        });
        let module = render_inline_module(
            &path,
            declaration.unwrap(),
            &library.root_source,
            &library,
            &selected,
        )
        .unwrap();
        assert!(!module.contains("DecimalDigitsExt"));
        let source = tempfile::tempdir().unwrap();
        let output = source.path().join("bundle.rs");
        fs::write(
            &output,
            format!(
                "{module}\nuse crate::string::manacher; fn main() {{ let _ = manacher(b\"\"); }}"
            ),
        )
        .unwrap();
        let manifest = toml::from_str("").unwrap();
        compile_with_cargo(&output, "2021", &manifest, source.path()).unwrap();
    }

    #[test]
    fn removes_only_exported_macros_from_grouped_import() {
        let macros = [("input".into(), vec!["io".into()])].into_iter().collect();
        for source in [
            "use atcoder::{input, /* keep comment */ io::Writer, geometry::Point};",
            "use atcoder::{/* keep comment */ io::Writer, geometry::Point, input};",
            "use atcoder::{input, /* keep comment */ io::Writer, input, geometry::Point, input,};",
        ] {
            let item = syn::parse_str::<syn::ItemUse>(source).unwrap();
            let mut edits = Vec::new();
            assert!(collect_macro_import_edits(
                &item.tree,
                &mut Vec::new(),
                "atcoder",
                &macros,
                &mut edits
            ));
            let rendered = apply_source_edits(source, edits);
            assert!(rendered.contains("/* keep comment */"));
            let filtered = syn::parse_str::<syn::ItemUse>(&rendered).unwrap();
            let mut paths = Vec::new();
            collect_use_tree(&filtered.tree, &mut Vec::new(), &mut paths);
            assert_eq!(
                paths,
                [
                    vec!["atcoder", "io", "Writer"],
                    vec!["atcoder", "geometry", "Point"]
                ]
            );
        }
    }

    #[test]
    fn compiles_bundle_with_package_dependencies() {
        let directory = tempfile::tempdir().unwrap();
        let dependency = directory.path().join("dependency");
        fs::create_dir_all(dependency.join("src")).unwrap();
        fs::write(
            dependency.join("Cargo.toml"),
            "[package]\nname = \"sample-dependency\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
        )
        .unwrap();
        fs::write(
            dependency.join("src/lib.rs"),
            "pub fn answer() -> usize { 42 }\n",
        )
        .unwrap();

        let bundle = directory.path().join("bundle.rs");
        fs::write(
            &bundle,
            "fn main() { assert_eq!(sample_dependency::answer(), 42); }\n",
        )
        .unwrap();
        let manifest = toml::from_str(&format!(
            "[dependencies]\nsample-dependency = {{ path = {:?} }}\n",
            dependency
        ))
        .unwrap();

        compile_with_cargo(&bundle, "2021", &manifest, directory.path()).unwrap();
    }
}
