use clap::Shell;
use std::{env,path};
use std::fs::File;

include!("src/cli.rs");

fn main() {
    let outdir = env::var_os("OUT_DIR").unwrap();
    let mut app = build_cli();

    // Create dummy file to allow us to find this directory later via CI.
    let stamp_path = path::Path::new(&outdir).join("preferences-stamp");
    if let Err(err) = File::create(&stamp_path) {
        panic!("Failed to write {}: {}", stamp_path.display(), err);
    }

    app.gen_completions("preferences", Shell::Bash, &outdir);
    app.gen_completions("preferences", Shell::Fish, &outdir);
    app.gen_completions("preferences", Shell::Zsh, &outdir);
}
