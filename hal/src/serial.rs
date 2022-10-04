//! serial

use proc_macros::interface;

interface! {
    /// Serial interface.
    pub struct ISerialOut{
        /// Outputs a byte to the serial interface.
        putb: fn(b: u8),
        /// Outputs a stream of bytes to the serial interface.
        putv: fn(b: &[u8]),
    }
}
