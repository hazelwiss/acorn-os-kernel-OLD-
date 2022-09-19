use crate::{hal, once};
use spin::Mutex;

struct State {
    pos: Pos,
}

impl State {
    fn init(&self) {
        once! {}
    }

    fn putc(&self, c: char) {
        match c {
            '\n' => hal::console::new_line(),
            c => hal::console::putc(c),
        };
    }

    fn puts(&self, s: &str) {
        hal::console::puts(s);
    }

    fn clear(&self) {
        hal::console::clear();
    }

    fn new_line(&self) {
        hal::console::new_line();
    }

    fn get_dimensions(&self) -> Dimensions {
        Dimensions {
            width: hal::console::get_w(),
            height: hal::console::get_h(),
        }
    }
}

static CONSOLE: Mutex<State> = Mutex::new(State {
    pos: Pos { col: 0, row: 0 },
});

#[derive(Clone, Copy)]
pub struct Pos {
    pub col: u32,
    pub row: u32,
}

#[derive(Clone, Copy)]
pub struct Dimensions {
    pub width: usize,
    pub height: usize,
}

pub fn init() {
    CONSOLE.lock().init()
}

pub fn putc(c: char) {
    CONSOLE.lock().putc(c)
}

pub fn puts(s: &str) {
    CONSOLE.lock().puts(s)
}

pub fn set_pos(pos: Pos) {
    CONSOLE.lock().pos = pos;
}

pub fn newline() {
    CONSOLE.lock().new_line();
}

pub fn get_pos() -> Pos {
    CONSOLE.lock().pos
}

pub fn clear() {
    CONSOLE.lock().clear();
}

pub fn get_dimensions() -> Dimensions {
    CONSOLE.lock().get_dimensions()
}
