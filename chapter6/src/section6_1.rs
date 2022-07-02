use std::fs::File;
use std::io::Read;

const BUFSIZE: usize = 1024;

pub fn fn_6_1() {
    match list_6_1() {
        Ok(ok) => println!("OK: {:?}", ok),
        Err(err) => println!("Err: {:?}", err),
    };
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
