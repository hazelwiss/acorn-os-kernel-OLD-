use spin::Mutex;

use crate::{
    hal::keyboard,
    keyboard::{key, Key, KeyState},
    once,
};

pub enum KbdLayout {
    QwertyUS,
    QwertySE,
}

pub struct State {
    layout: KbdLayout,
}

macro_rules! pk {
    ($k:expr) => {{
        KeyState::Pressed($k)
    }};
}

macro_rules! kbd {
    ($match:ident; $($num:literal => $e:expr),* $(,)?) => {
        match $match{
            $($num => { $e })*
            _ => return None,
        }
    };
}

macro_rules! kbd_us {
    ($match:ident) => {
        kbd!(
            $match;
            0x01 => pk!(Key::Escape),
            0x02 => pk!(key!(1)),
            0x03 => pk!(key!(2)),
            0x04 => pk!(key!(3)),
            0x05 => pk!(key!(4)),
            0x06 => pk!(key!(5)),
            0x07 => pk!(key!(6)),
            0x08 => pk!(key!(7)),
            0x09 => pk!(key!(8)),
            0x0A => pk!(key!(9)),
            0x0B => pk!(key!(0)),
            0x0C => pk!(key!(-)),
            0x0D => pk!(key!(=)),
            0x0E => pk!(Key::Backspace),
            0x0F => pk!(Key::Tab),
            0x10 => pk!(key!(q)),
            0x11 => pk!(key!(w)),
            0x12 => pk!(key!(e)),
            0x13 => pk!(key!(r)),
            0x14 => pk!(key!(t)),
            0x15 => pk!(key!(y)),
            0x16 => pk!(key!(u)),
            0x17 => pk!(key!(i)),
            0x18 => pk!(key!(o)),
            0x19 => pk!(key!(p)),
            0x1A => pk!(Key::SqrBracketL),
            0x1B => pk!(Key::SqrBracketR),
            0x1C => pk!(Key::Enter),
            0x1D => pk!(Key::LCtrl),
            0x1E => pk!(key!(a)),
            0x1F => pk!(key!(s)),
            0x20 => pk!(key!(d)),
            0x21 => pk!(key!(f)),
            0x22 => pk!(key!(g)),
            0x23 => pk!(key!(h)),
            0x24 => pk!(key!(j)),
            0x25 => pk!(key!(k)),
            0x26 => pk!(key!(l)),
            0x27 => pk!(key!(:)),
            0x28 => pk!(Key::SQuote),
            0x29 => pk!(Key::BackTick),
            0x2A => pk!(Key::LShift),
            0x2B => pk!(Key::BSlash),
            0x2C => pk!(key!(z)),
            0x2D => pk!(key!(x)),
            0x2E => pk!(key!(c)),
            0x2F => pk!(key!(v)),
            0x30 => pk!(key!(b)),
            0x31 => pk!(key!(n)),
            0x32 => pk!(key!(m)),
            0x33 => pk!(key!(,)),
            0x34 => pk!(key!(.)),
            0x35 => pk!(key!(/)),
            0x36 => pk!(Key::RShift),
            0x37 => pk!(key!(*)),
            0x38 => pk!(Key::LAlt),
            0x39 => pk!(key!( )),
        )
    };
}

macro_rules! kbd_se {
    ($match:ident) => {
        kbd!($match;)
    };
}

impl State {
    fn init(&self) {
        once! {}
    }

    fn getk(&self) -> Option<KeyState> {
        if let Some(b) = keyboard::getb() {
            Some(match self.layout {
                KbdLayout::QwertyUS => kbd_us!(b),
                KbdLayout::QwertySE => kbd_se!(b),
            })
        } else {
            None
        }
    }

    fn getc_pressed(&self) -> Option<char> {
        todo!()
    }

    fn get_release(&self) -> Option<char> {
        todo!()
    }
}

static PS2: Mutex<State> = Mutex::new(State {
    layout: KbdLayout::QwertyUS,
});

pub fn init() {
    PS2.lock().init()
}

pub fn getk() -> Option<KeyState> {
    PS2.lock().getk()
}
