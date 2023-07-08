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
    let sock = UdpSocket::bind("192.168.178.104:8003").unwrap();
    let mut serial_port = serialport::new("/dev/cu.usbmodem212301", 9600)
        .timeout(Duration::from_millis(1000))
        .open()
        .expect("Failed to open serial port");

    let output = "This is a test.\n".as_bytes();
    serial_port.write(output).expect("Write failed!");
    serial_port.flush().unwrap();
    let mut to_addr = String::from("192.168.178.104:8001");
    let mut reader = BufReader::new(&mut serial_port);
    let mut my_str = String::new();
    loop {

        reader.read_line(&mut my_str).unwrap();
        trim_newline(&mut my_str);
        let fader_value: i32 = my_str.parse().unwrap();
        println!("{}", fader_value);
        my_str = String::new();
        let msg_buf = encoder::encode(&OscPacket::Message(OscMessage {
            addr: "/Page1/Fader403".to_string(),
            args: vec![OscType::Int(fader_value)],
        }))
            .unwrap();
        sock.send_to(&msg_buf, &to_addr).unwrap();
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

fn trim_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}