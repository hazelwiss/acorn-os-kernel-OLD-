pub struct Colour {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

pub struct ISerialIn {
    pub getb: fn() -> u8,
}

pub struct ISerialOut {
    pub putb: fn(b: u8),
}

impl ISerialOut {
    #[inline]
    pub fn putb(&self, b: u8) {
        (self.putb)(b);
    }

    #[inline]
    pub fn write_c(&self, c: char) {
        self.putb(c as u8);
        if c == '\n' {
            self.putb('\r' as u8);
        }
    }
}

pub struct IConsole {
    pub putc: fn(c: char),
    pub puts: fn(s: &str),
    pub getc: fn() -> char,
    pub clear: fn(),
    pub put_cursor: fn(pos: (usize, usize)),
    pub set_colour: fn(col: Colour),
}
