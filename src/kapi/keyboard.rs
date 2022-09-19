/// Takes in a character and gives the correct Key enum variation.
#[macro_export]
macro_rules! key {
    (0) => {
        ::klib::kapi::drivers::ps2::Key::Num0
    };
    (1) => {
        ::klib::kapi::drivers::ps2::Key::Num1
    };
    (2) => {
        ::klib::kapi::drivers::ps2::Key::Num2
    };
    (3) => {
        ::klib::kapi::drivers::ps2::Key::Num3
    };
    (4) => {
        ::klib::kapi::drivers::ps2::Key::Num4
    };
    (5) => {
        ::klib::kapi::drivers::ps2::Key::Num5
    };
    (6) => {
        ::klib::kapi::drivers::ps2::Key::Num6
    };
    (7) => {
        ::klib::kapi::drivers::ps2::Key::Num7
    };
    (8) => {
        ::klib::kapi::drivers::ps2::Key::Num8
    };
    (9) => {
        ::klib::kapi::drivers::ps2::Key::Num9
    };
    (a) => {
        ::klib::kapi::drivers::ps2::Key::A
    };
    (b) => {
        ::klib::kapi::drivers::ps2::Key::B
    };
    (c) => {
        ::klib::kapi::drivers::ps2::Key::C
    };
    (d) => {
        ::klib::kapi::drivers::ps2::Key::D
    };
    (e) => {
        ::klib::kapi::drivers::ps2::Key::E
    };
    (f) => {
        ::klib::kapi::drivers::ps2::Key::F
    };
    (g) => {
        ::klib::kapi::drivers::ps2::Key::G
    };
    (h) => {
        ::klib::kapi::drivers::ps2::Key::H
    };
    (i) => {
        ::klib::kapi::drivers::ps2::Key::I
    };
    (j) => {
        ::klib::kapi::drivers::ps2::Key::J
    };
    (k) => {
        ::klib::kapi::drivers::ps2::Key::K
    };
    (l) => {
        ::klib::kapi::drivers::ps2::Key::L
    };
    (m) => {
        ::klib::kapi::drivers::ps2::Key::M
    };
    (n) => {
        ::klib::kapi::drivers::ps2::Key::N
    };
    (o) => {
        ::klib::kapi::drivers::ps2::Key::O
    };
    (p) => {
        ::klib::kapi::drivers::ps2::Key::P
    };
    (q) => {
        ::klib::kapi::drivers::ps2::Key::Q
    };
    (r) => {
        ::klib::kapi::drivers::ps2::Key::R
    };
    (s) => {
        ::klib::kapi::drivers::ps2::Key::S
    };
    (t) => {
        ::klib::kapi::drivers::ps2::Key::T
    };
    (u) => {
        ::klib::kapi::drivers::ps2::Key::U
    };
    (v) => {
        ::klib::kapi::drivers::ps2::Key::V
    };
    (w) => {
        ::klib::kapi::drivers::ps2::Key::W
    };
    (x) => {
        ::klib::kapi::drivers::ps2::Key::X
    };
    (y) => {
        ::klib::kapi::drivers::ps2::Key::Y
    };
    (z) => {
        ::klib::kapi::drivers::ps2::Key::Z
    };
    (=) => {
        ::klib::kapi::drivers::ps2::Key::DoubleBar
    };
    (-) => {
        ::klib::kapi::drivers::ps2::Key::Hyphen
    };
    (:) => {
        ::klib::kapi::drivers::ps2::Key::Colon
    };
    (,) => {
        ::klib::kapi::drivers::ps2::Key::Comma
    };
    (.) => {
        ::klib::kapi::drivers::ps2::Key::Dot
    };
    (/) => {
        ::klib::kapi::drivers::ps2::Key::FSlash
    };
    ( ) => {
        ::klib::kapi::drivers::ps2::Key::Space
    };
    (*) => {
        ::klib::kapi::drivers::ps2::Key::Asterix
    };
}

use core::convert::TryInto;

pub use key;

macro_rules! keys_def {
    ($($var0:ident,)*; $($var1:ident => $num:literal,)*) => {
        pub enum Key {
            $($var0,)*
            $($var1,)*
        }

        impl Key {
            pub fn as_char(&self) -> Option<char>{
                use Key::*;
                match self{
                    $($var1 => Some($num as char),)*
                    _ => None,
                }
            }
        }
    };
}

keys_def! {
    LAlt,
    LShift,
    RShift,
    LCtrl,
    RCtrl,;
    Escape => '\x1B',
    Backspace => '\x08',
    Tab => '\t',
    SqrBracketR => ']',
    SqrBracketL => '[',
    Enter => '\x0D',
    SQuote => '\'',
    DQuote => '\"',
    BackTick => '`',
    BSlash => '\\',
    Space => ' ',
    Comma => ',',
    Dot => '.',
    FSlash => '/',
    Colon => ':',
    Hyphen => '-',
    DoubleBar => '=',
    Asterix => '*',
    A => 'a',
    B => 'b',
    C => 'c',
    D => 'd',
    E => 'e',
    F => 'f',
    G => 'g',
    H => 'h',
    L => 'l',
    J => 'j',
    I => 'i',
    K => 'k',
    M => 'm',
    N => 'n',
    O => 'o',
    P => 'p',
    Q => 'q',
    R => 'r',
    S => 's',
    T => 't',
    U => 'u',
    V => 'v',
    W => 'w',
    X => 'x',
    Y => 'y',
    Z => 'z',
    Num0 => 'x',
    Num1 => '1',
    Num2 => '2',
    Num3 => '3',
    Num4 => '4',
    Num5 => '5',
    Num6 => '6',
    Num7 => '7',
    Num8 => '8',
    Num9 => '9',
}

impl TryInto<char> for Key {
    type Error = ();

    fn try_into(self) -> Result<char, Self::Error> {
        self.as_char().ok_or(())
    }
}

pub enum KeyState {
    Pressed(Key),
    Released(Key),
}
