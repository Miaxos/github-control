use std::cell::Cell;
use std::convert::TryInto;
use std::io::{Stdout, Write};
use termion::raw::RawTerminal;

pub struct ScreenWriter {
    /// Current selected line
    current_line: Cell<usize>,
    tupples: Vec<(String, String)>,
}

impl ScreenWriter {
    pub fn new(current_line: usize, tupples: Vec<(String, String)>) -> ScreenWriter {
        ScreenWriter {
            current_line: Cell::new(current_line),
            tupples,
        }
    }

    pub fn update_tupple(&mut self, tupples: Vec<(String, String)>) -> () {
        (*self).tupples = tupples;
        let len = (*self).tupples.len();
        if self.current_line.get() > len - 1 {
            self.current_line.set(len - 1);
        }
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

    pub fn go_to_begining(stdout: &mut RawTerminal<Stdout>) -> () {
        write!(*stdout, "{}", termion::cursor::Goto(1, 1)).unwrap();
    }

    pub fn print_tupple(&self, stdout: &mut RawTerminal<Stdout>) -> () {
        if self.tupples.is_empty() {
            write!(*stdout, "No PRs...").unwrap();
            return ();
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

    /**
     * Get current url from PRs
     */
    pub fn get_current_url(&self) -> Option<String> {
        (*(*self).tupples)
            .get(self.current_line.get())
            .map(|x| x.1.clone())
    }

    /**
     * Try to go to the previous line.
     */
    pub fn try_previous_line(&self) -> () {
        if self.current_line.get() > 0 {
            self.current_line.set(self.current_line.get() - 1)
        }
    }

    /**
     * Try to go to the next line.
     */
    pub fn try_next_line(&self) -> () {
        if self.current_line.get() < self.tupples.len() - 1 {
            self.current_line.set(self.current_line.get() + 1)
        }
    }
}
