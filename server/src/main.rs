fn main() {
    let ip = "192.168.0.1:8080";
    let server = Server::new(&ip);
    server.run();
}

struct Server<'a>{
    addr: &'a str,
}

impl<'a> Server<'a>{
    fn new(addr: &'a str) -> Self{
        Self { addr }
    }

    fn run(self){
        println!("Listening on {} ", self.addr);
    }
}