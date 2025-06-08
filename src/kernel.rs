/// Halt the CPU operation.
///
/// **This is a kernel-level macro.** Be careful!
///
/// # Safety
/// This macro exhibits the following behavior that is unsafe:
/// - Uses raw assembly
///     - CPU halting
///
/// ## Possible faults
/// - *Malicious use*: A program or bad actor that is granted access to this
/// macro can use it to their will to stop the OS.
///
/// To ensure safety, check the following:
/// - The macro is exposed only to kernel-level functionality/programs.
/// - The macro is kept overall secure.
#[macro_export]
macro_rules! halt_cpu {
    () => {
        unsafe {
            core::arch::asm!("hlt");
        }
    };
}
