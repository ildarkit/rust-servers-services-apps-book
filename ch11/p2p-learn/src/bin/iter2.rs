use libp2p::swarm::{SwarmBuilder,
    dummy::Behaviour, SwarmEvent};
use libp2p::futures::StreamExt;
use libp2p::{identity, PeerId};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let new_key = identity::Keypair::generate_ed25519();
    let new_peer_id = PeerId::from(new_key.public());
    println!("Local peer id: {:?}", new_peer_id);
    let behaviour = Behaviour;
    let transport = libp2p::tokio_development_transport(new_key)?;
    let mut swarm = SwarmBuilder::with_tokio_executor(
        transport, behaviour, new_peer_id).build();
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => {
                println!("Listening on local address {:?}", address)
            }
            _ => {}
        }
    }
}
