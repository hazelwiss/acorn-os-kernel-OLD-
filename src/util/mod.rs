pub mod mem;

macro_rules! once {
    ($($tokens:tt)*) => {
        {
            static STATIC_ONCE: ::spin::Once = ::spin::Once::new();
            STATIC_ONCE.call_once(||{ $($tokens)* });
        }
    };
}

pub(crate) use once;
