// C foreign function interface for C data types
use core::ffi;

/// Copies arbitrary data. Use over `memmove` *only* if you know for sure that
/// `destination` and `source` memory regions don't overlap.
///
/// This function uses the **C calling convention**, as the linker doesn't
/// understand the Rust convention and expects the four core memory symbols
/// (like `memcpy`, this function) to use the C ABI.
///
/// # Parameters
/// | Solis memory API name | Traditional libc name | Description                                                   |
/// | :-------------------: | :-------------------: | :-----------------------------------------------------------: |
/// | `destination`         | `dest`                | A mutable pointer to the start of the memory to be copied to. |
/// | `source`              | `src`                 | A constant pointer to the source start address.               |
/// | `size`                | `n`                   | The size of the source data in bytes.                         |
///
/// # Returns
/// The destination address that was originally given (`*mut core::ffi::c_void`).
///
/// # Safety
/// This function exhibits the following behaviors that can be considered/is unsafe:
/// - Disables name mangling.
/// - Dereferences raw pointers.
///     - Reads and writes to arbitrary memory.
///
/// To ensure safety, check the following:
/// - The given destination address is valid (ergo non-null).
/// - The given source address is valid (ergo non-null).
/// - The source size doesn't exceed the valid memory.
/// - Only using the function if it's been confirmed the destination and source
/// memory regions **do not** overlap.
///
/// ## Possible faults
/// **This list is (likely) non-exhaustive. Always exercise caution with unsafe code.**
/// - *Null pointer dereferencing*: If `destination` is null, then undefined
/// behavior will occur.
/// - *Out-of-bounds memory access*: If the valid memory in bytes minus the
/// destination's length from 0x0 in bytes is less than or equal to 0, memory
/// corruption or a crash will occur.
/// - *Memory regional overlap*: If the destination and source's memory regions
/// overlap, then undefined behavior will occur.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn memcpy(
    destination: *mut ffi::c_void,
    source: *const ffi::c_void,
    size: usize,
) -> *mut ffi::c_void {
    // Cast the addresses into usable ones
    let address = destination as *mut u8;
    let buf_addr = source as *const u8;

    // Copy `size` bytes past `source` (including the byte on it)
    for index in 0..size {
        unsafe {
            *address.add(index) = *buf_addr.add(index);
        }
    }

    // Return the destination address
    destination
}

/// Fills a block of memory with the same byte value.
///
/// This function uses the **C calling convention**, as the linker doesn't
/// understand the Rust convention and expects the four core memory symbols
/// (like `memcpy`) to use the C ABI.
///
/// # Parameters
/// | Solis memory API name | Traditional libc name | Description                                                   |
/// | :-------------------: | :-------------------: | :-----------------------------------------------------------: |
/// | `destination`         | `s`                   | A mutable pointer to the start of the memory to be copied to. |
/// | `source_byte`         | `c`                   | An integer (which will be cast to the byte).                  |
/// | `byte_count`          | `n`                   | How many bytes to fill.                                       |
///
/// # Returns
/// The destination that was given. (`*mut core::ffi::c_void`)
///
/// # Safety
/// This function exhibits the following behaviors that can be considered/is unsafe:
/// - Disables name mangling.
/// - Dereferences raw pointers.
///     - Writes to arbitrary memory.
///
/// To ensure safety, check the following:
/// - The given offset is a valid address (ergo non-null).
/// - The source byte is a valid C integer.
/// - The amount of bytes to be filled doesn't exceed the valid memory.
///
/// ## Possible faults
/// **This list is (likely) non-exhaustive. Always exercise caution with unsafe code.**
/// - *Null pointer dereferencing*: If `offset` is null, then undefined
/// behavior will occur.
/// - *Out-of-bounds memory access*: If the valid memory in bytes minus the
/// offset's length from 0x0 in bytes is less than or equal to 0 or the amount
/// of bytes to be filled is greater than the amount of valid memory, memory
/// corruption or a crash will occur.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn memset(
    destination: *mut ffi::c_void,
    source_byte: ffi::c_int,
    byte_count: usize,
) -> *mut ffi::c_void {
    // Cast the destination and byte into a usable address or byte
    let start = destination as *mut u8;
    let byte = source_byte as u8;

    // Fill `byte_count` bytes of memory
    for index in 0..byte_count {
        unsafe {
            *start.add(index) = byte;
        }
    }

    // Return the destination
    destination
}

