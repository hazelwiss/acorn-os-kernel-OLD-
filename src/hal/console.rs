use crate::{arch::get_arch_interfaces, once};

pub struct IDesc {
    pub init: fn(),
    pub putc: fn(char),
    pub puts: fn(&str),
    pub get_w: fn() -> usize,
    pub get_h: fn() -> usize,
    pub new_line: fn(),
    pub clear: fn(),
}

static ICONSOLE: IDesc = get_arch_interfaces().console;

pub fn init() {
    once!(
        (ICONSOLE.init)();
    )
}

pub fn putc(c: char) {
    (ICONSOLE.putc)(c);
}

pub fn puts(s: &str) {
    (ICONSOLE.puts)(s);
}

pub fn get_w() -> usize {
    (ICONSOLE.get_w)()
}

pub fn get_h() -> usize {
    (ICONSOLE.get_h)()
}

pub fn new_line() {
    (ICONSOLE.new_line)();
}

pub fn clear() {
    (ICONSOLE.clear)();
}
