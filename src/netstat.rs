use std::collections::HashMap;
use std::iter::Map;
use std::net::IpAddr;
use std::str::FromStr;
use dns_lookup::lookup_addr;
use netstat2::*;
use sysinfo::{Process, System};
use std::net::{SocketAddr, ToSocketAddrs};
use std::process::{Command, Stdio};
use itertools::iproduct;

pub struct Netstat {
  pub local_addr: String,
  pub local_port: u16,
  pub remote_addr: String,
  pub remote_port: u16,
  pub dns: String,
  pub protocol: String,
  pub pids: Vec<u32>,
  pub proc_name: String,
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

  let allProcesses = get_all_processes();

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
          dns: get_dns(tcp_si.remote_addr.to_string()),
          protocol: String::from("TCP"),
          pids: si.associated_pids.clone(),
          proc_name: allProcesses.get(si.associated_pids.get(0).unwrap()).unwrap().to_string(),
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
          dns: String::from("?"),
          protocol: String::from("UDP"),
          pids: si.associated_pids.clone(),
          proc_name: allProcesses.get(si.associated_pids.get(0).unwrap()).unwrap().to_string(),
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

fn get_all_processes() -> HashMap<u32, String> {
  let mut result : HashMap<u32, String> = HashMap::new();

  let s = System::new_all();

  for (pid, process) in s.processes() {
    //println!("{} {}", pid, process.name());
    let p1 = process.name();
    match p1 {
      _ => {
        result.insert(pid.as_u32(), p1.to_str().unwrap().to_string());
        // println!("PID is -> {} {}", pid, p1.to_str().unwrap());
      }
    }
  }

  return result;
}

pub fn get_dns(ipStr: String) -> String {
  let ip: std::net::IpAddr = IpAddr::from_str(ipStr.as_str()).unwrap();// ipStr.parse().unwrap();
  if ip.is_ipv6() {
    return "?".to_string();
  }

  let output = Command::new("host")
    .arg(ipStr.to_string())
    .output()
    .expect("?");

  let stdout = String::from_utf8_lossy(&output.stdout);

  if stdout.contains("not found") {
    return "?".to_string();
  } else {
    return stdout.split(" ").last().unwrap().to_string();
  }

  return stdout.to_string();
}