/// Copies arbitrary data. Use this function over `memcpy` if you're unsure as
/// to whether or not the `destination` and `source` memory regions overlap.
///
/// This function uses the **C calling convention**, as the linker doesn't
/// understand the Rust convention and expects the four core memory symbols
/// (like `memcpy`) to use the C ABI.
///
/// # Parameters
/// | Solis memory API name | Traditional libc name | Description                                                |
/// | :-------------------: | :-------------------: | :--------------------------------------------------------: |
/// | `destination`         | `dest`                | A mutable pointer to the start of the memory to be filled. |
/// | `source`              | `src`                 | The byte (a C integer, will be cast) to fill with.         |
/// | `source_size`         | `n`                   | How many bytes of memory to fill from the destination.     |
///
/// # Returns
/// The destination address that was originally given (`*mut core::ffi::c_void`).
///
/// # Safety
/// This function exhibits the following behaviors that can be considered/is unsafe:
/// - Disables name mangling.
/// - Dereferences raw pointers.
///     - Reads and writes to arbitrary memory.
///
/// To ensure safety, check the following:
/// - The given destination address is valid (ergo non-null).
/// - The given source address is valid (ergo non-null).
/// - The source size doesn't exceed the valid memory.
///
/// ## Possible faults
/// **This list is (likely) non-exhaustive. Always exercise caution with unsafe code.**
/// - *Null pointer dereferencing*: If `destination` is null, then undefined
/// behavior will occur.
/// - *Out-of-bounds memory access*: If the valid memory in bytes minus the
/// destination's length from 0x0 in bytes is less than or equal to 0, memory
/// corruption or a crash will occur.
pub extern "C" fn memmove(
    destination: *mut ffi::c_void,
    source: *const ffi::c_void,
    buffer_size: usize,
) -> *mut ffi::c_void {
    // Cast the addresses into usable ones
    let address = destination as *mut u8;
    let buf_addr = source as *const u8;

    // Normal copy if the source is in higher memory
    if (destination as usize) > (source as usize) {
        for index in 0..buffer_size {
            unsafe {
                *address.add(index) = *buf_addr.add(index);
            }
        }
    }
    // Reverse copy if the source is in lower memory
    else if (destination as usize) < (source as usize) {
        for index in (0..buffer_size).rev() {
            unsafe {
                *address.add(index) = *buf_addr.add(index);
            }
        }
    }

    // Return the destination
    destination
}

/// Compares two blocks of memory byte-by-byte.
///
/// This function uses the **C calling convention**, as the linker doesn't
/// understand the Rust convention and expects the four core memory symbols
/// (like `memcpy`) to use the C ABI.
///
/// # Parameters
/// | Solis memory API name | Traditional libc name | Description                                                                        |
/// | :-------------------: | :-------------------: | :--------------------------------------------------------------------------------: |
/// | `block1`              | `s1`                  | A constant pointer to the start of the first block to be compared.                 |
/// | `block2`              | `s2`                  | A constant pointer to the start of the second block to be compared.                |
/// | `bytes_past`          | `n`                   | How many bytes past the starts to compare. (Including the value on the addresses.) |
///
/// # Returns
/// A negative value if the first differing byte in `block1` is less than in `block2`.
/// Zero if the blocks are equal.
/// A positive value if the first differing byte in `block1` is greater than in `block2`.
///
/// # Safety
/// This function exhibits the following behaviors that can be considered/is unsafe:
/// - Disables name mangling.
/// - Dereferences raw pointers.
///     - Reads arbitrary memory.
///
/// To ensure safety, check the following:
/// - The given first block address is valid (ergo non-null).
/// - The given second block address is valid (ergo non-null).
/// - The amount of bytes to compare doesn't exceed the valid memory.
///
/// ## Possible faults
/// **This list is (likely) non-exhaustive. Always exercise caution with unsafe code.**
/// - *Null pointer dereferencing*: If one or both block addresses are null,
/// then undefined behavior will occur.
/// - *Out-of-bounds memory access*: If the valid memory in bytes minus the
/// amount of bytes to compare is less than or equal to 0, memory corruption or
/// a crash will occur.
#[unsafe(no_mangle)]
pub extern "C" fn memcmp(
    block1: *const ffi::c_void,
    block2: *const ffi::c_void,
    bytes_past: usize,
) -> ffi::c_int {
    let one = block1 as *const u8;
    let two = block2 as *const u8;

    for index in 0..bytes_past {
        unsafe {
            let comp1 = *one.add(index);
            let comp2 = *two.add(index);
        }

        if comp1 != comp2 {
            return if comp1 < comp2 { -1 } else { 1 };
        }
    }

    0
}
