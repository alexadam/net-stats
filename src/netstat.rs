use netstat2::*;

pub struct Netstat {
  pub local_addr: String,
  pub local_port: u16,
  pub remote_addr: String,
  pub remote_port: u16,
  pub protocol: String,
  pub pids: Vec<u32>,
  pub state: String
}

// impl Netstat {
//   pub(crate) const fn ref_array(&self) -> [&String; 7] {
//     [&self.local_addr, &self.local_port.to_string(), &self.remote_addr, &self.remote_port.to_string(),
//       &self.protocol, &self.pids.get(0).unwrap().to_string(), &self.state]
//   }
//
// }


pub fn get_all_connections() -> Vec<Netstat> {
  let af_flags = AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6;
  let proto_flags = ProtocolFlags::TCP | ProtocolFlags::UDP;
  let sockets_info = get_sockets_info(af_flags, proto_flags).unwrap();

  let mut netStats = Vec::new();

  for si in sockets_info {
    match si.protocol_socket_info {
      ProtocolSocketInfo::Tcp(tcp_si) => {
        netStats.push(Netstat {
          local_addr: tcp_si.local_addr.to_string(),
          local_port: tcp_si.local_port,
          remote_addr: tcp_si.remote_addr.to_string(),
          remote_port: tcp_si.remote_port,
          protocol: String::from("TCP"),
          pids: si.associated_pids.clone(),
          state: tcp_si.state.to_string(),
        });
        // println!(
        //   "TCP {}:{} -> {}:{} {:?} - {}",
        //   tcp_si.local_addr,
        //   tcp_si.local_port,
        //   tcp_si.remote_addr,
        //   tcp_si.remote_port,
        //   si.associated_pids,
        //   tcp_si.state
        // )
      },
      ProtocolSocketInfo::Udp(udp_si) => {
        netStats.push(Netstat {
          local_addr: udp_si.local_addr.to_string(),
          local_port: udp_si.local_port,
          remote_addr: String::from("?"),
          remote_port: 0,
          protocol: String::from("UDP"),
          pids: si.associated_pids.clone(),
          state: String::from("?"),
        });

        // println!(
        //   "UDP {}:{} -> *:* {:?}",
        //   udp_si.local_addr, udp_si.local_port, si.associated_pids
        // )
      },
    }
  }

  netStats

  // return Option::from("");
}

