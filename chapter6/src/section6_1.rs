use std::fs::File;
use std::io::{Read, BufReader, BufRead};

const BUFSIZE: usize = 1024;

pub fn fn_6_1() {
    match list_6_1() {
        Ok(ok) => println!("OK: {:?}", ok),
        Err(err) => println!("Err: {:?}", err),
    };
    match list_6_2() {
        Ok(ok) => println!("OK: {:?}", ok),
        Err(err) => println!("Err: {:?}", err),
    };
    list_6_3().unwrap();   
}

fn list_6_1() -> std::io::Result<()> {
    let mut f = File::open("input.txt")?;
    let mut buf = [0_u8; BUFSIZE];

    let mut lines: Vec<String> = Vec::new();
    let mut linebuf = String::new();

    loop {
        let read_size = f.read(&mut buf)?;

        if read_size == 0 {
            break;
        }

        for cc in  &buf[..read_size]{
            if *cc == b'\n' {
                lines.push(linebuf);
                linebuf = String::new();
            } else {
                linebuf.push(*cc as char);
            }
        }
    }

    println!("{:?}", lines);

    Ok(())
}

fn list_6_2() -> std::io::Result<()> {
    let mut f = File::open("input.txt")?;

    let mut lines = Vec::new();
    let mut linebuf = String::new();

    let mut buf = Vec::new();
    let read_size = f.read_to_end(&mut buf)?;

    for cc in &buf[..read_size] {
        if *cc == b'\n' {
            lines.push(linebuf);
            linebuf = String::new();
        } else {
            linebuf.push(*cc as char);
        }
    }

    println!("{:?}", lines);

    Ok(())
}

fn list_6_3() -> std::io::Result<()> {
    let f = File::open("input.txt")?;
    let f = BufReader::new(f);

    let mut lines = Vec::new();

    for ll in f.lines() {
        lines.push(ll.unwrap());
    }

    println!("{:?}", lines);

    Ok(())
}
