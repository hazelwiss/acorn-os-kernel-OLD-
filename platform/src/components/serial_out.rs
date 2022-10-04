#[cfg(target_arch = "x86_64")]
mod amd64_ibmpc;

pub use crate::Requested;

proc_macros::component!(
    /// Serial out.
    SerialOut => hal::serial::ISerialOut impl {
        x86_64_ibmpc => {
            fn init(){
                amd64_ibmpc::init();
            }

            fn on_request() -> Requested<Self>{
                amd64_ibmpc::request()
            }
        }
    }
);
