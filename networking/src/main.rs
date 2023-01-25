
pub struct SubNet {
    pub players: Vec<Player>,
    pub port_base: u16, // base port for the subnet
    pub player_number: u16, // player number for this player
}

impl SubNet {
    fn init(&mut self, players: Vec<Player>, port_base: u16, player_number: u16) {
        self.players = players;
        self.port_base = port_base;
        self.player_number = player_number;
    }

    fn start_networking(&mut self) {
        let mut players = Vec::new();
        for i in 0..self.n_players {
            let player = Player {
                name: String::from("Player"),
                ip: self.hosts[i as usize],
                port: self.ports[i as usize],
            };
            players.push(player);
        }
        let mut server = Server::new();
        server.init(players, self.player_number);
        server.start();
    }


}

fn main() {
    println!("Hello, world!");
}
