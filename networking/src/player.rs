pub struct Player {
    pub id: u16,
    pub ip: IpAddr,
    pub port: u16,
    pub subnet: SubNet
}

impl Player {
    fn init(&mut self, id: u16, ip: IpAddr, port: u16) {
        self.id = id;
        self.ip = ip;
        self.port = port;
    }

    fn send_int(&mut self, to: u16, a: i32) { }

    fn receive_int(&mut self, from: u16) -> i32 { 0 }

    fn send_to_player(&mut self, to: u16, msg: vec<u8>) { }

    fn receive_from_player(&mut self, from: u16) -> vec<u8> { vec![] }

    fn exchange(&mut self, to: u16, msg: vec<u8>) -> vec<u8> { vec![] }

    fn pass_around(&mut self, send: vec<u8>, receive: vec<u8>) { }

    fn send_all(&mut self, msg: vec<u8>) { }

    fn receive_all(&mut self, msg: vec<u8>) { }

    
}

