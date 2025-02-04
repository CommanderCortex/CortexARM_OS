use core::fmt::{Result, Write};

// Module for serial driver
pub struct SerialDriver{ // Serial driver struct
	uart_addr: *mut u8 // Pointer to UART address
}
impl SerialDriver{ // Serial driver implementation
	pub fn new(uart_addr: *mut u8) -> Self{ // Constructor
		SerialDriver{	// Return serial driver struct
			uart_addr: uart_addr, // Set UART address
		}
	}
	pub fn serial_put_string(&mut self, s: &str) { // Function to put string
		for c in s.chars() {	// Iterate over characters in string
			self.serial_put_char(c); // Put characters in string
		}
	}
	
	pub fn serial_put_char(&mut self, c: char) { // Function to put character
		unsafe { // Unsafe block
			self.uart_addr.write_volatile(c as u8); // Write character to UART address
 		}
	}
}

impl Write for SerialDriver{
	fn write_str(&mut self, s: &str) -> Result{
		self.serial_put_string(s);
		Ok(())
	}
}