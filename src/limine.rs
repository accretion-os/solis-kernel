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

/// Mark an item as a request to Limine.
#[macro_export]
macro_rules! limine_request {
    ($item:item) => {
        #[used]
        #[unsafe(link_section = ".limine_requests")]
        $item
    };
}

/// Mark an item to be the start marker for the Limine requests section.
#[macro_export]
macro_rules! limine_req_start {
    ($item:item) => {
        #[used]
        #[unsafe(link_section = ".limine_requests_start")]
        $item
    };
}

/// Mark an item to be the end marker for the Limine requests section.
#[macro_export]
macro_rules! limine_req_end {
    ($item:item) => {
        #[used]
        #[unsafe(link_section = ".limine_requests_end")]
        $item
    };
}
