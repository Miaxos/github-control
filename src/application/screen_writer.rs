use std::cell::Cell;
use std::convert::TryInto;
use std::io::{Stdin, Stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::RawTerminal;

pub struct ScreenWriter<'a> {
    /// Current selected line
    current_line: Cell<usize>,
    tupples: &'a mut Vec<(String, String)>,
}

impl<'c> ScreenWriter<'c> {
    pub fn new<'a>(
        current_line: usize,
        tupples: &'a mut Vec<(String, String)>,
    ) -> ScreenWriter<'a> {
        ScreenWriter {
            current_line: Cell::new(current_line),
            tupples,
        }
    }

    pub fn update_tupple(&mut self, tupples: &'c mut Vec<(String, String)>) -> () {
        (*self).tupples = tupples;
    }

    pub fn clear_screen(stdout: &mut RawTerminal<Stdout>) -> () {
        write!(
            *stdout,
            "{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
        )
        .unwrap();
    }

    pub fn cursor_hide(stdout: &mut RawTerminal<Stdout>) -> () {
        write!(*stdout, "{}", termion::cursor::Hide,).unwrap();
    }

    pub fn cursor_show(stdout: &mut RawTerminal<Stdout>) -> () {
        write!(*stdout, "{}", termion::cursor::Show,).unwrap();
    }

    pub fn go_to_begining(&self, stdout: &mut RawTerminal<Stdout>) -> () {
        write!(*stdout, "{}", termion::cursor::Goto(1, 1)).unwrap();
    }

    pub fn print_tupple(&self, stdout: &mut RawTerminal<Stdout>) -> () {
        // Reset selected line
        if self.current_line.get() > self.tupples.len() - 1 {
            self.current_line
                .set(self.tupples.len().try_into().unwrap());
        }

        let _bl = self
            .tupples
            .iter()
            .enumerate()
            .map(|(i, x)| {
                write!(
                    *stdout,
                    "[{}]: {}{}",
                    match self.current_line.get() == i {
                        true => "x",
                        false => "-",
                    },
                    x.0,
                    termion::cursor::Goto(1, (i + 2).try_into().unwrap()),
                )
                .unwrap();
            })
            .collect::<Vec<_>>();
        (*stdout).flush().unwrap();
    }

    pub fn waiting_keys(&self, stdin: Stdin, stdout: &mut RawTerminal<Stdout>) -> () {
        for c in stdin.keys() {
            match c.unwrap() {
                Key::Ctrl('c') => break,
                Key::Char('q') => break,
                Key::Esc => break,
                Key::Char('o') => {
                    let current_url = (*(*self).tupples).get(self.current_line.get());
                    match current_url {
                        Some((_, url)) => {
                            let _bl = webbrowser::open(url);
                        }
                        _ => {}
                    };
                }
                Key::Char('k') => {
                    if self.current_line.get() > 0 {
                        self.current_line.set(self.current_line.get() - 1)
                    }
                }
                Key::Char('j') => {
                    if self.current_line.get() < self.tupples.len() - 1 {
                        self.current_line.set(self.current_line.get() + 1)
                    }
                }
                _ => {}
            }
            self.go_to_begining(stdout);
            self.print_tupple(stdout);
        }
    }
}
