use crate::packet::TCPPacket;
use crate::socket::{SockID, Socket, TcpStatus};
use crate::tcpflags;
use anyhow::{Context, Result};
use pnet::packet::{ip::IpNextHeaderProtocols, tcp::TcpPacket, Packet};
use pnet::transport::{self, TransportChannelType};
use rand::{rngs::ThreadRng, Rng};
use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr};
use std::process::Command;
use std::sync::{Arc, Condvar, Mutex, RwLock, RwLockWriteGuard};
use std::time::{Duration, SystemTime};
use std::{cmp, ops::Range, str, thread};

const UNDETERMINED_IP_ADDR: std::net::Ipv4Addr = Ipv4Addr::new(0, 0, 0, 0);
const UNDETERMINED_PORT: u16 = 0;
const MAX_TRANSMITTION: u8 = 5;
const RETRANSMITTION_TIMEOUT: u64 = 3;
const MSS: usize = 1460;
const PORT_RANGE: Range<u16> = 40000..60000;

pub struct TCP {
    // ハッシュテーブルは複数のスレッドから書き込まれるためRwLockで保護する
    // RwLockは多数のreaderまたは最大1人のwriterを許可する
    sockets: RwLock<HashMap<SockID, Socket>>,
    event_condvar: (Mutex<Option<TCPEvent>>, Condvar),
}

impl TCP {
    pub fn new() -> Arc<Self> {
        let sockets = RwLock::new(HashMap::new());
        // Arcを返す
        // Arc/Rcは参照カウントされた共有スマートポインタ
        let tcp = Arc::new(Self {
            sockets,
            event_condvar: (Mutex::new(None), Condvar::new()),
        });
        let cloned_tcp = tcp.clone();
        std::thread::spawn(move || {
            // パケットの受診用スレッド
            cloned_tcp.receive_handler().unwrap();
        });
        tcp
    }

    fn select_unused_port(&self, rng: &mut ThreadRng) -> Result<u16> {
        Ok(33445)
    }

    pub fn connect(&self, addr: Ipv4Addr, port: u16) -> Result<SockID> {
        let mut rng = rand::thread_rng();
        let mut socket = Socket::new(
            get_source_addr_to(addr)?,
            addr,
            self.select_unused_port(&mut rng)?,
            port,
        )?;
        socket.send_tcp_packet(tcpflags::SYN, &[])?;
        let sock_id = socket.get_sock_id();
        Ok(sock_id)
    }
}

fn get_source_addr_to(addr: Ipv4Addr) -> Result<Ipv4Addr>{
    Ok("10.0.0.1".parse().unwrap())
}
