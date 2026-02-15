use assert_cmd::Command;

fn normalize(s: &str) -> String {
    s.replace("\r\n", "\n")
}

fn stderr_kind(stderr: &str) -> &'static str {
    if stderr.is_empty() {
        "empty"
    } else if stderr.contains("Parse error:") {
        "parse_error"
    } else if stderr.contains("Undefined variable:") {
        "undefined_variable"
    } else {
        "other"
    }
}

fn stdout_flag(stdout: &str, needle: &str) -> bool {
    stdout.contains(needle)
}

fn run_case(case: &str, fixture: &str) -> String {
    let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let fixture_path = manifest_dir.join("tests").join("fixtures").join(fixture);

    let output = Command::cargo_bin("agentc")
        .expect("agentc binary should build")
        .arg(&fixture_path)
        .output()
        .expect("should run agentc");

    let exit = output.status.code().unwrap_or(-1);
    let stdout = normalize(&String::from_utf8_lossy(&output.stdout));
    let stderr = normalize(&String::from_utf8_lossy(&output.stderr));

    // Keep snapshots stable by recording *signals* rather than full raw stdout.
    let has_parsed = stdout_flag(&stdout, "✓ Parsed successfully");
    let has_typechecked = stdout_flag(&stdout, "✓ Type checked successfully");
    let has_compiled = stdout_flag(&stdout, "✓ Compiled to bytecode");
    let has_exec_banner = stdout_flag(&stdout, "Executing...");
    let has_exec_agent = stdout_flag(&stdout, "Executing agent:");
    let has_handler = stdout_flag(&stdout, "  Handler:");

    let kind = stderr_kind(&stderr);

    format!(
        "case: {case}\nexit: {exit}\nstderr_kind: {kind}\nstdout_flags:\n- parsed_ok: {has_parsed}\n- typechecked_ok: {has_typechecked}\n- compiled_ok: {has_compiled}\n- exec_banner: {has_exec_banner}\n- exec_agent: {has_exec_agent}\n- handler: {has_handler}\n",
    )
}

#[test]
fn ok_minimal_snapshot() {
    let s = run_case("ok_minimal", "ok_minimal.agent");
    insta::assert_snapshot!(s, @r###"case: ok_minimal
exit: 0
stderr_kind: empty
stdout_flags:
- parsed_ok: true
- typechecked_ok: true
- compiled_ok: true
- exec_banner: true
- exec_agent: true
- handler: true
"###);
}

#[test]
fn type_error_undefined_var_snapshot() {
    let s = run_case("type_error_undefined_var", "type_error_undefined_var.agent");
    insta::assert_snapshot!(s, @r###"case: type_error_undefined_var
exit: 1
stderr_kind: undefined_variable
stdout_flags:
- parsed_ok: true
- typechecked_ok: false
- compiled_ok: false
- exec_banner: false
- exec_agent: false
- handler: false
"###);
}

#[test]
fn parse_error_missing_semicolon_snapshot() {
    let s = run_case(
        "parse_error_missing_semicolon",
        "parse_error_missing_semicolon.agent",
    );
    insta::assert_snapshot!(s, @r###"case: parse_error_missing_semicolon
exit: 1
stderr_kind: parse_error
stdout_flags:
- parsed_ok: false
- typechecked_ok: false
- compiled_ok: false
- exec_banner: false
- exec_agent: false
- handler: false
"###);
}
