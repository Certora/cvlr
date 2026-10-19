//! `#[rule]` writes `.cvlr_rules`, and `cargo certora-sbf` keeps the section.

mod section;

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};

use section::{rule_record, LoadKind, ReadError};

#[test]
fn rule_record_is_in_the_host_object() {
    let staged = Stage::new("host");
    let cvlr = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    fs::create_dir_all(staged.path.join("src")).unwrap();
    fs::write(
        staged.path.join("Cargo.toml"),
        format!(
            r#"[package]
name = "rule_manifest_host"
version = "0.0.0"
edition = "2021"
publish = false

[workspace]

[dependencies]
cvlr = {{ path = "{cvlr}" }}
"#,
            cvlr = cvlr.display(),
        ),
    )
    .unwrap();
    fs::write(
        staged.path.join("src/lib.rs"),
        "use cvlr::prelude::*;\n\n#[rule]\nfn rule_alpha() {}\n",
    )
    .unwrap();

    let target = workspace_target().join("rule-manifest-host");
    let output = cargo(
        &staged.path,
        &[
            "rustc",
            "--manifest-path",
            staged.path.join("Cargo.toml").to_str().unwrap(),
            "--message-format=json",
            "--color",
            "never",
            "--",
            "--emit=obj",
            "-C",
            "codegen-units=1",
        ],
        &target,
    );
    assert!(
        output.status.success(),
        "host rustc --emit=obj failed:\n{}",
        tail(&output.stderr),
    );
    let objects = object_files(
        &String::from_utf8_lossy(&output.stdout),
        "rule_manifest_host",
    );
    let mut hits = Vec::new();
    for object in &objects {
        match section::read_rules(object) {
            Ok(rules) => hits.push((object.clone(), rules)),
            Err(ReadError::MissingSection) => {}
            Err(err) => panic!("reading {}: {err}", object.display()),
        }
    }
    assert_eq!(
        hits.len(),
        1,
        "expected one object with .cvlr_rules, found {hits:?} among {objects:?}"
    );
    let (_, rules) = &hits[0];
    assert_eq!(
        rules.bytes,
        rule_record("rule_alpha"),
        "object section bytes are not the tag-1 record for rule_alpha"
    );
    assert_eq!(
        rules.names.iter().map(String::as_str).collect::<Vec<_>>(),
        ["rule_alpha"]
    );
    assert_eq!(rules.anchor_count, 0);
}

#[test]
fn rule_manifest_survives_certora_sbf() {
    if !certora_sbf_on_path() || !platform_tools_v143_installed() {
        eprintln!(
            "skipping rule_manifest_survives_certora_sbf: cargo-certora-sbf is not on PATH or platform tools v1.43 are not installed"
        );
        return;
    }

    let staged = Stage::new("sbf");
    let fixture = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/rule_manifest");
    copy_tree(&fixture, &staged.path);
    let manifest_path = staged.path.join("Cargo.toml");
    let manifest = fs::read_to_string(&manifest_path).unwrap();
    let cvlr = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    fs::write(
        &manifest_path,
        manifest.replace("../../../cvlr", &cvlr.display().to_string()),
    )
    .unwrap();

    // Inside the package so certora-sbf can report `executables` relative to
    // `project_directory`. A target dir outside that prefix makes it fail.
    let target = staged.path.join("target");
    let output = cargo(
        &staged.path,
        &[
            "certora-sbf",
            "--json",
            "--no-rustup",
            "--no-tools-install",
            "--tools-version",
            "v1.43",
            "--manifest-path",
            manifest_path.to_str().unwrap(),
            "--features",
            "certora",
        ],
        &target,
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        output.status.success(),
        "cargo certora-sbf failed:\n{}\n{}",
        tail(&output.stderr),
        stdout,
    );
    let project = json_string(&stdout, "project_directory")
        .unwrap_or_else(|| panic!("build JSON has no project_directory:\n{stdout}"));
    let executables = json_string(&stdout, "executables")
        .unwrap_or_else(|| panic!("build JSON has no executables:\n{stdout}"));
    let artifact = {
        let path = PathBuf::from(&executables);
        if path.is_absolute() {
            path
        } else {
            PathBuf::from(project).join(path)
        }
    };
    let rules = section::read_rules(&artifact)
        .unwrap_or_else(|err| panic!("reading {}: {err}", artifact.display()));
    assert_eq!(
        rules.names.iter().map(String::as_str).collect::<Vec<_>>(),
        ["rule_alpha", "rule_beta"],
    );
    assert!(!rules.names.contains("rule_cfg_removed"));
    assert!(!rules.names.contains("not_a_rule"));
    assert!(rules.anchor_count >= 1, "tag-0 CV1 anchor is absent");
    assert_eq!(
        rules.load,
        LoadKind::ReadOnly,
        ".cvlr_rules is not in a non-executable PT_LOAD: {rules:?}"
    );
    assert!(
        rules.saw_text_segment,
        ".text is not in a PT_LOAD, so the segment check would be vacuous: {rules:?}"
    );
    assert!(
        !rules.shares_text_segment,
        ".cvlr_rules shares the .text PT_LOAD: {rules:?}"
    );
}

