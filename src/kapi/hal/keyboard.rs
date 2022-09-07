pub struct IDesc {
    pub init: fn(),
    pub putb: fn(u8),
    pub getb: fn() -> Option<u8>,
    pub getc: fn() -> Option<char>,
}
