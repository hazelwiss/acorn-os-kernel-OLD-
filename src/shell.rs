use crate::drivers::console;
use alloc::{
    string::{String, ToString},
    vec::Vec,
};
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

pub fn getc() -> char {
    todo!()
}

pub fn run() {
    LOG.lock().reapply_to_console();
    let mut string = String::new();
    loop {}
}
