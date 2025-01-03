use std::io::stdout;

use anyhow::Result;
use crossterm::{
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, size, EnterAlternateScreen, LeaveAlternateScreen,
    },
};

pub fn init_terminal() -> Result<()> {
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;

    Ok(())
}

pub fn cleanup_terminal() -> Result<()> {
    execute!(stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}

pub fn terminal_size() -> Result<(usize, usize)> {
    let (cols, rows) = size()?;
    Ok((cols as usize, rows as usize))
}
