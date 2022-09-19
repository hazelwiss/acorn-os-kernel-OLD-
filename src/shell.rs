use crate::{
    drivers::console,
    kapi::{
        drivers::{irq, ps2},
        keyboard::{Key, KeyState},
    },
};
use alloc::{string::String, vec::Vec};
use core::mem;
use spin::Mutex;

struct Shell {
    log_lines: Vec<String>,
    cur_line: String,
    input_str: String,
}

impl Shell {
    fn putc(&mut self, c: char) {
        self.putc_no_reapply(c);
        self.reapply_to_console();
    }

    fn puts(&mut self, s: &str) {
        for c in s.chars() {
            self.putc_no_reapply(c);
            self.reapply_to_console();
        }
    }

    #[inline]
    fn putc_no_reapply(&mut self, c: char) {
        match c {
            '\n' => self.new_line(),
            c => self.cur_line.push(c),
        }
        let dimensions = console::get_dimensions();
        if self.cur_line.len() >= dimensions.width {
            self.new_line()
        }
    }

    fn getc(&self) {
        todo!()
    }

    #[inline]
    fn new_line(&mut self) {
        let mut move_str = String::with_capacity(console::get_dimensions().width);
        mem::swap(&mut self.cur_line, &mut move_str);
        self.log_lines.push(move_str);
    }

    fn reapply_to_console(&self) {
        console::clear();
        let log_max_lines = console::get_dimensions().height - 2;
        let start_index = if self.log_lines.len() < log_max_lines {
            0
        } else {
            self.log_lines.len() - log_max_lines
        };
        let len = self.log_lines.len();
        let slice = &self.log_lines[start_index..len];
        for line in slice {
            console::puts(&line);
            console::newline();
        }
        console::puts(&self.cur_line);
        console::puts(&format!("> {}", self.input_str));
    }

    fn clear(&mut self) {
        self.log_lines = vec![];
        self.cur_line = String::new();
    }

    fn help(&self) {}

    fn pressed(&mut self, input: Key) {
        match input {
            Key::Enter => {
                self.cur_line.push_str(&self.input_str);
                self.new_line();
                match self.input_str.as_str() {
                    "help" => self.help(),
                    "clear" => self.clear(),
                    _ => self.puts(&format!("invalid command '{}'\n", self.input_str)),
                }
                self.input_str = String::new();
            }
            Key::Backspace => {
                self.input_str.pop();
            }
            c if c.as_char().is_some() => {
                let c = unsafe { c.as_char().unwrap_unchecked() };
                self.input_str.push(c);
            }
            _ => {}
        }
        self.reapply_to_console();
    }
}

static LOG: Mutex<Shell> = Mutex::new(Shell {
    log_lines: vec![],
    cur_line: String::new(),
    input_str: String::new(),
});

pub fn putc(c: char) {
    LOG.lock().putc(c);
}

pub fn puts(s: &str) {
    LOG.lock().puts(s);
}

pub fn new_line() {
    LOG.lock().new_line();
}

pub fn getc() -> char {
    todo!()
}

pub fn run() {
    LOG.lock().reapply_to_console();
    loop {
        irq::wait();
        if let Some(c) = ps2::getk() {
            match c {
                KeyState::Pressed(c) => LOG.lock().pressed(c),
                _ => {}
            }
        }
    }
}
