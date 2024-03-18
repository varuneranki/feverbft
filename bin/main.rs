use feverbft::node::FeverBftNode;
use tokio;

//git issues

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a FeverBftNode instance
    let node = FeverBftNode::new().await?;

    // Placeholder for testing network behavior
    println!("FeverBFT Node running with PeerId: {}", node.id);

    // ... (add code to simulate network interactions)

    Ok(())
}



/*use feverbft::{consensus_logic, Node};

fn main() {
    // Create some nodes for testing
    let node1 = Node::new(1);
    let node2 = Node::new(2);

    // Simulate calling consensus logic from your library
    consensus_logic(&node1, &node2);

    println!("Successfully used hotstuff_consensus functionalities!");
}*/