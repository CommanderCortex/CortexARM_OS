#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::arch::global_asm;

#[no_mangle]
#[link_section = ".stack"]
static mut STACK: [u8; 1024] = [0; 1024];

const STACK_SIZE: usize = 1024;

global_asm!(include_str!("boot.s"), sym STACK, const STACK_SIZE);

//
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {	
	loop{}
}

fn serial_put_char(c: char) {
	let uart_addr = 0x0900_0000 as *mut u8;
	unsafe {
		*uart_addr = c as u8;
	}
}

#[no_mangle]
fn main() {
	serial_put_char('H');
	loop{}
}	
