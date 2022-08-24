use spin::{Mutex, Once};

use crate::util::mem;

const VGA_BASE_ADR: usize = 0xB8000;
pub const VGA_WIDTH: usize = 80;
pub const VGA_HEIGHT: usize = 25;

static VGA_INIT_ONCE: Once = Once::new();

pub fn init() {
    VGA_INIT_ONCE.call_once(|| {});
}

pub fn change_mode(mode: VGAMode) {
    VGA_STATE.lock().mode = mode;
}

pub fn putc(c: char) {
    let mut state = VGA_STATE.lock();
    putc_onto_vga(&mut state, c);
}

pub fn puts(s: &str) {
    let mut state = VGA_STATE.lock();
    for c in s.chars() {
        putc_onto_vga(&mut state, c);
    }
}

pub fn set_pos(pos: (usize, usize)) {
    let mut state = VGA_STATE.lock();
    state.pos_x = pos.0 % VGA_WIDTH;
    state.pos_y = pos.1 & VGA_HEIGHT;
}

pub fn new_line() {
    let mut state = VGA_STATE.lock();
    state.pos_y = (state.pos_y + 1) % VGA_HEIGHT;
    state.pos_x = 0;
}

pub fn clear() {
    let mut state = VGA_STATE.lock();
    let ptr = VGA_BASE_ADR as *mut u8;
    unsafe {
        ptr.write_bytes(0, VGA_HEIGHT * VGA_WIDTH * 2);
    }
    state.pos_x = 0;
    state.pos_y = 0;
}

fn putc_onto_vga(state: &mut VGAState, c: char) {
    if state.pos_x >= VGA_WIDTH {
        state.pos_x = 0;
        state.pos_y = (state.pos_y + 1) % VGA_WIDTH;
    }
    let adr = VGA_BASE_ADR + state.pos_x * 2 + state.pos_y * 2 * VGA_WIDTH;
    unsafe { mem::write(adr, [c as u8, 9]) };
    state.pos_x += 1;
}

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
