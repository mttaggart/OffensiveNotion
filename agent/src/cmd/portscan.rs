use std::ops::RangeInclusive;
use std::{error::Error, env::args, str::FromStr};
use std::{
    net::{IpAddr, SocketAddr, ToSocketAddrs},
    time::Duration,
};
use libc::uint16_t;
use tokio::net::TcpStream;
use tokio::sync::mpsc::{channel, Sender, Receiver};
use cidr_utils;

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


async fn scan(target: IpAddr, full: bool, concurrency: usize, timeout: u64) -> Vec<String> {
    let ports = get_ports(full);
    let (tx, mut rx) = channel::<String>(concurrency);
    let mut scan_results: Vec<String> = Vec::new();


    tokio::spawn(async move{
        
        for port in ports{
            // .for_each_concurrent(concurrency, |port| scan_port(target, port, timeout))
            //.await;
            println!("[*] Scanning port {port} on host {target}");
            let res: String = scan_port(target, port, timeout).await.unwrap();
            if res != "" {
                tx.send(res).await.unwrap();
            }
        }

    });
    while let Some(r) = rx.recv().await {
        scan_results.push(r);
    }
    println!("{:?}", scan_results);
    scan_results
}

async fn scan_port(target: IpAddr, port: u16, timeout: u64) -> Result<String, Box<dyn Error>> {
    let timeout = Duration::from_secs(timeout);
    let socket_address = SocketAddr::new(target.clone(), port);

    match tokio::time::timeout(timeout, TcpStream::connect(&socket_address)).await {
        Ok(Ok(_)) => Ok(format!("[+] {port} is open on host {target}")),
        _ => Ok("".to_string())
    }
}

fn get_ports(full: bool) -> Vec<u16> {
    if full {
        (1..=u16::MAX).into_iter().collect()
    } else {
        MOST_COMMON_PORTS_1002.to_owned()
    }
}

pub async fn handle(_s: &String) -> Result<String, Box<dyn Error>> {
    
    let mut args: Vec<&str> = _s.split(" ").collect();
    
    println!("[*] Portscan args: {}", &_s);
    println!("{}", args.len().to_string());

    if args.len() < 3 {
    match args.len() {
        0 => {
            Ok("[-] Improper args.\n[*] Usage: portscan [ip] [true/false] [concurrency] [timeout]".to_string())
        }
    
        1 => {
            Ok("[-] Improper args.\n[*] Usage: portscan [ip] [true/false] [concurrency] [timeout]".to_string())
        }

        2 => {
        // checks for rest of args and handle if they don't exist     
        let mut ip_addr:IpAddr = args[0].parse::<IpAddr>().unwrap();
        let mut full: bool = false;
        let mut concurrent: usize = 10;
        let mut timeout: u64 = 1;

        let scan_handle = tokio::spawn( async move {
            return scan(ip_addr, full,concurrent, timeout)
        });
        
        let scan_res = scan_handle.await?.await;
    
        let print_res = scan_res.as_slice().join("\n");
        //println!("{print_res}");
        Ok(print_res)        
        }
        
        3 => {
        let mut ip_addr:IpAddr = args[0].parse::<IpAddr>().unwrap();
        let mut full: bool = args[1].parse::<bool>().unwrap();
        let mut concurrent: usize = 10;
        let mut timeout: u64 = 1;

        let scan_handle = tokio::spawn( async move {
            return scan(ip_addr, full,concurrent, timeout)
        });
        
        let scan_res = scan_handle.await?.await;
    
        let print_res = scan_res.as_slice().join("\n");
        //println!("{print_res}");
        Ok(print_res)        
        }
        _ => {
        Ok("[-] Improper args.\n[*] Usage: portscan [ip] [true/false] [concurrency] [timeout]".to_string())
        }
    }

    } else {

    let mut ip_addr:IpAddr = args[0].parse::<IpAddr>().unwrap();
    let mut full: bool = args[1].parse::<bool>().unwrap();
    let mut concurrent: usize = args[2].parse::<usize>().unwrap();
    let mut timeout: u64 = args[3].parse::<u64>().unwrap();
    
    let scan_handle = tokio::spawn( async move {
        return scan(ip_addr, full,concurrent, timeout)
    });
    
    let scan_res = scan_handle.await?.await;

    let print_res = scan_res.as_slice().join("\n");
    //println!("{print_res}");
    Ok(print_res)        
}    
        
}
