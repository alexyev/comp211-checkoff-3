struct Buffer {
    name: String,
}

impl Drop for Buffer {
    fn drop(&mut self) {
        println!("Releasing {}", self.name);
    }
}

fn main() {
    let a = Buffer { name: String::from("a") };
    println!("Created {}", a.name);

    {
        let b = Buffer { name: String::from("b") };
        println!("Created {}", b.name);
    } 

    println!("End of main");
}