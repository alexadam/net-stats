// use std::net::TcpStream;
use tokio::net::{TcpStream, UdpSocket};
use tokio::time::{timeout, Duration};
use futures::stream::{self, StreamExt};

async fn scan_port(host: &str, port: u16) -> Option<u16> {
  println!("Scanning {}:{}", host, port);
  let address = format!("{}:{}", host, port);

  // match TcpStream::connect(&address).await {
  //   Ok(_) => Some(port),  // Port is open
  //   Err(_) => None,       // Port is closed
  // }

  match timeout(Duration::from_secs(3), TcpStream::connect(&address)).await {
    Ok(Ok(_)) => {
      println!("Port {} is OPEN", port);
      Some(port)  // Connection successful
    }
    Ok(Err(e)) => {
      println!("Port {} closed: {}", port, e);
      None  // Connection failed
    }
    Err(_) => {
      println!("Port {} timed out", port);
      None  // Timeout occurred
    }
  }

}

async fn scan_udp(ip: &str, port: u16) -> Option<u16> {
  let address = format!("{}:{}", ip, port);
  let socket = match UdpSocket::bind("0.0.0.0:0").await {
    Ok(s) => s,
    Err(_) => return None, // Failed to create UDP socket
  };

  let data = [0u8; 1]; // Sending a small test packet
  let _ = socket.send_to(&data, &address).await;

  let mut buf = [0u8; 1024];
  match timeout(Duration::from_secs(3), socket.recv_from(&mut buf)).await {
    Ok(Ok(_)) => {
      println!("✅ UDP Port {} is OPEN (received response)", port);
      Some(port)
    }
    _ => {
      println!("❓ UDP Port {} is UNKNOWN (no response)", port);
      None
    }
  }
}

#[tokio::main]
pub async fn scanner() {
  let host = "82.78.48.73"; // Replace with the target host
  // let host = "192.168.88.1"; // Replace with the target host
  // let ports: Vec<u16> = (38760..38770).collect(); // Ports to scan
  let ports: Vec<u16> = (1..1000).collect(); // Ports to scan

  let stream = stream::iter(ports.into_iter().map(|port| {
    let host = host.to_string();
    tokio::spawn(async move {
      scan_port(&host, port).await
    })
  }));



  let results: Vec<_> = stream.buffer_unordered(1000).collect().await;

  println!("Open Ports:");
  for result in results {
    if let Ok(Some(port)) = result {
      println!("{}", port);
    }
  }


  /////
  /////
  /////

  let udp_ports: Vec<u16> = (1..1000).collect(); // vec![53, 67, 68, 123, 161]; // Common UDP ports (DNS, DHCP, NTP, SNMP)
  let udp_tasks: Vec<_> = udp_ports.into_iter().map(|port| {
    let ip = host.to_string();
    tokio::spawn(async move { scan_udp(&ip, port).await })
  }).collect();

  let results_udp = futures::future::join_all(udp_tasks).await;

  println!("\n🔍 Open UDP Ports:");
  for result in results_udp {
    if let Ok(Some(port)) = result {
      println!("🟢 Port {}", port);
    }
  }
}