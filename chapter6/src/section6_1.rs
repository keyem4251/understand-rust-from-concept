use std::fs::File;
use std::io::{Read, Cursor};
use byteorder::{LittleEndian, ReadBytesExt};
use crate::line_read::get_lines;

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
    let lines = get_lines(f);
    
    println!("{:?}", lines);

    Ok(())
}

#[allow(non_snake_case)]
#[derive(Debug)]
struct BmpFileHeader {
    bfType: [u8; 2],
    bfSize: u32,
    bfReserved1: u16,
    bfReserved2: u16,
    bfOffBits: u32,
}

#[allow(non_snake_case)]
impl BmpFileHeader {
    fn parse_file_header(buf: &[u8]) -> std::io::Result<BmpFileHeader> {
        let mut cur = Cursor::new(buf);
        let mut bfType = [0_u8; 2];

        for cc in &mut bfType {
            *cc = cur.read_u8()?;
        }

        let bfSize = cur.read_u32::<LittleEndian>()?;
        let bfReserved1 = cur.read_u16::<LittleEndian>()?;
        let bfReserved2 = cur.read_u16::<LittleEndian>()?;
        let bfOffBits = cur.read_u32::<LittleEndian>()?;

        Ok(
            BmpFileHeader {
                bfType,
                bfSize,
                bfReserved1,
                bfReserved2,
                bfOffBits,
            }
        )
    }
}
