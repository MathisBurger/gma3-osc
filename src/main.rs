extern crate rosc;

use rosc::encoder;
use rosc::{OscMessage, OscPacket, OscType};
use std::net::{SocketAddrV4, UdpSocket};
use std::str::FromStr;
use std::time::Duration;
use std::{env, f32, io, thread};
use std::io::{BufRead, BufReader, Read, Write};
use serialport;


fn main() {
    let args: Vec<String> = env::args().collect();
    let usage = format!(
        "Usage: {} HOST_IP:HOST_PORT CLIENT_IP:CLIENT_PORT",
        &args[0]
    );
    let sock = UdpSocket::bind("127.0.0.1:9107").unwrap();
    let mut serial_port = serialport::new("/dev/cu.usbmodem2123201", 9600)
        .timeout(Duration::from_millis(1000))
        .open()
        .expect("Failed to open serial port");

    let output = "This is a test.\n".as_bytes();
    serial_port.write(output).expect("Write failed!");
    serial_port.flush().unwrap();
    let mut to_addr = String::from("127.0.0.1:9104");
    let mut buf = [0u8; rosc::decoder::MTU];

    loop {
        /*match sock.recv_from(&mut buf) {
            Ok((size, addr)) => {
                println!("Received packet with size {} from: {}", size, addr);
                let (_, packet) = rosc::decoder::decode_udp(&buf[..size]).unwrap();
                handle_packet(packet);
            }
            Err(e) => {
                println!("Error receiving from socket: {}", e);
                break;
            }
        }*/
        //let mut reader = BufReader::new(&mut serial_port);
        //let mut my_str = String::new();
        //reader.read_line(&mut my_str).unwrap();
        sock.send("/cmd,s,FaderMaster Page 1.402 At 100".as_bytes()).unwrap();
    }
}

fn handle_packet(packet: OscPacket) {
    match packet {
        OscPacket::Message(msg) => {
            println!("OSC address: {}", msg.addr);
            println!("OSC arguments: {:?}", msg.args);
        }
        OscPacket::Bundle(bundle) => {
            println!("OSC Bundle: {:?}", bundle);
        }
    }
}