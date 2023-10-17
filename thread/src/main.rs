use std::thread;
use std::time::Duration;

fn main() {
    let th1 = thread::spawn(|| {
        let mut all: u8 = 0;
        for i in 0..10_i8 {
            all += i as u8;
            println!("Thread 1 {}", i);
            thread::sleep(Duration::from_millis(1));
        }
        return all;
    });
    let th2 = thread::spawn(|| {
        let mut all: u16 = 0;
        for i in 100..110_i8 {
            all += i as u16;
            println!("thread 2 {}", i);
            thread::sleep(Duration::from_millis(1));
        }
        return all;
    });

    let re_th1: u8 = match th1.join() {
        Ok(v) => v,
        Err(_) => 0,
    };
    let re_th2 = match th2.join() {
        Ok(v) => v,
        Err(_) => 0,
    };

    println!("th1 = {}", re_th1);
    println!("th2 = {}", re_th2);
    println!("th1 + th2 = {}", re_th1 as u16 + re_th2);
}
