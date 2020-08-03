use std::convert::TryInto;
use std::io::{stdin, stdout, Stdout, Write};
use termion::color;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

fn reset_show(stdout: &mut RawTerminal<Stdout>, prs: &Vec<String>, current_line: usize) -> () {
    write!(
        *stdout,
        "{}{}{}{}",
        color::Fg(color::White),
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        termion::cursor::Hide,
    )
    .unwrap();

    let _bl = (*prs)
        .iter()
        .enumerate()
        .map(|(i, x)| {
            write!(
                *stdout,
                "[{}]: {}{}",
                match i == current_line {
                    true => "x",
                    false => "-",
                },
                x,
                termion::cursor::Goto(1, (i + 2).try_into().unwrap()),
            )
            .unwrap();
        })
        .collect::<Vec<_>>();

    (*stdout).flush().unwrap();
}

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    let mut current_line: usize = 0;

    let prs: Vec<String> = vec![
        format!("Ceci est un test"),
        format!("Ceci est un test"),
        format!("Ceci est un test"),
        format!("Ceci est un test"),
    ];

    let max_count = prs.len() - 1;
    let min_count = 0;

    reset_show(&mut stdout, &prs, current_line);

    for c in stdin.keys() {
        /*
        write!(
            stdout,
            "{}{}",
            termion::cursor::Goto(1, 1),
            termion::clear::CurrentLine
        )
        .unwrap();
        */

        match c.unwrap() {
            Key::Ctrl('c') => break,
            Key::Char('q') => break,
            Key::Char('k') => {
                if current_line > min_count {
                    current_line = current_line - 1;
                }
            }
            Key::Char('j') => {
                if current_line < max_count {
                    current_line = current_line + 1;
                }
            }
            _ => {}
        }
        reset_show(&mut stdout, &prs, current_line);
    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();
}