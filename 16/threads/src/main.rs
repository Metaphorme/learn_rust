use std::thread;
use std::time::Duration;

fn main() {
    println!("1");

    // 子线程立即开始执行
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    println!("2");
    thread::sleep(Duration::from_secs(1));

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    println!("3");

    // 仅确保子线程结束，具有阻塞的功能。
    handle.join().unwrap();

    println!("4");
}
