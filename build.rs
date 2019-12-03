use clap::Shell;
use std::env;

include!("src/cli.rs");

fn main() {
    let outdir = env::var_os("OUT_DIR").unwrap();
    let mut app = build_cli();
    app.gen_completions("apply-user-defaults", Shell::Bash, &outdir);
    app.gen_completions("apply-user-defaults", Shell::Fish, &outdir);
    app.gen_completions("apply-user-defaults", Shell::Zsh, &outdir);
}
