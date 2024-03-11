use std::{
    io::Error,
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
};

fn main() -> Result<(), Error> {
    // let arr = Arc::new(Mutex::new(vec![1, 2, 3, 4, 5, 6, 7, 8]));

    // works โดยจะเก็บเป็นตัวเลขและให้ thread ลดเลขลง 1 และเพิ่นเข้ามาที่ works
    // หากเป็น 0 ไม่ต้องเพิ่ม แสดงว่างานจบแล้ว
    let works = Arc::new(Mutex::new(vec![1, 2, 3]));
    // loop ดึงค่าจาก works และใส่ลงไปใน thread
    loop {
        // ใช้สำหรับเก็บ thread โดยจะยังไม่รันทันที่ที่สร้าง
        let mut tasks: Vec<JoinHandle<()>> = vec![];
        let work = works.clone();

        // ดึงค่าออกมาจาก works
        for v in work.lock().unwrap().pop() {
            let work_th = works.clone();
            // สร้าง thread 
            let a = thread::spawn(move || {
                println!("v: {}", v);
                if v > 0 {
                    work_th.lock().unwrap().push(v - 1);
                }
            });
            // เก็บ thread ลงใน tasks เพื่อจะสั่งรันในอนาคต
            tasks.push(a);
        }
        println!("task on");
        // run thread
        for task in tasks {
            task.join().unwrap()
        }
        println!("task end");
        if works.clone().lock().unwrap().len() == 0 {
            break;
        }
    }
    // println!("{:?}", arr.lock().unwrap());
    Ok(())
}
