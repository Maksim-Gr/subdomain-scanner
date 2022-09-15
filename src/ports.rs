use crate::{
  common_ports::MOST_COMMON_PORTS_100,
  model::{Port, Subdomain},
};
use rayon::prelude::*;
use std::net::{SocketAddr, ToSocketAddrs};
use std::{net::TcpStream, time::Duration};

pub fn scan_ports(mut subdomain:Subdomain) -> Subdomain {
  let socket_addresses: Vec<SocketAddr> = format!("{}:1024", subdomain.domain)
      .to_socket_addrs()
      .expect("port scanner: Creating socket address")
      .collect();
  if socket_addresses.is_empty() {
    return subdomain;
  }
  subdomain.open_ports = MOST_COMMON_PORTS_100
      .into_par_iter()
      .map(|port| scan_port(socket_addresses[0], *port))
      .filter(|port| port.is_open)
      .collect();
  subdomain
}