use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

use clap::Parser;

fn main() {
    //connect to proxy
    let args = Arguments::parse();
    dbg!(&args);
    args.do_thing(&args.host);
    let mut tcp_connection = TcpStream::connect((&args.host as &str, args.port)).unwrap();
    let negotiate_packet = &[0x05, 0x01, 0x01];
    tcp_connection.write(negotiate_packet).unwrap();
    let buffer: &mut [u8] = &mut [];
    let negotiate_response = tcp_connection.read(buffer).unwrap();
    println!("{}", negotiate_response);
    //send packagek
    //finalize
    loop {}
}

#[derive(Debug, Parser)]
pub struct Arguments {
    host: String,
    port: u16,
}

impl Arguments {
    pub fn do_thing(&self, some_str: &str) {}
}
