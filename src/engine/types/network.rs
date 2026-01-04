use rand_core::OsRng;
use std::convert::TryInto;
use std::io::Read;
use std::io::Write;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4, TcpStream, UdpSocket};
use x25519_dalek::{EphemeralSecret, PublicKey, SharedSecret};

use crate::engine::{Error, ErrorKind};

pub const DEFAULT_TCP_PORT: u16 = 8000;
pub const DEFAULT_UDP_PORT: u16 = 8500;
pub const SERVER_ADDRESS: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
pub const UDP_LISTEN_ADDRESS: IpAddr = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
pub const TCP_SERVER_SOCKET: SocketAddr = SocketAddr::new(SERVER_ADDRESS, DEFAULT_TCP_PORT);
pub const UDP_LISTEN_SOCKET: SocketAddr = SocketAddr::new(UDP_LISTEN_ADDRESS, DEFAULT_UDP_PORT);
pub const TCP_HEADER_SIZE: usize = 5;
pub const X25519_KEY_SIZE: usize = 32;
pub const CLIENT_VERSION_SIZE: usize = 2;

pub const HEL_HEADER: [u8; TCP_HEADER_SIZE] = [0, 34, b'H', b'E', b'L'];
pub const CLIENT_VERSION: u16 = 1;
pub const SERVER_VERSION: u16 = 1;

pub struct Network {
    pub tcp_stream: Option<TcpStream>,
    pub udp_listener: Option<UdpSocket>,
    pub udp_address: Option<SocketAddr>,
    pub x25519_public: PublicKey,
    pub x25519_private: Option<EphemeralSecret>,
    pub x25519_shared: Option<SharedSecret>,
    pub nounce_count: u32,
}

impl Default for Network {
    fn default() -> Self {
        let x25519_private = Some(EphemeralSecret::random_from_rng(OsRng));
        let x25519_public = PublicKey::from(x25519_private.as_ref().unwrap());
        let x25519_shared = None;
        Self {
            tcp_stream: None,
            udp_listener: None,
            udp_address: None,
            x25519_public,
            x25519_private,
            x25519_shared,
            nounce_count: 0,
        }
    }
}

impl Network {
    pub fn send_hel(&mut self) -> Result<(u16, u16), Error> {
        if self.tcp_stream.is_some() {
            return Err(Error::new(
                "client already connected to server",
                ErrorKind::Network,
            ));
        }
        let mut stream = match TcpStream::connect(TCP_SERVER_SOCKET) {
            Ok(s) => s,
            Err(e) => {
                return Err(Error::from(
                    e,
                    "failed to conenct to game server",
                    ErrorKind::Network,
                ));
            }
        };

        stream.write(&HELMessage::to_bytes(&self.x25519_public));

        let mut buffer = [0u8; std::u16::MAX as usize + 100];
        match stream.read(&mut buffer) {
            Ok(_) => {}
            Err(e) => {
                return Err(Error::from(
                    e,
                    "failed to recieve server response",
                    ErrorKind::Network,
                ));
            }
        }

        let msg = match self.parse_tcp_message(&buffer) {
            Ok(val) => val,
            Err(e) => return Err(e),
        };
        if msg.kind == "REJ" {
            return Err(Error::new("HEL Message Rejected", ErrorKind::Network))
        }
        if msg.kind != "ACK" {
            return Err(Error::new("Invalid HEL Message Response", ErrorKind::Network));
        }

        let peer_ver = ((msg.body[0] as u16)  << 8) + (msg.body[1] as u16);
        if peer_ver != SERVER_VERSION {
            return Err(Error::new("Unexpected Server Version", ErrorKind::Network));
        }
        let peer_tick_rate = ((msg.body[2] as u16) << 8) + (msg.body[3] as u16);
        let peer_pk: PublicKey = PublicKey::from(match <[u8;32]>::try_from(&msg.body[4..]){
            Ok(pk)=> pk,
            Err(e) => return Err(Error::from(e, "failed to convert msg bytes to peer public key", ErrorKind::Network)),
        });
        if self.x25519_private.is_some() {
            let priv_key = match self.x25519_private.take() {
                Some(s) => s,
                None => return Err(Error::new("no x25519 private key exists", ErrorKind::Network)),
            };
            self.x25519_shared = Some(priv_key.diffie_hellman(&peer_pk));

        } else {
            return Err(Error::new("no x25519 private key exist for client", ErrorKind::Network));
        }
        self.tcp_stream = Some(stream);
        Ok((peer_ver, peer_tick_rate))
    }
    pub fn recv_ack(&mut self) {}

    pub fn parse_tcp_message(&self, buffer: &[u8]) -> Result<TCPMessage, Error> {
        if buffer.len() >= TCP_HEADER_SIZE {
            return Err(Error::new("recieved packet too small", ErrorKind::Network)); // Buffer too small
        }
        match self.x25519_shared.as_ref() {
            Some(key) => {
                todo!()
            }
            None => {
                let msg_size = (buffer[0] as u16) << 8 + buffer[1] as u16;
                let msg_type = String::from_utf8(buffer[2..5].to_vec()).unwrap();
                if msg_size as usize != buffer[5..].len() {
                    return Err(Error::new("Invalid Message Format", ErrorKind::Network)) // Invalid Message size not correctly declared
                }
                Ok(TCPMessage{
                    size: msg_size,
                    kind: msg_type,
                    body: buffer[5..].to_vec(),
                })
            }
        }
    }
}

pub struct TCPMessage {
    pub size: u16,
    pub kind: String,
    pub body: Vec<u8>
}

pub struct HELMessage ();

impl HELMessage {
    pub fn to_bytes(key: &PublicKey) -> Vec<u8> {
        let mut buffer: Vec<u8> = Vec::with_capacity(100);
        buffer.push(0);
        buffer.push((X25519_KEY_SIZE + CLIENT_VERSION_SIZE) as u8);
        buffer.push(b'H');
        buffer.push(b'E');
        buffer.push(b'L');
        buffer.push((CLIENT_VERSION >> 8) as u8);
        buffer.push(CLIENT_VERSION as u8);
        buffer.append(&mut key.to_bytes().to_vec());
        buffer
    }
}
