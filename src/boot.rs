mod amd64;

pub struct BootInfo {}

#[macro_export]
macro_rules! entry_point {
    ($path:path) => {
        /// Kernel entry point.
        #[export_name = "__rust_entry"]
        pub extern "C" fn __impl_start(boot_info: &'static mut kernel::boot::BootInfo) -> ! {
            // validate the signature of the program entry point
            let f: fn(&'static mut kernel::boot::BootInfo) -> ! = $path;
            f(boot_info)
        }
    };
}

pub use entry_point;
