use crate::{kutil::mem, once};
use spin::Mutex;

const VGA_BASE_ADR: usize = 0xB8000;
pub const VGA_WIDTH: usize = 80;
pub const VGA_HEIGHT: usize = 25;

pub enum VGAMode {
    Text,
}

struct VGAState {
    pos_x: usize,
    pos_y: usize,
    mode: VGAMode,
}

static VGA_STATE: Mutex<VGAState> = Mutex::new(VGAState {
    pos_x: 0,
    pos_y: 0,
    mode: VGAMode::Text,
});

impl VGAState {
    fn change_mode(&mut self, mode: VGAMode) {
        self.mode = mode;
    }

    fn putc(&mut self, c: char) {
        if self.pos_x >= VGA_WIDTH {
            self.pos_x = 0;
            self.pos_y = (self.pos_y + 1) % VGA_WIDTH;
        }
        let adr = VGA_BASE_ADR + self.pos_x * 2 + self.pos_y * 2 * VGA_WIDTH;
        unsafe { mem::write(adr, [c as u8, 9]) };
        self.pos_x += 1;
    }

    fn puts(&mut self, s: &str) {
        for c in s.chars() {
            self.putc(c);
        }
    }

    fn clear(&mut self) {
        let ptr = VGA_BASE_ADR as *mut u8;
        unsafe {
            ptr.write_bytes(0, VGA_HEIGHT * VGA_WIDTH * 2);
        }
        self.pos_x = 0;
        self.pos_y = 0;
    }

    fn get_w(&self) -> usize {
        VGA_WIDTH
    }

    fn get_h(&self) -> usize {
        VGA_HEIGHT
    }

    fn new_line(&mut self) {
        self.pos_y = (self.pos_y + 1) % VGA_HEIGHT;
        self.pos_x = 0;
    }
}

pub fn init() {
    once!();
}

pub fn change_mode(mode: VGAMode) {
    VGA_STATE.lock().mode = mode;
}

pub fn putc(c: char) {
    VGA_STATE.lock().putc(c);
}

pub fn puts(s: &str) {
    VGA_STATE.lock().puts(s);
}

pub fn clear() {
    VGA_STATE.lock().clear();
}

pub fn get_h() -> usize {
    VGA_HEIGHT
}

pub fn get_w() -> usize {
    VGA_WIDTH
}

pub fn new_line() {
    VGA_STATE.lock().new_line();
}
