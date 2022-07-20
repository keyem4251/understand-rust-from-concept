pub fn section8_3() {
    fn8_1();
    fn8_2();
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
