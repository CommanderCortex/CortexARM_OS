pub struct SerialDriver{
	uart_addr: *mut u8
}
impl SerialDriver{
	pub fn new(uart_addr: *mut u8) -> Self{
		SerialDriver{
			uart_addr: uart_addr,
		}
	}
	pub fn serial_put_string(&mut self, s: &str) {
		for c in s.chars() {
			self.serial_put_char(c);
		}
	}
	
	pub fn serial_put_char(&mut self, c: char) {
		unsafe {
			self.uart_addr.write_volatile(c as u8);
		}
	}
}