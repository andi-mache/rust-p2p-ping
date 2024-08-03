//! # Simple Libp2p Application with Ping Protocol
//!
//! This example demonstrates how to set up a basic libp2p node using the `tokio`
//! asynchronous runtime. It configures the node to communicate over TCP with TLS
//! encryption and Yamux stream multiplexing. Additionally, it incorporates the
//! ping protocol to allow nodes to check connectivity with each other.
//!
//! ## Features Demonstrated
//! - Initializing a libp2p swarm with a new identity.
//! - Configuring transports (TCP) and stream multiplexers (Yamux).
//! - Adding behavior to the swarm (ping protocol).
//! - Listening on a random port and dialing peers.
//! - Handling swarm events asynchronously.
//!
//! ## Usage
//! Run this application and optionally pass a multi-address of a peer to dial as
//! a command-line argument. The application will start listening on a random port
//! and print out the address it's listening on. If a peer address is provided,
//! it will attempt to establish a connection to that peer and send pings.
//!
//! ```
//!
//  Replace `[peer_multiaddr]` with the actual multi-address of the peer you wish
//  to connect to, e.g., `/ip4/127.0.0.1/tcp/12345/p2p/Qm...`.
//!

use futures::prelude::*;
use libp2p::swarm::SwarmEvent;
use libp2p::{ping, Multiaddr};
use std::error::Error;
use std::time::Duration;
use tracing_subscriber::EnvFilter;

/// Main entry point of the application.
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize logging with environment filter for log level control.
    tracing_subscriber::fmt().with_env_filter(EnvFilter::from_default_env()).init();

    // Create a new libp2p SwarmBuilder instance with a randomly generated identity.
    let mut swarm = libp2p::SwarmBuilder::with_new_identity()
        .with_tokio() // Specify tokio as the executor for async operations.
        .with_tcp(
            libp2p::tcp::Config::default(), // Default TCP configuration.
            libp2p::tls::Config::new, // Enable TLS for secure communication.
            libp2p::yamux::Config::default, // Use Yamux for stream multiplexing.
        )?
        .with_behaviour(|_| ping::Behaviour::default())? // Add ping behaviour to the swarm.
        .with_swarm_config(|cfg| cfg.with_idle_connection_timeout(Duration::from_secs(30))) // Set idle connection timeout.
        .build(); // Finalize building the swarm.

    // Start listening on all interfaces at a random OS-assigned port.
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    // Attempt to dial a peer if a multi-address is provided as a command-line argument.
    if let Some(addr) = std::env::args().nth(1) {
        let remote: Multiaddr = addr.parse()?; // Parse the multi-address.
        swarm.dial(remote)?; // Dial the peer.
        println!("Dialed {addr}");
    }

    // Event loop to handle incoming swarm events.
    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => println!("Listening on {:?}", address), // Log new listen addresses.
            SwarmEvent::Behaviour(event) => println!("{:?}", event), // Log behaviour-specific events (e.g., ping responses).
            _ => {} // Ignore other events.
        }
    }
}
