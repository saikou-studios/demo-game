extern crate embed_resource;
use std::{env, path::PathBuf, process::Command};

fn main() {
    set_rerun();
    set_commit_info();
    set_resources();
}

fn set_resources() {
    let target = env::var("TARGET").unwrap();
    if target.contains("windows") {
        embed_resource::compile("build/windows/icon.rc", embed_resource::NONE);
    }
}

fn set_rerun() {
    println!("cargo:rerun-if-env-changed=CFG_RELEASE");

    let mut manifest_dir = PathBuf::from(
        env::var("CARGO_MANIFEST_DIR").expect("`CARGO_MANIFEST_DIR` is always set by cargo."),
    );

    while manifest_dir.parent().is_some() {
        let head_ref = manifest_dir.join(".git/HEAD");
        if head_ref.exists() {
            println!("cargo:rerun-if-changed={}", head_ref.display());
            return;
        }

        manifest_dir.pop();
    }

    println!("cargo:warning=Could not find `.git/HEAD` from manifest dir!");
}

fn set_commit_info() {
    let output = match Command::new("git")
        .arg("log")
        .arg("-1")
        .arg("--date=short")
        .arg("--format=%H %h %cd")
        .output()
    {
        Ok(output) if output.status.success() => output,
        _ => return,
    };
    let stdout = String::from_utf8(output.stdout).unwrap();
    let mut parts = stdout.split_whitespace();
    let mut next = || parts.next().unwrap();
    println!("cargo:rustc-env=RA_COMMIT_HASH={}", next());
    println!("cargo:rustc-env=RA_COMMIT_SHORT_HASH={}", next());
    println!("cargo:rustc-env=RA_COMMIT_DATE={}", next())
}
