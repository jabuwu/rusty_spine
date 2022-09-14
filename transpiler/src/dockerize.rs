use std::{
    ffi::OsStr,
    io,
    process::{Command, Output, Stdio},
};

pub fn docker<I, S>(args: I) -> Result<Output, io::Error>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    Command::new("docker")
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
}

pub fn run() {
    build_container().unwrap();
    run_container(false).unwrap();
}

pub fn run_interactive() {
    build_container().unwrap();
    run_container(true).unwrap();
}

pub fn build_container() -> Result<(), io::Error> {
    docker(["build", ".", "-t", "rusty_spine_transpiler"])?;
    Ok(())
}

pub fn run_container(interactive: bool) -> Result<(), io::Error> {
    let cwd = std::env::current_dir().unwrap();
    let src_volume = format!("{}/src:/transpiler/src", cwd.to_str().unwrap());
    let spine_runtimes_volume = format!("{}/spine-runtimes:/spine-runtimes", cwd.to_str().unwrap());
    let out_volume = format!("{}/out:/out", cwd.to_str().unwrap());
    let transpiler_target_volume = format!(
        "{}/transpiler-target:/transpiler/target",
        cwd.to_str().unwrap()
    );
    let mut args = vec![
        "run",
        "--platform",
        "linux/amd64",
        "--rm",
        "-v",
        &spine_runtimes_volume,
        "-v",
        &out_volume,
        "-v",
        &transpiler_target_volume,
    ];
    if interactive {
        args.push("-it");
        args.push("-v");
        args.push(&src_volume);
    }
    args.push("rusty_spine_transpiler");
    if interactive {
        args.push("bash");
    } else {
        args.push("cargo");
        args.push("run");
        args.push("--");
        args.push("--container");
    }
    docker(args)?;
    Ok(())
}
