use tokio::time::Duration;

async fn sleep_then_print(timer: i32) -> i32 {
    println!("Start timer {}.", timer);

    tokio::time::sleep(Duration::from_secs(3)).await;
    //                                            ^ execution can be paused here

    println!("Timer {} done.", timer);

    timer
}

#[tokio::main]
async fn main() {
    // The join! macro lets you run multiple things concurrently.
    let mut tasks = Vec::new();

    for i in 1..100 {
        tasks.push(tokio::spawn(async move { sleep_then_print(i).await }));
    }

    for t in tasks {
        t.await.unwrap();
    }

    //let res = tokio::join!(tasks);

    //println!("{:#?}", res);
}
