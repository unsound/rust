#[cfg(feature = "rustc")]
use std::sync::atomic::Ordering;
#[cfg(feature = "rustc")]
use ui_test::{run_tests, Config};

#[cfg(feature = "rustc")]
fn main() -> ui_test::color_eyre::Result<()> {
    let config = Config::rustc("examples_tests/rustc_basic");
    let abort_check = config.abort_check.clone();
    ctrlc::set_handler(move || abort_check.store(true, Ordering::Relaxed))?;

    // Compile all `.rs` files in the given directory (relative to your
    // Cargo.toml) and compare their output against the corresponding
    // `.stderr` files.
    run_tests(config)
}

#[cfg(not(feature = "rustc"))]
fn main() -> ui_test::color_eyre::Result<()> {
    Ok(())
}
