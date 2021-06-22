use clap::{crate_version, App, Arg};
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{poll, read, Event, KeyCode, KeyEvent},
    execute, queue,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, ClearType, DisableLineWrap, EnableLineWrap,
        EnterAlternateScreen, LeaveAlternateScreen,
    },
    Result,
};
use std::fs::File;
use std::io::{stdout, Write};
use std::io::{BufRead, BufReader};
use std::time::Duration;

mod game;

fn main() -> Result <()> {
    let mut game = game::Universe::new(5, 5);
    game.set_cells(&[(2, 1), (2, 2), (2, 3)]);
    execute!(
        stdout(),
        EnterAlternateScreen,
        SetForegroundColor(Color::Grey),
        Hide
    )?;

    loop {
        if poll(Duration::from_millis(500))? {
            match read()? {
                Event::Key(_) => break,
                _ => {}
            }
        } else {
            execute!(
                stdout(),
                Clear(ClearType::All),
                MoveTo(0,0),
                Print(&game),
                Print("Press enter to exit...")
            )?;
            game.tick();
        }
    }
    execute!(stdout(), ResetColor, Show, LeaveAlternateScreen)?;
    Ok(())
}
