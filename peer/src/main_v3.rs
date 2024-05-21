use futures::stream::StreamExt;
use libp2p::{gossipsub, mdns, noise, swarm::NetworkBehaviour, swarm::SwarmEvent, tcp, yamux};
use rand::random;
use std::error::Error;
use std::time::Duration;
use tokio::{io, io::AsyncBufReadExt, select, time::sleep};
use tracing_subscriber::EnvFilter;
use chrono::{NaiveDateTime, Timelike};
use regex::Regex;

mod clocky;

//Message string constants
/*const MESSAGE_ATTACK: &str = "ATTACK";
const MESSAGE_RETREAT: &str = "RETREAT";
const MESSAGE_START: &str = "START";
const MESSAGE_STOP: &str = "STOP";
const MESSAGE_ECHO: &str = "ECHO";
const MESSAGE_BYZANTINE: &str = "BYZANTINE";
*/

// We create a custom network behaviour that combines Gossipsub and Mdns.
//#[derive(NetworkBehaviour, Clone)]
#[derive(NetworkBehaviour)]
struct MyBehaviour {
    gossipsub: gossipsub::Behaviour,
    mdns: mdns::tokio::Behaviour,
}

//Cloning not needed and method doesnt exist
/* 
impl Clone for MyBehaviour {
    fn clone(&self) -> Self {
        // Clone internal fields manually
        let gossipsub = self.gossipsub.clone();
        let mdns = self.mdns.clone();
        MyBehaviour { gossipsub, mdns }
    }
}*/


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
    
    println!("Listening on network");

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

    tokio::spawn(async move {
         clocky::synchronize_logical_clock();
   	 CommunicateReceive(&mut swarm, topic).await;
    });

    loop {
        // Main thread can continue doing other tasks if needed
        tokio::time::sleep(Duration::from_secs(60)).await;
    }

    Ok(())
}

async fn CommunicateSend(swarm: &mut libp2p::Swarm<MyBehaviour>, topic: gossipsub::IdentTopic, message: &str) {
    let logical_clock_time = clocky::current_logical_clock_time();
    let message_with_time = format!("<{}> {}", logical_clock_time, message);
    println!("{}", message_with_time);
    if let Err(e) = swarm
        .behaviour_mut()
        .gossipsub
        .publish(topic, message_with_time.as_bytes()) {
        println!("Publish error: {e:?}");
    }
}


async fn CommunicateReceive(swarm: &mut libp2p::Swarm<MyBehaviour>, topic: gossipsub::IdentTopic) {
    let mut attack_count = 0;
    let mut retreat_count = 0;
    let mut start_sender = None;

    loop {
        select! {
            event = swarm.select_next_some() => match event {
                SwarmEvent::Behaviour(MyBehaviourEvent::Gossipsub(gossipsub::Event::Message { propagation_source: peer_id, message_id: id, message })) => {
                    clocky::synchronize_logical_clock();
                    let current_time = clocky::current_logical_clock_time();
                    let message_str = String::from_utf8_lossy(&message.data).trim().to_string();
                    println!("Got message: '{}' at '{}' with id: {} from peer: {}", message_str, current_time, id, peer_id);

                    let re = Regex::new(r"START (ATTACK|RETREAT)").unwrap();
                    if re.is_match(&message_str) {
                        start_sender = Some(peer_id);
                        if message_str == "START ATTACK" {
                            CommunicateSend(swarm, topic.clone(), "ATTACK");
                        } else if message_str == "START RETREAT" {
                            CommunicateSend(swarm, topic.clone(), "RETREAT");
                        }

                        let attack_re = Regex::new(r"ATTACK").unwrap();
                        let retreat_re = Regex::new(r"RETREAT").unwrap();
                        let end_time = tokio::time::Instant::now() + Duration::from_millis(300);

                        loop {
                            let timeout = tokio::time::timeout_at(end_time, swarm.select_next_some());
                            if let Ok(event) = timeout.await {
                                match event {
                                    SwarmEvent::Behaviour(MyBehaviourEvent::Gossipsub(gossipsub::Event::Message { message, .. })) => {
                                        let msg_str = String::from_utf8_lossy(&message.data).trim().to_string();
                                        if attack_re.is_match(&msg_str) {
                                            attack_count += 1;
                                        } else if retreat_re.is_match(&msg_str) {
                                            retreat_count += 1;
                                        }
                                    },
                                    _ => {}
                                }
                            } else {
                                break;
                            }
                        }

                        let consensus_msg = if attack_count > retreat_count {
                            "ATTACK"
                        } else {
                            "RETREAT"
                        };

                        if let Some(sender) = start_sender {
                        	println!("Implement Log function");
                            //Log(&mut swarm, topic.clone(), consensus_msg, sender).await;
                        }

                        break;
                    }
                },
                _ => {}
            }
        }
    }
}

/*
async fn Log(
    swarm: &mut libp2p::Swarm<MyBehaviour>,
    _: Topic<gossipsub::topic::IdentityHash>, // Annotate the type or use a parameter name
    message: &str,
    peer_id: libp2p::PeerId,
) {
    let logical_clock_time = clocky::current_logical_clock_time();
    let log_message = format!("<{}> LOG: {}", logical_clock_time, message);
    println!("Sending log message to {}: {}", peer_id, log_message);

    if let Err(e) = swarm
        .behaviour_mut()
        .gossipsub
        .publish(topic.clone(), log_message.as_bytes())
    {
        println!("Publish error: {:?}", e);
    }
}*/




/*    println!("Enter messages via STDIN and they will be sent to connected peers using Gossipsub");

    // Kick it off
    loop {
        select! {
            Ok(Some(line)) = stdin.next_line() => {
            	    tokio::spawn(async move {
        		clocky::synchronize_logical_clock();
   		});
            
            		
             	 let logical_clock_time = clocky::current_logical_clock_time();
           	 let message_with_time = format!("<{}> {}", logical_clock_time, line.trim());
           	 
           	 println!("{}",message_with_time);
                 if let Err(e) = swarm
                    .behaviour_mut().gossipsub
                    .publish(topic.clone(), message_with_time.as_bytes()) {
                    println!("Publish error: {e:?}");
                }
            }
            event = swarm.select_next_some() => match event {
                SwarmEvent::Behaviour(MyBehaviourEvent::Mdns(mdns::Event::Discovered(list))) => {
                    for (peer_id, _multiaddr) in list {
                        println!("mDNS discovered a new peer: {peer_id}");
                        swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer_id);
                    }
                },
                SwarmEvent::Behaviour(MyBehaviourEvent::Mdns(mdns::Event::Expired(list))) => {
                    for (peer_id, _multiaddr) in list {
                        println!("mDNS discover peer has expired: {peer_id}");
                        swarm.behaviour_mut().gossipsub.remove_explicit_peer(&peer_id);
                    }
                },
                SwarmEvent::Behaviour(MyBehaviourEvent::Gossipsub(gossipsub::Event::Message {
                    propagation_source: peer_id,
                    message_id: id,
                    message,
                })) => {
                    tokio::spawn(async move {
       		 clocky::synchronize_logical_clock();
    			});
                    let current_time = clocky::current_logical_clock_time();
                    let message_str = String::from_utf8_lossy(&message.data).trim().to_string();
                    
                    println!("Got message: '{}' at '{}' with id: {id} from peer: {peer_id}", message_str, current_time, id = id, peer_id = peer_id);
                },
                SwarmEvent::NewListenAddr { address, .. } => {
                    println!("Local node is listening on {address}");
                }
                _ => {}
            }
        }
    }
}*/
