use std::thread;
use std::time::Duration;

fn main() {
    let th1 = thread::spawn(|| {
        for i in 0..10_i8 {
            println!("Thread 1 {}", i);
            thread::sleep(Duration::from_millis(1));
        }
        return "***thread 1 success";
    });
    let th2 = thread::spawn(|| {
        for i in 100..110_i8 {
            println!("thread 2 {}", i);
            thread::sleep(Duration::from_millis(1));
        }
        return "***thread 2 success";
    });

    let re_th1: &str = match th1.join() {
        Ok(v) => v,
        Err(_) => "error",
    };
    let re_th2: &str = match th2.join() {
        Ok(v) => v,
        Err(_) => "error",
    };

    println!("th1 = {}", re_th1);
    println!("th2 = {}", re_th2);
}
