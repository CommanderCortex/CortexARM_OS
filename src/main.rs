#![no_std] //Leaves out all Standard Headers when compiling binary
#![no_main] // No Operating System or runtime to start our program at the Main Funtion

use core::panic::PanicInfo; // Builtin ARM Flag for Kernel Panics
use core::arch::global_asm; // Bultin ARM Instructions for the CPU Architecture
use crate::serialdriver::*; // Our implementation of a Serial Driver for displaying text to the console

mod serialdriver; //Module Implented

#[no_mangle] // No Mangles the name of our Stack
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
