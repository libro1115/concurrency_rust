use std::{thread, time::Duration, sync::mpsc};
fn main() {
    //concurrncy_test_normal();
    
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