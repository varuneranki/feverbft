use chrono::{NaiveDateTime, Timelike};
use futures::stream::StreamExt;
use libp2p::{
    gossipsub, mdns, noise,
    swarm::{NetworkBehaviour, SwarmEvent},
    tcp, yamux,
};
use rand::random;
use regex::Regex;
use std::error::Error;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::{io, io::AsyncBufReadExt, select, time::sleep};
use tracing_subscriber::EnvFilter;

mod clocky;

const BYZANTINE: u8 = 1; // Change this to 1 to enable Byzantine behavior

// We create a custom network behaviour that combines Gossipsub and Mdns.
#[derive(NetworkBehaviour)]
struct MyBehaviour {
    gossipsub: gossipsub::Behaviour,
    mdns: mdns::tokio::Behaviour,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .try_init();

    let mut swarm = libp2p::SwarmBuilder::with_new_identity()
        .with_tokio()
        .with_tcp(
            tcp::Config::default(),
            noise::Config::new,
            yamux::Config::default,
        )?
        .with_quic()
        .with_behaviour(|key| {
            // Set a custom gossipsub configuration
            let gossipsub_config = gossipsub::ConfigBuilder::default()
                .heartbeat_interval(Duration::from_secs(10))
                .validation_mode(gossipsub::ValidationMode::Strict)
                .build()
                .map_err(|msg| io::Error::new(io::ErrorKind::Other, msg))?; // Temporary hack because `build` does not return a proper `std::error::Error`.

            // Build a gossipsub network behaviour
            let gossipsub = gossipsub::Behaviour::new(
                gossipsub::MessageAuthenticity::Signed(key.clone()),
                gossipsub_config,
            )?;

            let mdns =
                mdns::tokio::Behaviour::new(mdns::Config::default(), key.public().to_peer_id())?;
            Ok(MyBehaviour { gossipsub, mdns })
        })?
        .with_swarm_config(|c| c.with_idle_connection_timeout(Duration::from_secs(60)))
        .build();

    // Create a Gossipsub topic
    let topic = gossipsub::IdentTopic::new("test-net");
    // Subscribe to our topic
    swarm.behaviour_mut().gossipsub.subscribe(&topic)?;

    // Read full lines from stdin
    let mut stdin = io::BufReader::new(io::stdin()).lines();

    // Listen on all interfaces and whatever port the OS assigns
    swarm.listen_on("/ip4/0.0.0.0/udp/0/quic-v1".parse()?)?;
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;
    
    tokio::spawn(async move {
        clocky::start_logical_clock().await;
    });

    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(3600));
        loop {
            interval.tick().await;
            clocky::synchronize_logical_clock();
        }
    });


    println!("Enter messages via STDIN and they will be sent to connected peers using Gossipsub");

    // Kick it off
    loop {
        select! {
            Ok(Some(line)) = stdin.next_line() => {
                if let Err(e) = swarm
                    .behaviour_mut().gossipsub
                    .publish(topic.clone(), line.as_bytes()) {
                    println!("Publish error: {e:?}");
                }
            }
            event = swarm.select_next_some() => handle_event(event, &mut swarm).await,
        }
    }
}

async fn handle_event(
    event: SwarmEvent<MyBehaviourEvent>,
    swarm: &mut libp2p::Swarm<MyBehaviour>,
) {
    match event {
        SwarmEvent::Behaviour(MyBehaviourEvent::Mdns(mdns::Event::Discovered(list))) => {
            for (peer_id, _multiaddr) in list {
                println!("mDNS discovered a new peer: {peer_id}");
                swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer_id);
            }
        }
        SwarmEvent::Behaviour(MyBehaviourEvent::Mdns(mdns::Event::Expired(list))) => {
            for (peer_id, _multiaddr) in list {
                println!("mDNS discover peer has expired: {peer_id}");
                swarm.behaviour_mut().gossipsub.remove_explicit_peer(&peer_id);
            }
        }
        SwarmEvent::Behaviour(MyBehaviourEvent::Gossipsub(gossipsub::Event::Message {
            propagation_source: peer_id,
            message_id: id,
            message,
        })) => {
        	clocky::synchronize_logical_clock();
               let current_time = clocky::current_logical_clock_time();
            let message_str = String::from_utf8_lossy(&message.data).trim().to_string();
            println!("Got message: '{}' at '{}' with id: {} from peer: {}", message_str, current_time, id, peer_id);

            if message_str == "START ATTACK" || message_str == "START RETREAT" {
                if BYZANTINE == 0 {
                    if message_str == "START ATTACK" {
                        send_message(swarm, "ATTACK").await;
                    } else if message_str == "START RETREAT" {
                        send_message(swarm, "RETREAT").await;
                    }
                } else if BYZANTINE == 1 {
                    if message_str == "START ATTACK" {
                        send_message(swarm, "RETREAT").await;
                    } else if message_str == "START RETREAT" {
                        send_message(swarm, "ATTACK").await;
                    }
                    //let response = if random::<bool>() { "ATTACK" } else { "RETREAT" };
                    //send_message(swarm, response).await;
                }

                let (attack_count, retreat_count) = count_messages(swarm, 5000).await;
                print_consensus(attack_count, retreat_count);
            }
        }
        SwarmEvent::NewListenAddr { address, .. } => {
            println!("Local node is listening on {address}");
        }
        _ => {}
    }
}


async fn send_message(
    swarm: &mut libp2p::Swarm<MyBehaviour>,
    message: &str,
) {
    let topic = gossipsub::IdentTopic::new("test-net");
    if let Err(e) = swarm
        .behaviour_mut()
        .gossipsub
        .publish(topic, message.as_bytes())
    {
        println!("Publish error: {e:?}");
    }
}

async fn count_messages(
    swarm: &mut libp2p::Swarm<MyBehaviour>,
    duration_ms: u64,
) -> (u32, u32) {
    let mut attack_count = 0;
    let mut retreat_count = 0;
    let attack_re = Regex::new(r"ATTACK").unwrap();
    let retreat_re = Regex::new(r"RETREAT").unwrap();
    let end_time = tokio::time::Instant::now() + Duration::from_millis(duration_ms);

    while tokio::time::Instant::now() < end_time {
        select! {
            event = swarm.select_next_some() => match event {
                SwarmEvent::Behaviour(MyBehaviourEvent::Gossipsub(gossipsub::Event::Message { message, .. })) => {
                    let msg_str = String::from_utf8_lossy(&message.data).trim().to_string();
                    if attack_re.is_match(&msg_str) {
                        attack_count += 1;
                    } else if retreat_re.is_match(&msg_str) {
                        retreat_count += 1;
                    }
                },
                _ => {}
            },
            _ = sleep(Duration::from_millis(duration_ms)) => break,
        }
    }

    (attack_count, retreat_count)
}

fn print_consensus(attack_count: u32, retreat_count: u32) {
    clocky::synchronize_logical_clock();
               let current_time = clocky::current_logical_clock_time();
    let consensus = if attack_count > retreat_count {
        "ATTACK"
    } else {
        "RETREAT"
    };
    
    println!("End of Consensus Result at {}: {}", current_time, consensus);
}

