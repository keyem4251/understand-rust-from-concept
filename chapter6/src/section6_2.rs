use std::net::TcpListener;

use crate::line_read::get_lines;

pub fn fn_6_2() {
    list_6_1().unwrap();
}

fn list_6_1() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:3210")?;

    for stream in listener.incoming() {
        let lines_vec = get_lines(stream?);
        println!("{:?}", lines_vec);
    }

    Ok(())
}
