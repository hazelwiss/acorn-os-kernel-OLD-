//! All requestable components.

mod serial_out;
mod shell;

use crate::{Interface, NativeArch, NativeCS};

macro_rules! components {
    ($($c:ty;)*) => {
        $(
            pub use $c;
        )*
        pub(crate) fn init(){
            $(
                <$c as Interface<NativeArch, NativeCS>>::init();
            )*
        }
    };
}

components! {
    serial_out::SerialOut;
    shell::Shell;
}
