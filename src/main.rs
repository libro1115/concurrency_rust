use std::{thread, time::Duration, sync::{mpsc, Mutex, Arc}};
fn main() {
    //concurrncy_test_normal();
    //concucurrncy_test_normal();
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

    for handle in handles{
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());
}

fn concurrncy_test_normal()
{
    let handle = thread::spawn(||{
        for i in 1..10{
            println!("number {} from the spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..5{
        println!("number {} from the main thread", i);
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap();
}

fn concurrncy_test_message()
{
    let (tx,rx) = mpsc::channel();
    thread::spawn(move ||{
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
            ];
        for val in vals{
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    for receive in rx{
        println!("Got: {}",receive);
    }
}