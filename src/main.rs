#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::arch::global_asm;
use crate::serialdriver::*;

mod serialdriver;

#[no_mangle]
#[link_section = ".stack"]
static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

const STACK_SIZE: usize = 1024;

global_asm!(include_str!("boot.s"), sym STACK, const STACK_SIZE);

//
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {	
	serial_put_string("Kernel Panic: Halt!\n");
	loop{}
}

#[no_mangle]
fn main() {
	serial_put_string("Hello, world!\n");
	panic!();
	
	loop{}
}	
