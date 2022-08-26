#![no_std]
#![allow(dead_code)]

pub mod math;
pub mod mem;

#[macro_export]
macro_rules! once {
    ($($tokens:tt)*) => {
        {
            static STATIC_ONCE: ::spin::Once = ::spin::Once::new();
            STATIC_ONCE.call_once(||{ $($tokens)* });
        }
    };
}
