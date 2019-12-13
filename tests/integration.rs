extern crate assert_cli;
use std::env;
use std::path;

#[test]
fn dry_run() {
    let defaults_path = asset_path().join("defaults.yml");
    let expected_output = format!(
        "==> defaults write com.apple.desktopservices DSDontWriteNetworkStores -bool true
==> defaults write com.apple.dock autohide -bool true
==> defaults write com.apple.finder NewWindowTarget -string PfHm
==> defaults write com.apple.finder NewWindowTargetPath -string file://{}",
        env::var("HOME").unwrap()
    );
    assert_cli::Assert::main_binary()
        .with_args(&[defaults_path.to_str().unwrap(), "--dry-run"])
        .stdout()
        .is(expected_output.as_str())
        .unwrap();
}

#[test]
fn dry_run_no_env() {
    let defaults_path = asset_path().join("defaults.yml");
    let expected_output =
        "==> defaults write com.apple.desktopservices DSDontWriteNetworkStores -bool true
==> defaults write com.apple.dock autohide -bool true
==> defaults write com.apple.finder NewWindowTarget -string PfHm
==> defaults write com.apple.finder NewWindowTargetPath -string file://${HOME}";
    assert_cli::Assert::main_binary()
        .with_args(&[defaults_path.to_str().unwrap(), "--dry-run", "--no-env"])
        .stdout()
        .is(expected_output)
        .unwrap();
}

#[test]
fn dry_run_escaped_env() {
    let defaults_path = asset_path().join("defaults-escaped.yml");
    let expected_output =
        "==> defaults write com.apple.desktopservices DSDontWriteNetworkStores -bool true
==> defaults write com.apple.dock autohide -bool true
==> defaults write com.apple.finder NewWindowTarget -string PfHm
==> defaults write com.apple.finder NewWindowTargetPath -string file://${HOME}";
    assert_cli::Assert::main_binary()
        .with_args(&[defaults_path.to_str().unwrap(), "--dry-run"])
        .stdout()
        .is(expected_output)
        .unwrap();
}

#[inline]
fn asset_path() -> path::PathBuf {
    path::Path::new(file!()).parent().unwrap().join("assets")
}
