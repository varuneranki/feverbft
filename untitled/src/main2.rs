/*fn main() {
    println!("Hello, world!");
    let mut a = 2;
    a = a+2;
    println!("value: {} {}", a , a / 6);

    /*if a % 2 == 0 && a % 1 == a {
        println!("even");
    } else {
        println!("odd");
    }*/


}*/

/*use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// Define a message type
#[derive(Debug, Clone)]
enum Message {
    Attack,
    Retreat,
}

// Define a struct to represent a server
struct Server {
    id: usize,
    messages: Arc<Mutex<Vec<Message>>>,
}

impl Server {
    // Constructor for creating a new server
    fn new(id: usize) -> Server {
        Server {
            id,
            messages: Arc::new(Mutex::new(Vec::new())),
        }
    }

    // Method for sending a message to all other servers
    fn send_message(&self, message: Message, servers: &[Server]) {
        for server in servers {
            if server.id != self.id {
                let mut messages = server.messages.lock().unwrap();
                messages.push(message.clone());
            }
        }
    }

    // Method for receiving messages from other servers and performing consensus
    fn receive_messages(&self, servers: &[Server]) -> Message {
        // Collect messages from all other servers
        let mut received_messages = Vec::new();
        for server in servers {
            if server.id != self.id {
                let messages = server.messages.lock().unwrap();
                received_messages.extend(messages.iter().cloned());
            }
        }

        // Perform simple consensus
        let attack_count = received_messages.iter().filter(|&m| *m == Message::Attack).count();
        let retreat_count = received_messages.iter().filter(|&m| *m == Message::Retreat).count();

        if attack_count > retreat_count {
            Message::Attack
        } else {
            Message::Retreat
        }
    }
}

fn main() {
    // Define the number of servers
    let num_servers = 3;

    // Create servers
    let mut servers = Vec::new();
    for i in 0..num_servers {
        servers.push(Server::new(i));
    }

    // Simulate communication between servers
    for server in &servers {
        let servers_ref = servers.clone();
        let server_ref = server.clone();
        thread::spawn(move || {
            // Simulate sending messages
            for _ in 0..3 {
                server_ref.send_message(Message::Attack, &servers_ref);
                thread::sleep(Duration::from_secs(1));
                server_ref.send_message(Message::Retreat, &servers_ref);
                thread::sleep(Duration::from_secs(1));
            }
        });
    }

    // Simulate receiving and making decisions based on received messages
    for server in &servers {
        let servers_ref = servers.clone();
        let server_ref = server.clone();
        thread::spawn(move || {
            // Simulate receiving messages and making decisions
            for _ in 0..3 {
                let decision = server_ref.receive_messages(&servers_ref);
                println!("Server {} decision: {:?}", server_ref.id, decision);
                thread::sleep(Duration::from_secs(2));
            }
        });
    }

    // Keep the main thread alive
    thread::sleep(Duration::from_secs(10));
}*/

//mod time;

use rand::Rng;

#[derive(Debug)]
enum Message {
    Attack,
    Retreat,
}

struct Server {
    id: u32,
    peers: Vec<u32>,
    received_messages: Vec<Message>,
}

impl Server {
    fn new(id: u32, peers: Vec<u32>) -> Self {
        Server { id, peers, received_messages: Vec::new() }
    }

    fn send_message(&self, message: Message) {
        for peer in self.peers.iter() {
            // Replace this with your communication library
            println!("Sending {:?} to server {}", message, peer);
        }
    }

    fn handle_message(&mut self, message: Message) {
        self.received_messages.push(message);
    }

    fn perform_consensus(&mut self) -> Option<Message> {
        let mut counts = [0; 2]; // Attack and Retreat counts
        for message in self.received_messages.iter() {
            counts[message as usize] += 1;
        }

        if counts[0] > counts[1] {
            Some(Message::Attack)
        } else if counts[1] > counts[0] {
            Some(Message::Retreat)
        } else {
            None // No consensus yet
        }
    }

    fn run(&mut self) {
        let mut rng = rand::thread_rng();
        let initial_message = if rng.gen_bool(0.5) {
            Message::Attack
        } else {
            Message::Retreat
        };

        self.send_message(initial_message);

        loop {
            let messages_from_peers = // Receive messages from peers
                // Replace this with your communication library
                vec![Message::Attack]; // Example message

            for message in messages_from_peers.iter() {
                self.handle_message(*message);
            }

            let consensus = self.perform_consensus();

            if let Some(decision) = consensus {
                println!("Server {} reached consensus: {:?}", self.id, decision);
                break;
            }

            let next_message = if rng.gen_bool(0.5) {
                Message::Attack
            } else {
                Message::Retreat
            };

            self.send_message(next_message);
        }
    }
}

fn main() {
    let server1 = Server::new(1, vec![2, 3]);
    let server2 = Server::new(2, vec![1, 3]);
    let server3 = Server::new(3, vec![1, 2]);

    server1.run();
    server2.run();
    server3.run();
}

