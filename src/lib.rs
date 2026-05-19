#![allow(internal_features)]
#![feature(gpu_offload)]
#![feature(abi_gpu_kernel)]
#![feature(core_intrinsics)]
#![no_std]

#[cfg(target_os = "linux")]
extern crate libc;
#[cfg(target_os = "linux")]
use core::mem;
#[cfg(target_os = "linux")]
use libc::c_char;

use core::offload::offload_kernel;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[cfg(target_os = "linux")]
#[unsafe(no_mangle)]
#[inline(never)]
fn main() {
    unsafe {
        let x: *mut [f64; 256] =
            libc::calloc(256, (mem::size_of::<f64>()) as libc::size_t) as *mut [f64; 256];
        core::intrinsics::offload::<_, (*mut [f64; 256],), ()>(foo, [1, 1, 1], [256, 1, 1], (x,));
    }
}

#[offload_kernel]
fn foo(x: *mut [f64; 256]) {
    unsafe { (*x)[0] = 21.0 };
}
