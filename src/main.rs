// use std::env;
use smoltcp::wire::{IpProtocol, IpVersion, Ipv4Packet, TcpPacket};
use std::{println, process::exit};
use windivert::*;

const DIVERT_BUF_SIZE: usize = 64 * 1024;

fn decimal_to_hex(a: Vec<u8>) {
    println!("{:X?}",a);
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
        let packet = match handle.recv(DIVERT_BUF_SIZE) {
            Ok(windivert_packet) => {
                println!("recv packet {:?}", windivert_packet);
                // decimal_to_hex(windivert_packet.data.clone());
                windivert_packet
            }
            Err(errors) => {
                println!("recv packet error, {}", errors.to_string());
                exit(1);
            }
        };

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
                        eprintln!("not supported other protocol");
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
