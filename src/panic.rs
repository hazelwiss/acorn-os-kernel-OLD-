#[panic_handler]
#[no_mangle]
unsafe fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    let msg = unsafe { info.message().unwrap_unchecked() };
    let location = unsafe { info.location().unwrap_unchecked() };
    log!("[PANIC] {location} {msg}\n\r");
    loop {}
}
