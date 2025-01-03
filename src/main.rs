use std::{
    io::{stdout, Write},
    path::PathBuf,
};

use anyhow::Result;
use clap::Parser;
use crossterm::{
    event::read,
    terminal::{Clear, ClearType},
};
use editor::Editor;
use utils::cli::{cleanup_terminal, init_terminal};

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg()]
    path: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let mut stdout = stdout();
    init_terminal()?;

    write!(stdout, "{}", Clear(ClearType::All))?;
    stdout.flush().unwrap();

    let path = PathBuf::from(args.path);
    let mut editor = Editor::open(path.clone());
    editor.draw(&mut stdout)?;

    loop {
        let event = read()?;
        if editor.on_event(event) {
            break;
        }
        editor.draw(&mut stdout)?;
    }

    cleanup_terminal()?;

    Ok(())
}
