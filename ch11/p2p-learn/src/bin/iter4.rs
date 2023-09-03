use libp2p::{futures::StreamExt, identity,
    mdns::{tokio::Behaviour, Event, Config},
    swarm::{SwarmBuilder, SwarmEvent},
    PeerId,
};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let new_key = identity::Keypair::generate_ed25519();
    let new_peer_id = PeerId::from(new_key.public());
    println!("Local peer id: {:?}", new_peer_id);

    let behaviour = Behaviour::new(Config::default(), new_peer_id)?;
    let transport = libp2p::tokio_development_transport(new_key)?;
    let mut swarm = SwarmBuilder::with_tokio_executor(
        transport, behaviour, new_peer_id).build();
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => {
                println!("Listening on local address {:?}", address)
            }
            SwarmEvent::Behaviour(Event::Discovered(peers)) => {
                for (peer, addr) in peers {
                    println!("Discovered {} {}", peer, addr)
                }
            }
            SwarmEvent::Behaviour(Event::Expired(expired)) => {
                for (peer, addr) in expired {
                    println!("Expired {} {}", peer, addr)
                }
            }
            _ => {} 
        }
    }
}
