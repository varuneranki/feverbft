use libp2p::{
    floodsub::{Floodsub, FloodsubEvent},
    gossipsub, mdns, noise, SwarmBuilder, tcp, NetworkBehaviour, NetworkBehaviourEventProcess,
    swarm::SwarmEvent, Swarm, yamux, Multiaddr, PeerId,
};
use serde::{Deserialize, Serialize};
use tokio::{io, io::AsyncBufReadExt, select};
use tracing_subscriber::EnvFilter;

// Define a message struct for leader election communication
#[derive(Serialize, Deserialize, Debug)]
struct ElectionMessage {
    view: u64, // View number
    is_leader: bool, // Indicates if the sender is claiming leader role
}

#[derive(Clone)]
struct ElectionBehaviour {
    gossipsub: Floodsub,
    mdns: mdns::tokio::Behaviour,
    leader: Option<PeerId>, // Store the current leader
}

impl NetworkBehaviour for ElectionBehaviour {
    type Protocols = (Floodsub, mdns::tokio::Behaviour);

    fn new(local_key: PeerId, gossipsub_topic: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let gossipsub_config = gossipsub::ConfigBuilder::default()
            .build()
            .map_err(|e| format!("Failed to create Gossipsub config: {}", e))?;
        let gossipsub = Floodsub::new(gossipsub_config)?;

        let mdns = mdns::tokio::Behaviour::new(mdns::Config::default(), local_key.public())?;

        Ok(Self {
            gossipsub,
            mdns,
            leader: None,
        })
    }

    async fn inject_event(&mut self, event: <Self::Protocols as libp2p::swarm::NetworkBehaviour>::Event) {
        match event {
            FloodsubEvent::Message(topic, message) if topic == gossipsub::floodsub::TOPIC_PUBLIC => {
                let msg = serde_json::from_slice(&message.data)?;
                let election_msg: ElectionMessage = serde_json::from_value(msg)?;
                // Handle received election messages (e.g., view change)
                println!("Received message: {:?}", election_msg);
                // ... (logic for processing view changes, leader announcements)
                if election_msg.is_leader && Some(election_msg.view) == self.leader.as_ref().map(|id| id.clone()) {
                    // Received leader announcement for current view, update leader
                    self.leader = Some(message.source);
                } else if election_msg.view > self.leader.as_ref().map_or(0, |id| id.clone()) {
                    // Received message for a higher view, initiate leader election
                    self.leader = None;
                    // ... (logic for initiating leader election for the new view)
                }
            }
            FloodsubEvent::Subscribed { .. } => println!("Subscribed to floodsub topic"),
            _ => {}
        }

        match event {
            mdns::Event::Discovered(peers) => {
                for (peer_id, _) in peers {
                    println!("Discovered peer: {:?}", peer_id);
                    // ... (add discovered peer to some data structure)
                }
            }
            mdns::Event::Expired(peers) => {
                for (peer_id, _) in peers {
                    println!("Expired peer: {:?}", peer_id);
                    // ... (remove expired peer from data structure)
                }
            }
            _ => {}
        }
    }
}

impl NetworkBehaviourEventProcess<FloodsubEvent> for ElectionBehaviour {}
impl NetworkBehaviourEventProcess<mdns::Event> for ElectionBehaviour {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .try_init();

    // Generate our local PeerId
    let local_key = libp2p::identity::Keypair::generate_ed25519().public();

    // Set the Floodsub topic for communication
    let gossipsub_topic = "election-protocol";

    // Create a Swarm using the custom behaviour
    let mut swarm = SwarmBuilder::new_identity(local_key)

