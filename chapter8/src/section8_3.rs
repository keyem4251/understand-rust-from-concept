pub fn section8_3() {
    fn8_1();
    fn8_2();
    fn8_3();
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
