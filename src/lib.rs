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
use libc::{c_char, printf};

use core::offload::offload_kernel;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[cfg(target_os = "linux")]
#[unsafe(no_mangle)]
#[inline(never)]
fn main() {
    let mut x = [0.0f64];
    core::intrinsics::offload::<_, (&mut [f64],), ()>(foo, [1, 1, 1], [256, 1, 1], (&mut x,));
    unsafe {
        printf(c"debug: %f\n".as_ptr(), x[0]);
    }
}

#[offload_kernel]
fn foo(x: &mut [f64]) {
    (*x)[0] = 21.0;
}
