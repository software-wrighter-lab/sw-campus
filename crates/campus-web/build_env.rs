use std::process::Command;

pub fn emit_build_env_vars() {
    let host = run_cmd("hostname", &[]);
    let sha = run_cmd("git", &["rev-parse", "--short", "HEAD"]);
    let timestamp = run_cmd("date", &["-u", "+%Y-%m-%dT%H:%M:%SZ"]);
    println!("cargo:rustc-env=BUILD_HOST={host}");
    println!("cargo:rustc-env=BUILD_SHA={sha}");
    println!("cargo:rustc-env=BUILD_TIMESTAMP={timestamp}");
}

fn run_cmd(program: &str, args: &[&str]) -> String {
    Command::new(program).args(args).output().map_or_else(
        |_| "unknown".to_owned(),
        |output| String::from_utf8_lossy(&output.stdout).trim().to_owned(),
    )
}
