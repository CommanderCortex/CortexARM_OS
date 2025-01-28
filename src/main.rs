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
const SERIAL_ADDR: *mut u8 = 0x0900_0000 as *mut u8;

global_asm!(include_str!("boot.s"), sym STACK, const STACK_SIZE);

//
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {	
	loop{}
}

#[no_mangle]
fn main() {
	let mut s = SerialDriver::new(SERIAL_ADDR);
	s.serial_put_string("Hello, World!\n");
	panic!();
	//loop{}
}	
