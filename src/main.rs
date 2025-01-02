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
    path: String,
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

    let path = PathBuf::from(args.path);
    let mut editor = Editor::open(path.clone());
    editor.draw(&mut stdout);

    for evt in stdin.events() {
        if editor.on_event(evt.unwrap(), path.clone()) {
            return Ok(());
        }
        editor.draw(&mut stdout);
    }

    Ok(())
}
