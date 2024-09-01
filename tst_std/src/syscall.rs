use syscalls_macro::__syscall;
__syscall!();

pub fn puts(message: *const u8, message_len: usize) {
    unsafe {
        syscall!(1, 1, message as isize, message_len as isize);
    }
}

pub fn exit(code: usize) -> ! {
    unsafe {
        syscall!(60, code as isize);
    }
    loop {}
}

const PROT_READ: usize = 0x1;
const PROT_WRITE: usize = 0x2;

const MAP_PRIVATE: usize = 0x2;
