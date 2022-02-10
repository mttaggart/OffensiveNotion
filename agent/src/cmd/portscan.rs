use std::{error::Error, env::args, str::FromStr};
use std::{
    net::{IpAddr, SocketAddr, ToSocketAddrs},
    time::Duration,
};
use futures::{self, stream, StreamExt};
use tokio::net::TcpStream;

// Scans target IP/CIDR for open ports
// Adapted from: https://kerkour.com/rust-fast-port-scanner/
//
//     portscan [target/s] [ports ('common', 'all', or comma delimited list)] [icmp|arp|none]
// Examples:
//      portscan 172.16.5.0/24 common icmp
//      portscan 172.16.5.4 80,3389,135,139,445,443 arp
//      portscan 172.16.5.4 all icmp
// 
// 

// from awk '$2~/tcp$/' /usr/share/nmap/nmap-services | sort -r -k3 | head -n 1000 | tr -s ' ' | cut -d '/' -f1 | sed 's/\S*\s*\(\S*\).*/\1,/'
pub const MOST_COMMON_PORTS_1002: &[u16] = &[
    5601, 9300, 80, 23, 443, 21, 22, 25, 3389, 110, 445, 139, 143, 53, 135, 3306, 8080, 1723, 111,
    995, 993, 5900, 1025, 587, 8888, 199, 1720, 465, 548, 113, 81, 6001, 10000, 514, 5060, 179,
    1026, 2000, 8443, 8000, 32768, 554, 26, 1433, 49152, 2001, 515, 8008, 49154, 1027, 5666, 646,
    5000, 5631, 631, 49153, 8081, 2049, 88, 79, 5800, 106, 2121, 1110, 49155, 6000, 513, 990, 5357,
    427, 49156, 543, 544, 5101, 144, 7, 389, 8009, 3128, 444, 9999, 5009, 7070, 5190, 3000, 5432,
    1900, 3986, 13, 1029, 9, 5051, 6646, 49157, 1028, 873, 1755, 2717, 4899, 9100, 119, 37, 1000,
    3001, 5001, 82, 10010, 1030, 9090, 2107, 1024, 2103, 6004, 1801, 5050, 19, 8031, 1041, 255,
];

async fn scan(target: IpAddr, full: bool, concurrency: usize, timeout: u64) {
    let ports = stream::iter(get_ports(full));

    ports
        .for_each_concurrent(concurrency, |port| scan_port(target, port, timeout))
        .await;
}

async fn scan_port(target: IpAddr, port: u16, timeout: u64) {
    let timeout = Duration::from_secs(timeout);
    let socket_address = SocketAddr::new(target.clone(), port);

    match tokio::time::timeout(timeout, TcpStream::connect(&socket_address)).await {
        Ok(Ok(_)) => println!("{}", port),
        _ => {}
    }
}

fn get_ports(full: bool) -> Box<dyn Iterator<Item = u16>> {
    if full {
        Box::new((1..=u16::MAX).into_iter())
    } else {
        Box::new(MOST_COMMON_PORTS_1002.to_owned().into_iter())
    }
}

#[tokio::main]
pub async fn handle(_s: &String) -> Result<String, Box<dyn Error>> {
    println!("[*] Portscan args: {}", _s);
    let ip_addr = (_s).parse::<IpAddr>().unwrap();
    scan(ip_addr,true,1024, 2);

    Ok("Under Construction".to_string())
}
