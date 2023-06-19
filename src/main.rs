// use std::env;
use smoltcp::wire::{IpProtocol, IpVersion, Ipv4Packet, TcpPacket};
use std::{println, process::exit};
use windivert::*;

const DIVERT_BUF_SIZE: usize = 64 * 1024;

fn is_subsequence(a: &Vec<u8>, b: Vec<u8>) -> Option<i32> {
    for i in 0..a.len() {
        if a[i]==b[0]{
            let mut index=1;
            if !(a.len() < i+b.len()) {
                for j in 1..b.len() {
                    if a[i+j] == b[j] {
                        index+=1;
                    }
                }
                if index==b.len() {
                    println!("here is something!! : {}",a[i+b.len()]);
                    return Some(10);
                }
                
            }
        }
    }
    return None;
}

fn decimal_to_hex(a: Vec<u8>) {
    println!("{:X?}",a);
}
fn hostname_filter(data: &Vec<u8>) -> Vec<u8>{
    let query:Vec<u8> = vec![72, 111, 115, 116, 58, 32];
    if is_subsequence(&data, query) {
        //delete 32 element
        println!("{:?}",data);
    }
    return 

}
fn main() {
    let handle = match WinDivert::new(
        "!loopback and (ip or tcp)",
        WinDivertLayer::Network,
        0,
        Default::default(),
    ) {
        Ok(windivert) => {
            println!("windivert connect success");
            windivert
        }
        Err(errors) => {
            println!("windivert handle error, {}", errors.to_string());
            exit(0);
        }
    };

    loop {
        //
        let mut packet = match handle.recv(DIVERT_BUF_SIZE) {
            Ok(windivert_packet) => {
                // println!("recv packet {:?}", windivert_packet);
                windivert_packet
            }
            Err(errors) => {
                println!("recv packet error, {}", errors.to_string());
                exit(1);
            }
        };

        packet.data = hostname_filter(&packet.data);

        let packet_bytes = packet.data.clone();
        let ip_version = IpVersion::of_packet(&packet_bytes);
        match ip_version {
            Ok(ip_version) => {
                if ip_version == IpVersion::Ipv6 {
                    // println!("not handle ipv6 packet");
                    let _ = handle.send(packet);
                    continue;
                }

                let mut ipv4_packet = match Ipv4Packet::new_checked(packet_bytes) {
                    Ok(p) => p,
                    Err(errors) => {
                        println!("convert ipv4 packet error, {}", errors.to_string());
                        let _ = handle.send(packet);
                        continue;
                    }
                };

                match ipv4_packet.protocol() {
                    IpProtocol::Tcp => {
                        let src_addr = ipv4_packet.src_addr();
                        let dst_addr = ipv4_packet.dst_addr();

                        let mut tcp_packet = match TcpPacket::new_checked(ipv4_packet.payload_mut())
                        {
                            Ok(packet) => packet,
                            Err(error) => {
                                println!("create checked tcp packet error, {}", error.to_string());
                                let _ = handle.send(packet);
                                continue;
                            }
                        };

                        let src_port = tcp_packet.src_port();
                        let dst_port = tcp_packet.dst_port();
                        let _ = handle.send(packet);
                        println!(
                            "send tcp {}:{} => {}:{}",
                            src_addr, src_port, dst_addr, dst_port
                        );
                    }
                    _ => {
                        let _ = handle.send(packet);
                        continue;
                    }
                }
            }
            Err(errors) => {
                eprintln!("ip version error, {}", errors.to_string());
                return;
            }
        }
    }
}
