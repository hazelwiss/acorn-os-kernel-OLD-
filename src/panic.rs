#[panic_handler]
unsafe fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    let location = info.location().unwrap_unchecked();
    let msg = info.message().unwrap_unchecked();
    log!("[PANIC] {msg}\n");
    log!("{}", location);
    loop {}
}
