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

#[inline]
pub fn delay(delay: u32) {
    let mut delay: u32 = core::hint::black_box(delay);
    while delay > 0 {
        delay -= 1;
    }
}
