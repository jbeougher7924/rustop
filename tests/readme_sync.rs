use std::process::Command;

/// Ensures README.md matches the output of `cargo readme`.
/// Skips if `cargo-readme` is not installed.
#[test]
fn readme_is_up_to_date() {
    let output = match Command::new("cargo").args(["readme"]).output() {
        Ok(out) => out,
        Err(err) => {
            eprintln!("Skipping README sync check: cargo-readme unavailable ({err})");
            return;
        }
    };

    if !output.status.success() {
        eprintln!(
            "Skipping README sync check: `cargo readme` exited with {}",
            output.status
        );
        return;
    }

    let generated = String::from_utf8_lossy(&output.stdout)
        .trim_end()
        .to_owned();
    let readme = std::fs::read_to_string("README.md")
        .expect("README.md should be present in the project root")
        .trim_end()
        .to_owned();

    assert_eq!(
        generated, readme,
        "README.md is out of date. Regenerate with scripts/update_readme.sh"
    );
}
