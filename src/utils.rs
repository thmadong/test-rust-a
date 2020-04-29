pub struct Logger { }

impl Logger {
	pub fn debug(s: &str) {
        println!("[test-rust-a:debug]: {}", s);
    }

    pub fn error(s: &str) {
        println!("[test-rust-a:error]: {}", s);
    }
}