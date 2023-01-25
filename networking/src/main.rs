pub struct Player {
    pub name: String,
    pub ip: IpAddr,
    pub port: u16,
}

pub struct SubNet {
    pub hosts: Vec<String>, // ip addrs or urls of peers
    pub ports: Vec<u16>, // ports of peers
    pub n_players: u16, // number of players in the subnet
    pub port_base: u16, // base port for the subnet
    pub player_number: u16, // player number for this player
}

impl SubNet {
    fn init(&mut self, n_players: u16, port_base: u16) {
        self.n_players = n_players;
        self.port_base = port_base;
        self.player_number = 0;
        self.hosts = Vec::new();
        self.ports = Vec::new();
    }

}

fn main() {
    println!("Hello, world!");
}
