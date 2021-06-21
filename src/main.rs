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

fn main() {
    let matches = App::new("CLI Game Of Life")
        .version(crate_version!())
        .author("blabtonic")
        .about("Simple implementation of Conway's Game of Life in rust. Rest in Peace Johm Conway.")
        .after_help("No win conditions have fun!")
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to configure initial state")
                .short("i")
                .long("input")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("DELAY")
                .help("Sets the delay between game ticks. Value is in miliseconds")
                .short("d")
                .long("delay")
                .takes_value(true)
                .default_value("500"),
        )
        .get_matches();
    let mut stdout = stdout();
    // What is this? unwrap -> parse -> unwrap WHY???
    let delay: u64 = matches.value_of("DELAY").unwrap().parse().unwrap();
}
