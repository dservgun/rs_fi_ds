use std::io::{self, Read, Write};

const CHUNK_SIZE : usize = 16 * 1024;

pub mod data_io {
	fn pipeviewer() {
		let mut buffer = [0, CHUNK_SIZE];
		let num_read = io::stdin().read(&mut buffer).unwrap();
		eprintln!("num_read : {}", num_read);
		io::stdout().write_all(&buffer[..num_read]).unwrap();
	}
}