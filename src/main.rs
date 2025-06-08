// Disable the standard library and main function entrypoint
#![no_std]
#![no_main]

/// Kernel utility functions.
pub mod kernel;

/// Bindings from C for the Limine bootloader.
pub mod limine;

/// The four critical memory functions `memcpy`, `memset`, `memmove`, and `memcmp`.
pub mod memory;

// Set which base revision of Limine the kernel uses, currently 3
limine_request! { static LIMINE_BASE_REVISION: [u64; 3] = [0xf9562b2d5c95a6c8, 0x6a7b384944536bdc, 3]; }

// Request a framebuffer from Limine.
limine_request! {
    static FRAMEBUFFER_REQUEST: limine::FramebufferRequest = limine::FramebufferRequest {
        id: [
            0xc7b1dd30df4c8b88,
            0x0a82e883a194f07b,
            0x9d5827dcd881dd75,
            0xa3148604f6fab11b,
        ],
        revision: 0,
        response: core::ptr::null_mut(),
    };
}

// Define the start and end markers for the Limine requests section
limine_req_start! { static LIMINE_REQUESTS_START_MARKER: [u64; 4] = [
    0xf6b8f4b39de7d1ae,
    0xfab91a6940fcb9cf,
    0x785c6ed015d3e316,
    0x181e920a7852b9d9,
]; }

limine_req_end! {
    static LIMINE_REQUESTS_END_MARKER: [u64; 2] = [0xadc0e0531bb10d03, 0x9572709f31764c62];
}

// Handles panics
#[panic_handler]
fn panic(_panic_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

// Kernel entrypoint
#[unsafe(no_mangle)]
pub extern "C" fn solis_entry() -> ! {
    loop {}
}
