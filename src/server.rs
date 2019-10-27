use crate::io::IO;

pub struct Server {
    io: IO
}

impl Server {
    pub fn new(io: IO) -> Self {
        Self { io }
    }

    pub fn run(&self) {
        self.io.write(String::from("Hello, World!"));
    }
}
