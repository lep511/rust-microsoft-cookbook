use glommio::{defer, timer::TimerActionOnce, LocalExecutorBuilder, Placement};
use std::time::Duration;

fn main() {
    defer! {
        println!("Executor is done!");
    }

    LocalExecutorBuilder::new(Placement::Fixed(6))
    .spawn(|| async move {
        println!("This will print from 0");
    })
    .unwrap();

    let handle = LocalExecutorBuilder::default()
        .spawn(|| async move {
            defer! {
                println!("This will print after the timer");
            }

            println!("This will print first");
            let task = TimerActionOnce::do_in(Duration::from_secs(1), async move {
                println!("This will print after one second");
            });
            task.join().await;
        })
        .unwrap();
    handle.join().unwrap();
}
