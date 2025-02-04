#![no_std] //Leaves out all Standard Headers when compiling binary
#![no_main] // No Operating System or runtime to start our program at the Main Funtion

use core::panic::PanicInfo; // Builtin ARM Flag for Kernel Panics
use core::arch::global_asm; // Bultin ARM Instructions for the CPU Architecture
use crate::serialdriver::*; // Our implementation of a Serial Driver for displaying text to the console

mod serialdriver; //Module Implented
mod print; //Module Implented

#[no_mangle] // No Mangles the name of our Stack
#[link_section = ".stack"] // Links the Stack to the .stack section of the Binary
static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE]; // Creates a Stack of 1024 Bytes

const STACK_SIZE: usize = 1024; // 1024 Bytes for the Stack
const SERIAL_ADDR: *mut u8 = 0x0900_0000 as *mut u8; // Address of the Serial Port

global_asm!(include_str!("boot.s"), sym STACK, const STACK_SIZE); // Assembly Code for the Bootloader

//Kernel Panic Handler 
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {	
	loop{}
}

//Main Function
#[no_mangle]
fn main() {
	let mut s = SerialDriver::new(SERIAL_ADDR); //Creates a new Serial Driver
	s.serial_put_string("Serial: Hello, World!\n"); //Prints Hello, World! to the Console
	//panic!(); //Invokes our Kernel Panic
	cprint!("Print Function: Hello, World!\n"); //Prints Hello, World! to the Console
	//loop{}
}	
