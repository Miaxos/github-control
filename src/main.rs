mod application;
mod infrastructure;

use application::configuration::config_file::ApplicationConfiguration;
use clap::clap_app;
use std::io::{stdin, stdout};
use std::process::exit;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
    let matches = clap_app!(ghubcontrol =>
        (version: "0.1.0")
        (author: "Anthony Griffon <an.griffon@gmail.com>")
        (about: "Get your current PRs from github inside your terminal with automatic refresh")
        (@arg refresh: -r --refresh +takes_value "Will refresh PRs every X seconds instead of every 60 seconds")
    )
    .get_matches();

    let refresh_value = matches
        .value_of("refresh")
        .map(|x| match x.parse::<u64>() {
            Ok(parsed) => parsed,
            Err(_) => {
                println!("Can't parse refresh value, please enter an integer");
                exit(exitcode::USAGE);
            }
        })
        .unwrap_or(60);

    let cfg = ApplicationConfiguration::load_or_exit();

    let stdin = stdin();
    let mut stdout = match stdout().into_raw_mode() {
        Ok(stdout) => stdout,
        Err(_) => {
            println!("Can't capture stdout raw mode, try to avoid piping this command");
            exit(exitcode::USAGE);
        }
    };

    let prs: Vec<(String, String)> =
        match infrastructure::github::github::get_prs_from_github(cfg.github_key()) {
            Ok(result) => result,
            Err(_) => {
                let _ = stdout.suspend_raw_mode();
                println!("Can't request Github PRs, check if your token is in set up");
                exit(exitcode::USAGE);
            }
        };

    application::screen_writer::ScreenWriter::cursor_hide(&mut stdout);

    let writer = Arc::new(Mutex::new(application::screen_writer::ScreenWriter::new(
        0, prs,
    )));

    let my_writer_1 = Arc::clone(&writer);
    let my_writer_2 = Arc::clone(&writer);

    application::screen_writer::ScreenWriter::clear_screen(&mut stdout);
    application::screen_writer::ScreenWriter::go_to_begining(&mut stdout);
    my_writer_2.lock().unwrap().print_tupple(&mut stdout);

    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(refresh_value));
        let prs2: Vec<(String, String)> =
            match infrastructure::github::github::get_prs_from_github(cfg.github_key()) {
                Ok(result) => result,
                Err(err) => {
                    println!("[ERROR]:{}", err);
                    panic!();
                }
            };
        my_writer_1.lock().unwrap().update_tupple(prs2);

        match std::io::stdout().into_raw_mode() {
            Ok(mut stdout) => {
                application::screen_writer::ScreenWriter::clear_screen(&mut stdout);
                application::screen_writer::ScreenWriter::go_to_begining(&mut stdout);
                my_writer_1.lock().unwrap().print_tupple(&mut stdout);
            }
            Err(_) => {}
        };
    });

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Ctrl('c') => break,
            Key::Char('q') => break,
            Key::Esc => break,
            Key::Char('o') => {
                let current_url = my_writer_2.lock().unwrap().get_current_url();
                match current_url {
                    Some(url) => {
                        let _bl = webbrowser::open(&url);
                    }
                    _ => {}
                };
            }
            Key::Char('k') => {
                my_writer_2.lock().unwrap().try_previous_line();
            }
            Key::Char('j') => {
                my_writer_2.lock().unwrap().try_next_line();
            }
            _ => {}
        }
        application::screen_writer::ScreenWriter::clear_screen(&mut stdout);
        application::screen_writer::ScreenWriter::go_to_begining(&mut stdout);
        my_writer_2.lock().unwrap().print_tupple(&mut stdout);
    }

    application::screen_writer::ScreenWriter::cursor_show(&mut stdout);
}
