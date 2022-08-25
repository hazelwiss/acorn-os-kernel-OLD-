use crate::drivers;
use alloc::{string::String, vec::Vec};
use spin::Mutex;

struct Shell {
    log_lines: Vec<String>,
    cur_line: String,
}

impl Shell {
    fn putc(&mut self, c: char) {
        match c {
            '\n' => self.new_line(),
            c => self.cur_line.push(c),
        }
        let dimensions = drivers::console::get_dimensions();
        if self.cur_line.len() >= dimensions.width {
            self.new_line()
        }
    }

    fn puts(&mut self, s: &str) {
        for c in s.chars() {
            self.putc(c);
        }
    }

    fn getc(&self) {
        todo!()
    }

    fn new_line(&mut self) {
        let mut move_str = String::new();
        core::mem::swap(&mut self.cur_line, &mut move_str);
        self.log_lines.push(move_str);
    }
}

static LOG: Mutex<Shell> = Mutex::new(Shell {
    log_lines: vec![],
    cur_line: String::new(),
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
    let mut string = String::new();
    loop {
        log!("> ");
        loop {}
    }
}
