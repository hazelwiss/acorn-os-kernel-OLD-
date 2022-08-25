pub mod amd64;

pub struct Desc {
    pub halt: fn(),
    pub irq_enable: fn(bool),
}
