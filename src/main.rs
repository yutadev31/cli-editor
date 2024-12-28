use std::{
    io::{stdin, stdout, Write},
    path::PathBuf,
};

use anyhow::Result;
use clap::Parser;
use editor::Editor;
use termion::{clear, input::TermRead, raw::IntoRawMode, screen::IntoAlternateScreen};

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg()]
    path: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let stdin = stdin();
    let mut stdout = stdout()
        .into_raw_mode()
        .unwrap()
        .into_alternate_screen()
        .unwrap();

    write!(stdout, "{}", clear::All)?;

    stdout.flush().unwrap();

    let mut editor = match args.path {
        Some(path) => Editor::open(PathBuf::from(path))?,
        None => Editor::new(),
    };

    editor.draw(&mut stdout);

    for evt in stdin.events() {
        if editor.on_event(evt.unwrap()) != 0 {
            return Ok(());
        };
        editor.draw(&mut stdout);
    }

    return Ok(());
}
