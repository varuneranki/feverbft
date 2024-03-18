use tokio;

/*Mdns issue !todo();
https://discuss.libp2p.io/t/which-libp2p-module-do-i-use-to-broadcast-data-to-peers-in-the-network-e-g-blockchain-do-i-use-mdns-or-kademlia-for-blockchain/2172
               */

use libp2p::{ Swarm, mdns::{ Mdns, MdnsEvent}};
use libp2p::{
    identity,
    PeerId,
    Transport,
};
use libp2p::swarm::Config;

// ... other code for your FeverBFT library

// Struct for Node with additional network related fields
pub struct FeverBftNode {
    pub id: PeerId,
    // ... other FeverBFT related fields
    transport: dyn Transport<Error = std::io::Error, Output = Vec<u8>, ListenerUpgrade = (), Dial = ()>,
    swarm: Swarm<Mdns>,
}

impl FeverBftNode {
    // Function to create a new FeverBftNode with network setup
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // Generate a random PeerId
        let local_key = identity::Keypair::generate_ed25519();
        let local_peer_id = local_key.public().clone();

        // Create a Tokio based TCP transport
        let transport = Transport::default();

        // Build an Mdns service discovery layer
        let mdns = Mdns::new()?;
        let mut swarm = Swarm::new(transport, mdns, PeerId::from(local_peer_id),);

        // ... (add behavior for handling network events)

        Ok(Self { id: local_peer_id, transport, swarm })
    }
}