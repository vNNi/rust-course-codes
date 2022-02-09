use std::io::Read;
use std::net::TcpListener;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self) {
        println!("Listening on {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut steam, _)) => {
                    let mut buffer = [0; 1024]; // this iniate the array with 1024 a chunk of memory allocated, using value 0 as bit (flipping the bits to zero in all the memory space)
                    match steam.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer))
                        }
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }
                    println!("Ok");
                }
                Err(e) => println!("Failed to estabilish a connection: {}", e),
            }
        }
    }
}
