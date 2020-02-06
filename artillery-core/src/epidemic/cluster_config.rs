use chrono::Duration;
use std::net::{ToSocketAddrs, SocketAddr, UdpSocket};
use super::constants::*;
use std::ops::Deref;

pub struct ClusterConfig {
    pub cluster_key: Vec<u8>,
    pub ping_interval: Duration,
    pub network_mtu: usize,
    pub ping_request_host_count: usize,
    pub ping_timeout: Duration,
    pub listen_addr: SocketAddr,
}

impl Default for ClusterConfig {
    fn default() -> Self {
        let directed = SocketAddr::from(([127, 0, 0, 1], CONST_DISSEMINATION_PORT));

        ClusterConfig {
            cluster_key: "default".as_bytes().to_vec(),
            ping_interval: Duration::seconds(1),
            network_mtu: CONST_MTU,
            ping_request_host_count: 3,
            ping_timeout: Duration::seconds(3),
            listen_addr: directed.to_socket_addrs().unwrap().next().unwrap(),
        }
    }
}