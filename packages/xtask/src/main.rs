use std::{
    collections::VecDeque,
    env,
    ffi::{OsStr, OsString},
    path::PathBuf,
    process::{Command, exit},
};

const PROJECT_DIR: &str = env!("CARGO_MANIFEST_DIR");

fn main() -> anyhow::Result<()> {
    let mut args = env::args_os().skip(1);

    let command = args
        .next()
        .unwrap_or_default()
        .into_string()
        .unwrap_or_default();
    if command != "run" {
        eprintln!("Usage: cargo xtask run [...args]");
        exit(1);
    };

    let mut params: VecDeque<OsString> = args.collect();

    let mut release = false;
    while let Some(param) = params.front() {
        if param == "--release" || param == "-r" {
            release = true;
        } else {
            break;
        }

        params.pop_front();
    }

    let cargo = env::var_os("CARGO").unwrap();
    build_kernel(&cargo, release)?;
    run_sim(&cargo, release, &Vec::from(params))?;

    Ok(())
}

fn build_kernel(cargo: &OsStr, release: bool) -> anyhow::Result<()> {
    let mut cmd = Command::new(cargo);
    cmd.arg("build");
    if release {
        cmd.arg("--release");
    }
    cmd.current_dir(PathBuf::from(PROJECT_DIR).join("../kernel"));

    let status = cmd.status()?;
    if !status.success() {
        exit(status.code().unwrap_or(1));
    }

    Ok(())
}

fn run_sim(cargo: &OsStr, release: bool, args: &[OsString]) -> anyhow::Result<()> {
    let mut cmd = Command::new(cargo);
    cmd.args(["run", "--manifest-path"]);
    cmd.arg(
        PathBuf::from(PROJECT_DIR)
            .join("../client-cli/Cargo.toml")
            .display()
            .to_string(),
    );

    if release {
        cmd.arg("--release");
    }

    cmd.arg("--");
    cmd.args(args);

    let status = cmd.status()?;
    if !status.success() {
        exit(status.code().unwrap_or(1));
    }

    Ok(())
}
