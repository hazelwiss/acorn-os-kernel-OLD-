use crate::drivers::{
    self,
    interfaces::serial::{self, IConsole},
};

use super::{com, vga};
use core::sync::atomic::{AtomicUsize, Ordering};
use spin::Mutex;

pub const CONSOLE: drivers::Desc = drivers::Desc {
    entry: init,
    iserialin: None,
    iserialout: None,
    console: Some(IConsole {
        putc: putc,
        puts: puts,
        getc: getc,
        clear: clear,
        put_cursor: put_cursor,
        set_colour: set_colour,
    }),
};

const CONSOLE_WIDTH: usize = vga::VGA_WIDTH;
const CONSOLE_HEIGHT: usize = vga::VGA_HEIGHT;
const LOG_LINES: usize = CONSOLE_HEIGHT - 1;

struct Shell {
    log: [[char; CONSOLE_WIDTH]; LOG_LINES],
    start_row: usize,
    rows: usize,
    col_index: usize,
}

static SHELL_INSTANCES: [Mutex<Shell>; 1] = [Mutex::new(Shell {
    log: [[' '; CONSOLE_WIDTH]; LOG_LINES],
    start_row: 0,
    rows: 0,
    col_index: 0,
})];

static CURRENT_TTY: AtomicUsize = AtomicUsize::new(0);

pub fn init() {}

#[inline]
pub fn getc() -> char {
    todo!()
}

pub fn putc(c: char) {
    let index = CURRENT_TTY.load(Ordering::Relaxed);
    let mut tty = SHELL_INSTANCES[index].lock();
    putc_on_tty(&mut tty, c);
    send_to_serial(c);
    update_vga_buf(&mut tty);
}

pub fn puts(s: &str) {
    let index = CURRENT_TTY.load(Ordering::Relaxed);
    let mut tty = SHELL_INSTANCES[index].lock();
    for c in s.chars() {
        putc_on_tty(&mut tty, c);
        send_to_serial(c);
    }
    update_vga_buf(&mut tty);
}

pub fn clear() {}

pub fn put_cursor(_pos: (usize, usize)) {}

pub fn set_colour(_col: serial::Colour) {}

fn putc_on_tty(tty: &mut Shell, c: char) {
    if tty.col_index >= CONSOLE_WIDTH {
        new_line(tty)
    } else if c == '\n' {
        let row = tty.rows % LOG_LINES;
        let col = tty.col_index % CONSOLE_WIDTH;
        tty.log[row][col] = c;
        new_line(tty);
        return;
    }
    let row = tty.rows % LOG_LINES;
    let col = tty.col_index % CONSOLE_WIDTH;
    tty.log[row][col] = c;
    tty.col_index += 1;
}

fn new_line(tty: &mut Shell) {
    if tty.rows >= LOG_LINES {
        tty.start_row = (tty.start_row + 1) % LOG_LINES;
    }
    tty.col_index = 0;
    tty.rows += 1;
}

fn update_vga_buf(tty: &mut Shell) {
    vga::clear();
    let min = (LOG_LINES).min(tty.rows);
    for i in 0..min {
        let row_index = (i + tty.start_row) % LOG_LINES;
        let char_array = &tty.log[row_index];
        for c in char_array {
            match *c {
                '\0' => break,
                '\n' => {
                    vga::new_line();
                    break;
                }
                c => vga::putc(c),
            }
        }
    }
    vga::puts("> ");
}

#[inline]
fn send_to_serial(c: char) {
    match c {
        '\n' => {
            com::putb(c as u8);
            com::putb('\r' as u8);
        }
        c => com::putb(c as u8),
    }
}
