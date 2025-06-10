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
limine_base_revision!(3);

// Request a framebuffer from Limine
limine_framebuffer_request!();

// Define the start and end markers for the Limine requests section
limine_req_start!();
limine_req_end!();

// Handles panics
#[panic_handler]
fn panic(_panic_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

// Kernel entrypoint
#[unsafe(no_mangle)]
pub extern "C" fn ignis_entry() -> ! {
    // Halt the CPU if the base revision is not supported
    if limine_base_revision_supported!() == false {
        halt_cpu!()
    }

    // Ensure a framebuffer is available, else halt the CPU
    if FRAMEBUFFER_REQUEST.response.is_null() || FRAMEBUFFER_REQUEST.response.framebuffer_count < 1
    {
        halt_cpu!()
    }

    // Grab the first framebuffer
    let framebuffer: *mut limine::Framebuffer = FRAMEBUFFER_REQUEST.response.framebuffers[0];

    // Paint a line on the screen
    for i in 0..100 {
        let fb_ptr = framebuffer.address as *mut u32;
        unsafe {
            *fb_ptr.add(i * (framebuffer.pitch as usize / 4) + i) = 0x00ffffff;
        }
    }

    // Hang the CPU
    loop {}
}