fn certora_sbf_on_path() -> bool {
    // The binary only accepts `--version` when Cargo invokes it as
    // `cargo certora-sbf`. Cargo also finds it under `~/.cargo/bin`.
    Command::new("cargo")
        .args(["certora-sbf", "--version"])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok_and(|status| status.success())
}

fn platform_tools_v143_installed() -> bool {
    let Some(root) = std::env::var_os("CERTORA_PLATFORM_TOOLS_ROOT")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|home| PathBuf::from(home).join(".cache/solana")))
    else {
        return false;
    };
    ["platform-tools-certora", "platform-tools"]
        .into_iter()
        .any(|flavour| {
            root.join("v1.43")
                .join(flavour)
                .join("rust/bin/rustc")
                .is_file()
        })
}

fn workspace_target() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../target")
}

fn cargo(dir: &Path, args: &[&str], target_dir: &Path) -> Output {
    Command::new("cargo")
        .args(args)
        .current_dir(dir)
        .env("CARGO_TARGET_DIR", target_dir)
        .env("CARGO_TERM_COLOR", "never")
        .env("CARGO_INCREMENTAL", "0")
        .output()
        .expect("spawn cargo")
}

fn object_files(stdout: &str, package: &str) -> Vec<PathBuf> {
    let mut objects = Vec::new();
    for line in stdout.lines() {
        if !line.contains("\"compiler-artifact\"") || !line.contains(package) {
            continue;
        }
        let Some(names) = json_string_array(line, "filenames") else {
            continue;
        };
        for name in names {
            let path = PathBuf::from(&name);
            if name.ends_with(".o") {
                objects.push(path);
                continue;
            }
            // `cargo rustc -- --emit=obj` writes the object next to the
            // hashed rmeta, but the JSON filenames list the rmeta and rlib.
            if let Some(object) = object_beside_rmeta(&path) {
                objects.push(object);
            }
        }
    }
    objects
}

fn object_beside_rmeta(path: &Path) -> Option<PathBuf> {
    let name = path.file_name()?.to_str()?;
    let stem = name.strip_prefix("lib")?.strip_suffix(".rmeta")?;
    let object = path.with_file_name(format!("{stem}.o"));
    object.is_file().then_some(object)
}

fn json_string(json: &str, key: &str) -> Option<String> {
    let key_pat = format!("\"{key}\"");
    let idx = json.find(&key_pat)?;
    let after = json[idx + key_pat.len()..].trim_start();
    let after = after.strip_prefix(':')?.trim_start();
    parse_json_string(after).map(|(value, _)| value)
}

fn json_string_array(json: &str, key: &str) -> Option<Vec<String>> {
    let key_pat = format!("\"{key}\"");
    let idx = json.find(&key_pat)?;
    let after = json[idx + key_pat.len()..].trim_start();
    let mut rest = after.strip_prefix(':')?.trim_start().strip_prefix('[')?;
    let mut values = Vec::new();
    loop {
        rest = rest.trim_start();
        if rest.starts_with(']') {
            return Some(values);
        }
        if let Some(next) = rest.strip_prefix(',') {
            rest = next;
            continue;
        }
        let (value, next) = parse_json_string(rest)?;
        values.push(value);
        rest = next;
    }
}

fn parse_json_string(input: &str) -> Option<(String, &str)> {
    let rest = input.strip_prefix('"')?;
    let mut out = String::new();
    let mut chars = rest.char_indices();
    while let Some((index, ch)) = chars.next() {
        match ch {
            '\\' => {
                let (_, escaped) = chars.next()?;
                match escaped {
                    '"' => out.push('"'),
                    '\\' => out.push('\\'),
                    '/' => out.push('/'),
                    'n' => out.push('\n'),
                    'r' => out.push('\r'),
                    't' => out.push('\t'),
                    _ => return None,
                }
            }
            '"' => return Some((out, &rest[index + 1..])),
            other => out.push(other),
        }
    }
    None
}

fn copy_tree(src: &Path, dst: &Path) {
    fs::create_dir_all(dst).unwrap();
    for entry in fs::read_dir(src).unwrap() {
        let entry = entry.unwrap();
        let name = entry.file_name();
        if name == "target" || name == "Cargo.lock" {
            continue;
        }
        let to = dst.join(&name);
        if entry.file_type().unwrap().is_dir() {
            copy_tree(&entry.path(), &to);
        } else {
            fs::copy(entry.path(), &to).unwrap();
        }
    }
}

fn tail(bytes: &[u8]) -> String {
    let text = String::from_utf8_lossy(bytes);
    let start = text.len().saturating_sub(8_000);
    text[start..].to_string()
}

struct Stage {
    path: PathBuf,
}

impl Stage {
    fn new(label: &str) -> Self {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "cvlr-rule-manifest-{label}-{}-{nanos}",
            std::process::id()
        ));
        fs::create_dir_all(&path).unwrap();
        Self { path }
    }
}

impl Drop for Stage {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}
