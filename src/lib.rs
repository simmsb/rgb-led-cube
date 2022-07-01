#![no_main]
#![no_std]
#![feature(const_fn_floating_point_arithmetic)]
#![feature(const_float_bits_conv)]
#![feature(adt_const_params)]

pub mod dither;

#[cfg(feature = "debugger")]
use defmt_rtt as _;

#[cfg(feature = "debugger")]
use panic_probe as _;

// #[defmt::panic_handler]
// fn panic() -> ! {
//     cortex_m::asm::udf()
// }

// pub fn exit() -> ! {
//     loop {
//         cortex_m::asm::bkpt();
//     }
// }
