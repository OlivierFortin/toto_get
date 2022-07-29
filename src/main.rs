use std::io::prelude::*;
use std::net::TcpStream;
use dns_lookup::lookup_host;

fn main() -> std::io::Result<()> {
    let dns = std::env::args().nth(1).expect("no pattern given");
    let port = std::env::args().nth(2).expect("no pattern given");
    let path = std::env::args().nth(3).expect("babdas");
    let ips: Vec<std::net::IpAddr> = lookup_host(&dns).unwrap(); 
    println!("{}",ips[0]);
    let ip = ips[0];
    let mut stream = TcpStream::connect(format!("{ip}:{port}"))?;

    stream.write( format!("GET {path} HTTP/1.0\n
                   host: 127.0.0.1:5001\n
                   \n").as_bytes())?;
    let mut buffer = String::new();

    stream.read_to_string(&mut buffer)?;
    println!("{}", buffer);
    Ok(())


}
    
