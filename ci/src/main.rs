use anyhow::bail;
use bitflags::bitflags;
use xshell::{cmd, Shell};

mod utils;
use utils::*;

bitflags! {
    #[derive(Clone, Copy)]
    struct Check: u32 {
        const CHECK = 0b00000001;
        const WASM_CHECK = 0b00000010;
        const EXAMPLE_CHECK = 0b00000100;
        const FMT = 0b00001000;
        const TEST = 0b00010000;
        const DOC_TEST = 0b00100000;
        const DOC_CHECK = 0b01000000;
        const CLIPPY = 0b10000000;
    }
}

fn main() -> anyhow::Result<()> {
    std::env::set_var("RUSTFLAGS", "-D warnings");

    let arguments = [
        ("check", Check::CHECK),
        ("wasm-check", Check::WASM_CHECK),
        ("example-check", Check::EXAMPLE_CHECK),
        ("fmt", Check::FMT),
        ("test", Check::TEST),
        ("doc-test", Check::DOC_TEST),
        ("doc-check", Check::DOC_CHECK),
        ("clippy", Check::CLIPPY),
    ];

    let what_to_run = if let Some(arg) = std::env::args().nth(1).as_deref() {
        if let Some((_, check)) = arguments.iter().find(|(str, _)| *str == arg) {
            *check
        } else {
            bail!(
                "Invalid argument: {arg:?}.\nEnter one of: {}.",
                arguments[1..]
                    .iter()
                    .map(|(s, _)| s)
                    .fold(arguments[0].0.to_owned(), |c, v| c + ", " + v)
            );
        }
    } else {
        Check::all()
    };

    let sh = Shell::new()?;
    if what_to_run.contains(Check::CHECK) {
        check(&sh, Target::Default, Features(&["libc", "mint"]))?;
    }
    if what_to_run.contains(Check::WASM_CHECK) {
        check(&sh, Target::Wasm, Features(&["mint"]))?;
    }
    if what_to_run.contains(Check::EXAMPLE_CHECK) {
        example_check(&sh)?;
    }
    if what_to_run.contains(Check::FMT) {
        fmt(&sh)?;
    }
    if what_to_run.contains(Check::TEST) {
        test(&sh, Features(&["libc", "mint"]))?;
    }
    if what_to_run.contains(Check::DOC_TEST) {
        doc_test(&sh)?;
    }
    if what_to_run.contains(Check::DOC_CHECK) {
        doc_check(&sh)?;
    }
    if what_to_run.contains(Check::CLIPPY) {
        clippy(&sh)?;
    }
    Ok(())
}

fn check(sh: &Shell, target: Target, features: Features) -> anyhow::Result<()> {
    let target_flags = &target.flags();
    let feature_combination_flags = features.combination_flags();
    for feature_flags in feature_combination_flags.iter() {
        cmd!(sh, "cargo check {target_flags...} {feature_flags...}").run()?;
    }
    Ok(())
}

fn example_check(sh: &Shell) -> anyhow::Result<()> {
    cmd!(sh, "cargo check --examples").run()?;
    Ok(())
}

fn fmt(sh: &Shell) -> anyhow::Result<()> {
    cmd!(sh, "cargo fmt --all -- --check").run()?;
    Ok(())
}

fn test(sh: &Shell, features: Features) -> anyhow::Result<()> {
    let feature_combination_flags = features.combination_flags();
    for feature_flags in feature_combination_flags.iter() {
        cmd!(
            sh,
            "cargo test --workspace --lib --bins --tests {feature_flags...}"
        )
        .run()?;
    }
    Ok(())
}

fn doc_test(sh: &Shell) -> anyhow::Result<()> {
    cmd!(sh, "cargo test --workspace --doc").run()?;
    Ok(())
}

fn doc_check(sh: &Shell) -> anyhow::Result<()> {
    cmd!(
        sh,
        "cargo doc --workspace --all-features --no-deps --document-private-items"
    )
    .env("RUSTDOCFLAGS", "-Dwarnings")
    .run()?;
    Ok(())
}

fn clippy(sh: &Shell) -> anyhow::Result<()> {
    cmd!(sh, "cargo clippy --workspace --all-targets").run()?;
    Ok(())
}
