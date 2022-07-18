use std::thread::spawn;

pub fn section8_1() {
    list8_1();
}

fn list8_1() {
    let mut v_threads = Vec::new();
    for i in 0..10 {
        let thread = spawn(move || println!("{}", i));
        v_threads.push(thread);
    }
    let x: Vec<()> = v_threads.into_iter().map(|th| th.join().unwrap()).collect();
    println!("{:?}", x);
}
