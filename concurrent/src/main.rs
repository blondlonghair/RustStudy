use std::thread;
use std::time::Duration;
use std::sync::{mpsc, Mutex, Arc};
use std::rc::Rc;

fn main() {
    // let handle = thread::spawn(|| {
    //     for i in 1..10 {
    //         println!("새 스레드: {}", i);
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });

    // for i in 1..5 {
    //     println!("주 스레드: {}", i);
    //     thread::sleep(Duration::from_millis(1));
    // }
    
    // handle.join().unwrap();



    // let v = vec![1, 2, 3];

    // let handle = thread::spawn(move || {
    //     println!("벡터: {:?}", v);
    // });

    // handle.join().unwrap();



    // let (tx, rx) = mpsc::channel();

    // thread::spawn(move || {
    //     let val = String::from("안녕하세요");
    //     tx.send(val).unwrap();
    // });

    // let received = rx.recv().unwrap();
    // println!("수신: {}", received);

    // let (tx, rx) = mpsc::channel();

    // let tx1 = mpsc::Sender::clone(&tx);
    // thread::spawn(move || {
    //     let vals = vec![
    //         String::from("자식 스레드가"),
    //         String::from("안녕하세요"),
    //         String::from("라고"),
    //         String::from("인사합니다."),
    //     ];

    //     for val in vals {
    //         tx1.send(val).unwrap();
    //         thread::sleep(Duration::from_secs(1));
    //     }
    // });

    // thread::spawn(move || {
    //     let vals = vec![
    //         String::from("그리고"),
    //         String::from("더 많은"),
    //         String::from("메세지를"),
    //         String::from("보냅니다."),
    //     ];

    //     for val in vals {
    //         tx.send(val).unwrap();
    //         thread::sleep(Duration::from_secs(1));
    //     }
    // });

    // for received in rx {
    //     println!("수신: {}", received);
    // }



    // let m = Mutex::new(5);

    // {
    //     let mut num = m.lock().unwrap();
    //     *num = 6;
    // }

    // println!("m = {:?}", m);

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

    println!("결과: {}", *counter.lock().unwrap());
}
