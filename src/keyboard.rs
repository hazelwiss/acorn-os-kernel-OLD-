/// Takes in a character and gives the correct Key enum variation.
#[macro_export]
macro_rules! key {
    (0) => {
        ::klib::drivers::ps2::Key::Num0
    };
    (1) => {
        ::klib::drivers::ps2::Key::Num1
    };
    (2) => {
        ::klib::drivers::ps2::Key::Num2
    };
    (3) => {
        ::klib::drivers::ps2::Key::Num3
    };
    (4) => {
        ::klib::drivers::ps2::Key::Num4
    };
    (5) => {
        ::klib::drivers::ps2::Key::Num5
    };
    (6) => {
        ::klib::drivers::ps2::Key::Num6
    };
    (7) => {
        ::klib::drivers::ps2::Key::Num7
    };
    (8) => {
        ::klib::drivers::ps2::Key::Num8
    };
    (9) => {
        ::klib::drivers::ps2::Key::Num9
    };
    (a) => {
        ::klib::drivers::ps2::Key::A
    };
    (b) => {
        ::klib::drivers::ps2::Key::B
    };
    (c) => {
        ::klib::drivers::ps2::Key::C
    };
    (d) => {
        ::klib::drivers::ps2::Key::D
    };
    (e) => {
        ::klib::drivers::ps2::Key::E
    };
    (f) => {
        ::klib::drivers::ps2::Key::F
    };
    (g) => {
        ::klib::drivers::ps2::Key::G
    };
    (h) => {
        ::klib::drivers::ps2::Key::H
    };
    (i) => {
        ::klib::drivers::ps2::Key::I
    };
    (j) => {
        ::klib::drivers::ps2::Key::J
    };
    (k) => {
        ::klib::drivers::ps2::Key::K
    };
    (l) => {
        ::klib::drivers::ps2::Key::L
    };
    (m) => {
        ::klib::drivers::ps2::Key::M
    };
    (n) => {
        ::klib::drivers::ps2::Key::N
    };
    (o) => {
        ::klib::drivers::ps2::Key::O
    };
    (p) => {
        ::klib::drivers::ps2::Key::P
    };
    (q) => {
        ::klib::drivers::ps2::Key::Q
    };
    (r) => {
        ::klib::drivers::ps2::Key::R
    };
    (s) => {
        ::klib::drivers::ps2::Key::S
    };
    (t) => {
        ::klib::drivers::ps2::Key::T
    };
    (u) => {
        ::klib::drivers::ps2::Key::U
    };
    (v) => {
        ::klib::drivers::ps2::Key::V
    };
    (w) => {
        ::klib::drivers::ps2::Key::W
    };
    (x) => {
        ::klib::drivers::ps2::Key::X
    };
    (y) => {
        ::klib::drivers::ps2::Key::Y
    };
    (z) => {
        ::klib::drivers::ps2::Key::Z
    };
    (=) => {
        ::klib::drivers::ps2::Key::DoubleBar
    };
    (-) => {
        ::klib::drivers::ps2::Key::Hyphen
    };
    (:) => {
        ::klib::drivers::ps2::Key::Colon
    };
    (,) => {
        ::klib::drivers::ps2::Key::Comma
    };
    (.) => {
        ::klib::drivers::ps2::Key::Dot
    };
    (/) => {
        ::klib::drivers::ps2::Key::FSlash
    };
    ( ) => {
        ::klib::drivers::ps2::Key::Space
    };
    (*) => {
        ::klib::drivers::ps2::Key::Asterix
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
