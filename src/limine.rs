/// A framebuffer from Limine.
///
/// This structure uses **C data organization**, as it's bound from C.
#[repr(C)]
pub struct Framebuffer {
    pub address: *mut core::ffi::c_void,
    pub width: u64,
    pub height: u64,
    pub pitch: u64,
    pub bits_per_pixel: u16,
    pub memory_model: u8,
    pub red_mask_size: u8,
    pub red_mask_shift: u8,
    pub green_mask_size: u8,
    pub green_mask_shift: u8,
    pub blue_mask_size: u8,
    pub blue_mask_shift: u8,
    pub unused: [u8; 7],
    pub edid_size: u64,
    pub edid: *mut core::ffi::c_void,
}

/// Limine's response to a framebuffer request (see [Framebuffer], [FramebufferRequest]).
///
/// This structure uses **C data organization**, as it's bound from C.
#[repr(C)]
pub struct FramebufferResponse {
    pub revision: u64,
    pub framebuffer_count: u64,
    pub framebuffers: *mut *mut Framebuffer, // Mutt mutt
}

/// A request to Limine to get a framebuffer (see [Framebuffer], [FramebufferResponse]).
///
/// This structure uses **C data organization**, as it's bound from C.
#[repr(C)]
pub struct FramebufferRequest {
    pub id: [u64; 4],
    pub revision: u64,
    pub response: *mut FramebufferResponse,
}

// This is required for safe interthread sharing (the compiler gets upset if
// this trait isn't implemented)
unsafe impl Sync for FramebufferRequest {}

/// Set which base revision of Limine the kernel uses.
#[macro_export]
macro_rules! limine_base_revision {
    ($number:literal) => {
        static LIMINE_BASE_REVISION: [u64; 3] = [0xf9562b2d5c95a6c8, 0x6a7b384944536bdc, $number];
    };
}

#[macro_export]
macro_rules! limine_base_revision_supported {
    () => {
        LIMINE_BASE_REVISION[2] == 0
    };
}

/// Create a framebuffer request to Limine.
#[macro_export]
macro_rules! limine_framebuffer_request {
    () => {
        static FRAMEBUFFER_REQUEST: crate::limine::FramebufferRequest =
            crate::limine::FramebufferRequest {
                id: [
                    0xc7b1dd30df4c8b88,
                    0x0a82e883a194f07b,
                    0x9d5827dcd881dd75,
                    0xa3148604f6fab11b,
                ],
                revision: 0,
                response: core::ptr::null_mut(),
            };
    };
}

/// Place the start marker for the Limine requests section.
#[macro_export]
macro_rules! limine_req_start {
    () => {
        #[used]
        #[unsafe(link_section = ".limine_requests_start")]
        static LIMINE_REQUESTS_START_MARKER: [u64; 4] = [
            0xf6b8f4b39de7d1ae,
            0xfab91a6940fcb9cf,
            0x785c6ed015d3e316,
            0x181e920a7852b9d9,
        ];
    };
}

/// Place the end marker for the Limine requests section.
#[macro_export]
macro_rules! limine_req_end {
    () => {
        #[used]
        #[unsafe(link_section = ".limine_requests_end")]
        static LIMINE_REQUESTS_END_MARKER: [u64; 2] = [0xadc0e0531bb10d03, 0x9572709f31764c62];
    };
}
