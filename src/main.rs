mod application;
mod infrastructure;

use application::configuration::config_file::ApplicationConfiguration;
use std::io::{stdin, stdout};
use termion::raw::IntoRawMode;

fn main() {
    let cfg = ApplicationConfiguration::load_or_exit();

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    application::screen_writer::ScreenWriter::cursor_hide(&mut stdout);
    application::screen_writer::ScreenWriter::clear_screen(&mut stdout);

    let mut prs: Vec<(String, String)> =
        match infrastructure::github::github::get_truc(cfg.github_key()) {
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
