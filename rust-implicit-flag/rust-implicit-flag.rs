struct Buffer {
    name: String,
}

impl Drop for Buffer {
    fn drop(&mut self) {
        println!("    Releasing {}", self.name);
    }
}

// This function takes ownership. It releases buf at its closing brace.
fn take_ownership(buf: Buffer) {
    println!("    take_ownership now owns {}", buf.name);
} // buf released here

fn run(move_it: bool) {
    let buf = Buffer { name: String::from("data") };

    if move_it {
        take_ownership(buf);   // ownership moves ONLY when move_it is true
    }

    println!("    End of run");
} // Rust must release buf here, but ONLY if it did not move

fn main() {
    println!("Call 1: move_it = true");
    run(true);

    println!("Call 2: move_it = false");
    run(false);
}