pub fn serial_put_string(s: &str) {
	for c in s.chars() {
		serial_put_char(c);
	}
}

pub fn serial_put_char(c: char) {
	let uart_addr = 0x0900_0000 as *mut u8; 
	unsafe {
		uart_addr.write_volatile(c as u8);
	}
}