// Disable the standard library and main function entrypoint
#![no_std]
#![no_main]

/// Kernel utility functions.
pub mod kernel;

/// Bindings from C for the Limine bootloader.
pub mod limine;

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
    halt_cpu!();
}

// Kernel entrypoint
#[unsafe(no_mangle)]
pub extern "C" fn ignis_entry() -> ! {
    if !limine_base_revision_supported!() {
        halt_cpu!();
    }

    unsafe {
        let response = FRAMEBUFFER_REQUEST.response;
        if response.is_null() || (*response).framebuffer_count < 1 {
            nested_halt_cpu!();
        }

        let fb = (*(*response).framebuffers).add(0);
        let fb_ptr = (*fb).address as *mut u32;

        for i in 0..100 {
            *fb_ptr.add(i * ((*fb).pitch as usize / 4) + i) = 0x00ffffff;
        }
    }

    halt_cpu!();
}
