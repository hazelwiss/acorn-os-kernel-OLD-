//! Framebuffer interface

pub struct IDesc {
    pub init: fn(),
    pub putc: fn(char),
    pub puts: fs(&str),
}
