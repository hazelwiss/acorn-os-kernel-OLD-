pub unsafe fn write<T: Sized>(adr: usize, val: T) {
    let ptr = adr as *mut T;
    ptr.write(val);
}

pub unsafe fn read<T: Sized>(adr: usize) -> T {
    let ptr = adr as *const T;
    ptr.read()
}
