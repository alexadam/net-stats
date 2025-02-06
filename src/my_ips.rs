use if_addrs::get_if_addrs;
use std::error::Error;

pub struct IP_Address {
    pub ip: String,
    pub interface: String
}


async fn getPublicIp() -> Result<IP_Address, Box<dyn Error>> {
    let response = reqwest::get("https://api.ipify.org").await?;
    let ip_info = response.text().await?;
    let ip_address = ip_info.as_str(); //.unwrap_or("Unknown IP");

    println!("Your external IP address is: {}", ip_address);

    Ok(IP_Address {ip: ip_address.to_string(), interface: "".to_string()})

}

pub async fn getMyIps() -> Result<Vec<IP_Address>, Box<dyn Error>> {
    let ifaces = get_if_addrs()?;

    let mut ipAddresses = Vec::new();

    for iface in ifaces {
        if let ip = iface.ip() {
            println!("Interface: {} - IP: {}", iface.name, ip);
            ipAddresses.push(IP_Address {ip: iface.ip().to_string(), interface: iface.name.to_string()});
        }
    }

    let externamIpAddress = getPublicIp().await;

    if let addr = externamIpAddress.unwrap() {
        ipAddresses.push(addr);
    }

    Ok(ipAddresses)
}


// use pnet::{datalink, transport};
// use pnet::datalink::Channel::Ethernet;
// // use pnet::packet::arp::ArpHardwareTypes::Ethernet;
// use pnet::packet::ethernet::{MutableEthernetPacket};
// // 5.12.71.239
// // Ipv4Addr::new(5, 12, 71, 239);
//
// fn main() {
//   // Define the network interface
//   let interfaces = datalink::interfaces();
//   let interface = interfaces
//     .into_iter()
//     .find(|iface| iface.is_up() && !iface.ips.is_empty() && !iface.is_loopback() && iface.ips.iter().any(|ip| ip.is_ipv4()))
//     .expect("No suitable interface found");
//
//   // interfaces.iter().for_each(|iface| {println!("iii {:?}", iface);
//   //   iface.ips.iter().for_each(|ip| {
//   //     if ip.is_ipv4() {
//   //
//   //     }
//   //   })
//   // });
//






//   let interfaces = datalink::interfaces();
//   let interface = interfaces
//     .into_iter()
//     .find(|iface| iface.is_up() && !iface.ips.is_empty() && !iface.is_loopback() && iface.ips.iter().any(|ip| ip.is_ipv4()))
//     .expect("No suitable interface found");
//
//   let source_ip = interface
//     .ips
//     .iter()
//     .find_map(|ip| match ip.ip() {
//       IpAddr::V4(addr) => Some(addr),
//       _ => None,
//     })
//     .expect("No IPv4 address found");