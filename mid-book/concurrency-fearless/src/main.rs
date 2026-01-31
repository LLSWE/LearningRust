// use std::thread;
// use std::time::Duration;
//
// fn main() {
//     let handle = thread::spawn(|| {
//         for i in 1..10 {
//             println!("hi number {i} from the spawned thread");
//             thread::sleep(Duration::from_millis(1));
//         }
//     });
//
//     for i in 1..5 {
//         println!("hi number {i} from the main thread");
//         thread::sleep(Duration::from_millis(1));
//     }
//
//     handle.join().unwrap();
// // }
//
// use std::sync::mpsc;
// use std::thread;
//
// fn main() {
//     let (tx, rx) = mpsc::channel();
//
//     thread::spawn(move || {
//         let val = String::from("hi");
//         tx.send(val).unwrap();
//     });
//
//     let received = rx.recv().unwrap();
//     println!("Got : {received} ");
// }
//
// use std::sync::Mutex;
//
// fn main() {
//     let m = Mutex::new(5);
//
//     {
//         let mut num = m.lock().unwrap();
//         *num = 6;
//     }
//     println!("m = {m:?}")
// }

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result : {}", *counter.lock().unwrap());
}
