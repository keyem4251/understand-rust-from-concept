use std::error::Error;
use std::io::Write;
use std::io;
use tokio::io::AsyncWriteExt;
use std::net::{TcpStream, TcpListener};

pub fn section8_3() {
    fn8_1();
    fn8_2();
    fn8_3();
    fn8_4();
}

async fn sum_func(n: usize) -> usize {
    let ans = (1..=n).into_iter().sum::<usize>();
    print!("{}", ans);
    ans
}

fn fn8_1() {
    let fut = sum_func(100);
    
    let ls = tokio::task::LocalSet::new();
    let rt = tokio::runtime::Builder::new_multi_thread().enable_all().build().unwrap();

    ls.block_on(&rt, fut);
}

fn fn8_2() {
    let ls = tokio::task::LocalSet::new();
    let rt = tokio::runtime::Builder::new_multi_thread().enable_all().build().unwrap();

    ls.block_on(&rt, async {
        sum_func(100).await;
        sum_func(20).await;
    });
}

async fn print_number(n: usize) -> usize {
    for ii in 0..2 {
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        println!("{} from {}", ii, n);
    }
    n
}

#[tokio::main]
async fn fn8_3() {
    let mut handler_vec = Vec::new();
    (1..=3).for_each(|ii| {
        handler_vec.push(tokio::spawn(print_number(ii)));
    });

    let ret = futures::future::join_all(handler_vec).await;
    let retval = ret.into_iter().map(|r| r.unwrap()).collect::<Vec<usize>>();
    println!("{:?}", retval);
}

async fn echo_process(stream: &mut TcpStream) -> Result<(), Box<dyn Error>> {
    let mut buf = Vec::with_capacity(1024);
    let mut input_prompt = true;

    loop {
        if input_prompt {
            stream.write_all("input =>".as_bytes()).await?;
        }
        let read = stream.try_read_buf(&mut buf);
        match read {
            Ok(0) => break,
            Ok(n) => {
                stream.write_all("output =>".as_bytes()).await?;
                stream.write_all(&buf[..n]).await?;
                input_prompt = true;
            },
            Err(ref e) if e.kind() == io::ErrorKind:WouldBlock => {
                input_prompt = false;
                continue;
            },
            Err(e) => {
                return Err(e.into());
            }
        }
        buf.clear();
    }
    Ok(())
}

#[tokio::main]
async fn fn8_4() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        panic!("port is not specified");
    }
    let port: usize = args[1].parse().expect("Failed to get the port number");
    let addr = format!("localhost:{}", port);
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Listening to the port {}", port);

    loop {
        let (mut stream, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            echo_process(&mut stream).await.unwrap();
        });
    }
}
