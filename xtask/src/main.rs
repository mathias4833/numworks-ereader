use std::env;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

const TARGET: &str = "x86_64-unknown-linux-gnu";
const CRATE_LIB_NAME: &str = "libnumworks_ereader.so";

fn main() -> ExitCode {
    let project_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask must be inside the project root")
        .to_path_buf();

    let simulator = env::var_os("EPSILON_SIMULATOR")
        .map(PathBuf::from)
        .unwrap_or_else(|| project_root.join("./epsilon.bin"));

    let shared_lib = project_root
        .join("target")
        .join(TARGET)
        .join("debug")
        .join(CRATE_LIB_NAME);

    let status = Command::new("cargo")
        .current_dir(&project_root)
        .args(["build", "--lib", "--target", TARGET])
        .status()
        .expect("failed to run cargo build");

    if !status.success() {
        return ExitCode::FAILURE;
    }

    let status = Command::new(&simulator)
        .current_dir(&project_root)
        .arg("--nwb")
        .arg(&shared_lib)
        .status()
        .expect("failed to run simulator");

    if status.success() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
