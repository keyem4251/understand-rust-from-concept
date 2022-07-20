use std::{net::{TcpStream, TcpListener}, io::{Write, Read}};
use std::error::Error;

pub fn section8_2() {
    fn8_1();
}

fn echo_process(stream: &mut TcpStream) -> Result<(), Box<dyn Error>> {
    let mut buf = [0_u8; 1024];

    loop {
        stream.write_all("input =>".as_bytes())?;
        let read = stream.read(&mut buf);
        match read {
            Ok(0) => break,
            Ok(n) => {
                stream.write_all("output =>".as_bytes())?;
                stream.write_all(&buf[..n])?;
            },
            Err(e) => {
                return Err(e.into());
            }
        }
    }

    Ok(())
}

fn fn8_1() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        panic!("port is not specified");
    }

    let port: usize = args[1].parse().expect("Failed to get the port number");
    let addr = format!("localhost:{}", port);
    let listener = TcpListener::bind(addr).unwrap();
    println!("Listening to the port {}", port);

    loop {
        let (mut stream, _) = listener.accept().unwrap();
        std::thread::spawn(move || {
            echo_process(&mut stream).unwrap();
        });
    }
}
