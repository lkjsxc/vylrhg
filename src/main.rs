mod buffer;
mod commands;
mod editor;
mod fs_io;
mod render;

use std::path::PathBuf;

fn main() -> std::process::ExitCode {
    let path = std::env::args().nth(1).map(PathBuf::from);
    if let Err(err) = editor::run(path.as_deref()) {
        eprintln!("{err}");
        return std::process::ExitCode::from(1);
    }
    std::process::ExitCode::SUCCESS
}
