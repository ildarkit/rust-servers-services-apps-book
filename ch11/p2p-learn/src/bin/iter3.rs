use libp2p::swarm::{keep_alive, NetworkBehaviour, SwarmBuilder, SwarmEvent};
use libp2p::futures::StreamExt;
use libp2p::{ping, identity, PeerId, Multiaddr};
use std::error::Error;

#[derive(NetworkBehaviour, Default)]
struct Behaviour {
    keep_alive: keep_alive::Behaviour,
    ping: ping::Behaviour,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let new_key = identity::Keypair::generate_ed25519();
    let new_peer_id = PeerId::from(new_key.public());
    println!("Local peer id: {:?}", new_peer_id);

    let behaviour = Behaviour::default();
    let transport = libp2p::tokio_development_transport(new_key)?;
    let mut swarm = SwarmBuilder::with_tokio_executor(
        transport, behaviour, new_peer_id).build();
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    if let Some(remote_peer) = std::env::args().nth(1) {
        let remote_peer_multiaddr: Multiaddr = remote_peer.parse()?;
        swarm.dial(remote_peer_multiaddr)?;
        println!("Dialed remote peer: {:?}", remote_peer);
    }

    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => {
                println!("Listening on local address {:?}", address)
            }
            SwarmEvent::Behaviour(event) => {
                println!("Event received from peer is {:?}", event)
            }
            _ => {}
        }
    }
}
