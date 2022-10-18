use std::collections::BTreeMap;
use std::{fmt, vec};
use std::net::IpAddr;
use std::os::unix::io::AsRawFd;

use smoltcp::iface::{InterfaceBuilder, NeighborCache, Routes};
use smoltcp::phy::{wait as phy_wait, TunTapInterface};
use smoltcp::socket::{TcpSocket, TcpSocketBuffer};
use smoltcp::time::Instant;
use smoltcp::wire::{EthernetAddress, IpAddress, IpCidr, Ipv4Address};
use url::Url;

#[derive(Debug)]
enum HttpState {
    Connect,
    Request,
    Response,
}

#[derive(Debug)]
pub enum UpstreamError {
    Network(smoltcp::Error),
    InvalidUrl,
    Content(std::str::Utf8Error),
}

impl fmt::Display for UpstreamError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<smoltcp::Error> for UpstreamError {
    fn from(error: smoltcp::Error) -> Self {
        UpstreamError::Network(error)
    }
}

impl From<std::str::Utf8Error> for UpstreamError {
    fn from(error: std::str::Utf8Error) -> Self {
        UpstreamError::Content(error)
    }
}

fn random_port() -> u16 {
    49152 + rand::random::<u16>() % 16384
}

pub fn get(
    tap: TunTapInterface,
    mac: EthernetAddress,
    addr: IpAddr,
    url: Url,
) -> Result<(), UpstreamError> {
    let domain_name = url.host_str().ok_or(UpstreamError::InvalidUrl)?;

    let neighbor_cache = NeighborCache::new(BTreeMap::new());
    
    let tcp_rx_buffer = TcpSocketBuffer::new(vec![0; 1024]);
    let tcp_tx_buffer = TcpSocketBuffer::new(vec![0; 1024]);
    let tcp_socket = TcpSocket::new(tcp_rx_buffer, tcp_tx_buffer);

    let ip_addrs = [IpCidr::new(IpAddress::v4(192, 168, 12, 1), 24)];

    let fd = tap.as_raw_fd();
    let mut routes = Routes::new(BTreeMap::new());
    let default_gateway = Ipv4Address::new(192, 168, 12, 100);
    routes.add_default_ipv4_route(default_gateway).unwrap();
    let mut iface = InterfaceBuilder::new(tap, vec![])
        .hardware_addr(mac.into())
        .neighbor_cache(neighbor_cache)
        .ip_addrs(ip_addrs)
        .routes(routes)
        .finalize();
    
    let tcp_handle = iface.add_socket(tcp_socket);
    
    let http_header = format!(
        "GET {} HTTP/1.0\r\nHost: {}\r\nConnection: close\r\n\r\n",
        url.path(),
        domain_name
    );

    let mut state = HttpState::Connect;
    'http: loop {
        let timestamp = Instant::now();
        match iface.poll(timestamp) {
            Ok(_) => {}
            Err(smoltcp::Error::Unrecognized) => {}
            Err(e) => {
                eprintln!("error: {:?}", e);
            }
        }

        {
            let (socket, cx) = iface.get_socket_and_context::<TcpSocket>(tcp_handle);

            state = match state {
                HttpState::Connect if !socket.is_active() => {
                    eprintln!("connecting");
                    // See https://github.com/rohankumardubey/embassy/blob/bae26c46edbb3f439cd5deb8b3d04c41403fdd5f/embassy-net/src/tcp_socket.rs
                    socket.connect(cx, (addr, 80), random_port())?;
                    HttpState::Request
                }

                HttpState::Request if socket.may_send() => {
                    eprintln!("sending request");
                    socket.send_slice(http_header.as_ref())?;
                    HttpState::Response
                }

                HttpState::Response if socket.can_recv() => {
                    socket.recv(|raw_data| {
                        let output = String::from_utf8_lossy(raw_data);
                        println!("{}", output);
                        (raw_data.len(), ())
                    })?;
                    HttpState::Response
                }

                HttpState::Response if !socket.may_recv() => {
                    eprintln!("received complete response");
                    break 'http;
                }

                _ => state,
            }
        }

        phy_wait(fd, iface.poll_delay(timestamp)).expect("wait error");
    }

    Ok(())
}