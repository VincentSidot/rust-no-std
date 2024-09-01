#![no_std]
#![no_main]
mod syscall;

#[panic_handler]
fn idiot_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

// Entry point
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // syscall::exit(main())

    let message = b"Hello, World!\n";
    syscall::puts(message.as_ptr(), message.len());

    syscall::exit(0)
}

// fn main() -> usize {
//     let message = b"Hello, World!\n";
//     let message_len = message.len();
//     syscall::puts(message.as_ptr(), message_len);

//     0
// }
