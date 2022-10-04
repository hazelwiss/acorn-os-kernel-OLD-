pub use crate::Requested;

pub struct IShell {}

proc_macros::component! {
    /// Shell.
    Shell => IShell impl {
        x86_64_ibmpc => {
            fn init(){}

            fn on_request() -> Requested<Self>{
                todo!()
            }
        }
    }
}
