use std::{path::Path, process::Stdio};

use test_case::test_case;

#[test_case("empty")]
#[test_case("case-01")]
#[test_case("case-02")]
#[test_case("case-03")]
fn run_it(case_name: &str) {
    #[cfg(debug_assertions)]
    const RELEASE_OPT: Option<&str> = None;
    #[cfg(not(debug_assertions))]
    const RELEASE_OPT: Option<&str> = Some("--release");

    let input_file = Path::new(file!())
        .parent()
        .expect("file!().parent")
        .join("cases")
        .join(format!("{}.csv", case_name));
    let child = std::process::Command::new("cargo")
        .arg("run")
        .args(RELEASE_OPT)
        .arg("--")
        .arg(input_file)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("Command::spawn");
    let outcome = child.wait_with_output().expect("wait with output");

    let stdout = String::from_utf8_lossy(&outcome.stdout).into_owned();

    let mut output_lines = stdout.lines().collect::<Vec<_>>();
    if output_lines.len() > 1 {
        output_lines[1..].sort();
    }
    insta::with_settings!({
        snapshot_path => "cases",
        prepend_module_to_snapshot => false,
    }, {
        insta::assert_snapshot!(case_name, output_lines.join("\n"));
    });
}
