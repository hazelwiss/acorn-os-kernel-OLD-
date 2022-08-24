#[panic_handler]
unsafe fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    let msg = info.message().unwrap();
    log!("[PANIC] {msg}");
    loop {}
}
