use std::ops::RangeInclusive;
use std::{error::Error, env::args, str::FromStr};
use std::{
    net::{IpAddr, SocketAddr, ToSocketAddrs},
    time::Duration,
};
use cidr_utils::cidr::{IpCidr, self};
use libc::uint16_t;
use tokio::net::TcpStream;
use tokio::sync::mpsc::{channel, Sender, Receiver};

// Scans target IP/CIDR for open ports
// Adapted from: https://kerkour.com/rust-fast-port-scanner/


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


pub enum ScanTarget {
    Address(IpAddr),
    Cidr(IpCidr),
    Unknown(String)
}

async fn eval_target(target: String) -> ScanTarget {

    // CIDR
    if IpCidr::is_ip_cidr(&target) {
        println!("[*] Looks like a CIDR range.");
        ScanTarget::Cidr(IpCidr::from_str(target.as_str()).unwrap())
    
    // IP
    } else if let Ok(ip) = IpAddr::from_str(target.as_str()) {
        println!("[*] Looks like an IP address.");
        ScanTarget::Address(ip)

    // Hostname? Maybe someday
    
    // Or else
    } else {
        ScanTarget::Unknown(target)
    }
}


async fn scan(target: ScanTarget, full: bool, concurrency: usize, timeout: u64) -> Vec<String> {
    let ports = get_ports(full);
    let (tx, mut rx) = channel::<String>(concurrency);
    let mut scan_results: Vec<String> = Vec::new();

    let targets: Vec<IpAddr> = match target {
        ScanTarget::Address(a) => {
            vec![a]
        },

        ScanTarget::Cidr(c) => {
           
            // find method for converting CIDR to vec of IP 
            c.iter_as_ip_addr().collect()
        },

        ScanTarget::Unknown(u) => {
            vec![]
        }
    };

    let mut scan_targets: Vec<(IpAddr, u16)> = Vec::new();
    for addr in targets {
        get_ports(full).into_iter().for_each(|p| {
            scan_targets.push((addr, p))
        });
    }
    
    tokio::spawn(async move{    
        for (addr, port) in scan_targets{
            // .for_each_concurrent(concurrency, |port| scan_port(target, port, timeout))
            //.await;
            
            println!("[*] Scanning port {port} on host {addr}");
            let res: String = scan_target(addr, port, timeout).await.unwrap();
            if res != "" {
                tx.send(res).await.unwrap();
            }
        }

    });
    while let Some(r) = rx.recv().await {
        scan_results.push(r);
    }
    println!("{:?}", scan_results);
    
    if scan_results.is_empty(){
        scan_results.push("[*] No ports open on targets".to_string());
    }
    
    scan_results
}




async fn scan_target(target: IpAddr, port: u16, timeout: u64) -> Result<String, Box<dyn Error>> {
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
    let args: Vec<&str> = _s.split(" ").collect();
    println!("[*] Portscan args: {}", &_s);

    if args.len() <= 4 {
        Ok("[-] Improper args.\n[*] Usage: portscan [ip] [true/false] [concurrency] [timeout]\n\t  [*] Example: portscan 192.168.35.5 false 10 0 🎯".to_string())
    } else {

        let target: ScanTarget = eval_target(args[0].to_string()).await;
        
        let full: bool = args[1].parse::<bool>().unwrap();
        let concurrent: usize = args[2].parse::<usize>().unwrap();
        let timeout: u64 = args[3].parse::<u64>().unwrap();
    
        let scan_handle = tokio::spawn( async move {
            return scan(target, full,concurrent, timeout)
        });
        
        let scan_res = scan_handle.await?.await;
        let print_res = scan_res.as_slice().join("\n");
        //println!("{print_res}");
        Ok(print_res)
    }
}
       