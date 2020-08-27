mod application;
mod infrastructure;

use application::configuration::config_file::ApplicationConfiguration;
use clap::clap_app;
use std::io::{stdin, stdout};
use std::process::exit;
use termion::raw::IntoRawMode;

fn main() {
    let matches = clap_app!(ghubcontrol =>
        (version: "0.1.0")
        (author: "Anthony Griffon <an.griffon@gmail.com>")
        (about: "Get your current PRs from github inside your terminal")
        (@arg refresh: -r --refresh +takes_value "Will refresh PRs every X seconds")
    )
    .get_matches();

    let _refresh_value = matches.value_of("refresh");
    // todo!("Add thread::spawn wich will update prs");

    let cfg = ApplicationConfiguration::load_or_exit();

    let stdin = stdin();
    let mut stdout = match stdout().into_raw_mode() {
        Ok(stdout) => stdout,
        Err(_) => {
            println!("Can't capture stdout raw mode, try to avoid piping this command");
            exit(exitcode::IOERR);
        }
    };

    application::screen_writer::ScreenWriter::cursor_hide(&mut stdout);
    application::screen_writer::ScreenWriter::clear_screen(&mut stdout);

    let mut prs: Vec<(String, String)> =
        match infrastructure::github::github::get_prs_from_github(cfg.github_key()) {
            Ok(result) => result,
            Err(err) => {
                println!("[ERROR]:{}", err);
                panic!();
            }
        };

    let writer = application::screen_writer::ScreenWriter::new(0, &mut prs);

    writer.go_to_begining(&mut stdout);
    writer.print_tupple(&mut stdout);
    writer.waiting_keys(stdin, &mut stdout);
    application::screen_writer::ScreenWriter::cursor_show(&mut stdout);
}
