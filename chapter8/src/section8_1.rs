use std::thread::spawn;
use std::sync::{Arc, Mutex};

const N_MAX: usize = 1000;
const N_THREAD: usize = 4;

const N_ELEM_PER_THRD: usize = N_MAX / N_THREAD;
const RESIDUAL: usize = N_MAX % N_THREAD;

pub fn section8_1() {
    list8_1();
    list8_2();
    list8_3().unwrap();
    list8_4();
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

fn list8_2() {
    let mut v_threads = Vec::new();
    let hello = String::from("Hello");
    for i in 0..10 {
        let hello_cloned = hello.clone();
        let thread = spawn(move || println!("{}: {}", i, hello_cloned));
        v_threads.push(thread);
    }
    let x: Vec<()> = v_threads.into_iter().map(|th| th.join().unwrap()).collect();
    println!("{:?}", x);
}

fn list8_3() -> std::thread::Result<()> {
    if RESIDUAL != 0 {
        panic!("invalid combination of N_MAX and N_THREAD");
    }

    let mut thrd = Vec::new();
    let v = std::sync::Arc::new((1..=N_MAX).collect::<Vec<usize>>());

    // 1..=N_MAXをN_THREADに分割してそれぞれの和をスレッドで計算
    for ii in 0..N_THREAD {
        let ist = ii * N_ELEM_PER_THRD;
        let ien = ist + N_ELEM_PER_THRD;
        let vv = std::sync::Arc::clone(&v);
        let th = spawn(move || vv[ist..ien].iter().sum::<usize>());
        thrd.push(th);
    }

    // 各スレッドで計算した値を集めて、その和を取り、全体の和を求める
    let ans: usize = thrd.into_iter().map(|r| r.join().unwrap()).sum::<usize>();
    println!("{}", ans);

    assert_eq!(ans, N_MAX * (N_MAX + 1) / 2);
    Ok(())
}

fn list8_4() {
    let data = Arc::new(Mutex::new(Vec::new()));
    let added = vec![1, 2, 3, 4];

    let mut thrd = Vec::new();
    for aa in added {
        let data = Arc::clone(&data);
        let th = spawn(move || {
            let mut data = data.lock().unwrap();
            data.push(aa)
        });
        thrd.push(th);
    }

    thrd.into_iter().for_each(|th| th.join().unwrap());
    let x = data.lock().unwrap();

    println!("{:?}", x);
}
