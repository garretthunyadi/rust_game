extern crate url;
// use domain_info::{Domain, Scanner};
use futures::future::join_all;
use std::sync::mpsc;
// use url::Url;
use std::sync::mpsc::{Receiver, Sender};
// use std::thread;
// use std::io::{stderr, Write};
use std::{thread, time};

// #[tokio::main]
// pub async fn main() {
//     println!("wappalyzer_crate");
//     let url = Url::parse(&String::from("http://google.com")).unwrap();
//     let res = wappalyzer::scan(url).await;
//     println!("{:?}", res);

//     println!("domain_info_crate");
//     let res = Domain::from("google.com").unwrap().scan();
//     println!("{:?}", res);
// }

// async fn send_recv() {
//     // const BUFFER_SIZE: usize = 10;
//     let (mut tx, mut rx) = mpsc::channel::<i32>();

//     tx.send(1).await.unwrap();
//     tx.send(2).await.unwrap();
//     drop(tx);

//     // `StreamExt::next` is similar to `Iterator::next`, but returns a
//     // type that implements `Future<Output = Option<T>>`.
//     assert_eq!(Some(1), rx.next().await);
//     assert_eq!(Some(2), rx.next().await);
//     assert_eq!(None, rx.next().await);
// }

// static NTHREADS: i32 = 3;

// fn main() {
//     // Channels have two endpoints: the `Sender<T>` and the `Receiver<T>`,
//     // where `T` is the type of the message to be transferred
//     // (type annotation is superfluous)
//     let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();
//     let mut children = Vec::new();

//     for id in 0..NTHREADS {
//         // The sender endpoint can be copied
//         let thread_tx = tx.clone();

//         // Each thread will send its id via the channel
//         let child = thread::spawn(move || {
//             // The thread takes ownership over `thread_tx`
//             // Each thread queues a message in the channel
//             thread_tx.send(id).unwrap();

//             // Sending is a non-blocking operation, the thread will continue
//             // immediately after sending its message
//             println!("thread {} finished", id);
//         });

//         children.push(child);
//     }

//     // Here, all the messages are collected
//     let mut ids = Vec::with_capacity(NTHREADS as usize);
//     for _ in 0..NTHREADS {
//         // The `recv` method picks a message from the channel
//         // `recv` will block the current thread if there are no messages available
//         ids.push(rx.recv());
//     }
//     // Wait for the threads to complete any remaining work
//     for child in children {
//         child.join().expect("oops! the child thread panicked");
//     }

//     // Show the order in which the messages were sent
//     println!("{:?}", ids);
// }

//
//
//

async fn start_worker(id: i32, tx: Sender<i32>) {
    // eprint!("start:{}", id);
    // let _ = stderr().flush();

    thread::sleep(time::Duration::from_millis((100 * id) as u64));

    tx.send(id).unwrap();
    // println!("worker {} finished", id);
}

async fn listen(rx: Receiver<i32>) {
    for _ in 0..NWORKERS {
        // The `recv` method picks a message from the channel
        // `recv` will block the current thread if there are no messages available
        println!("REC'D: {}", rx.recv().unwrap());
    }
}

static NWORKERS: i32 = 10;

#[tokio::main]
pub async fn main() {
    // println!("HERE 1");

    // Channels have two endpoints: the `Sender<T>` and the `Receiver<T>`,
    // where `T` is the type of the message to be transferred
    // (type annotation is superfluous)
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();
    let mut children = Vec::new();

    for id in 0..NWORKERS {
        // The sender endpoint can be copied
        let worker_tx = tx.clone();
        let f = start_worker(id, worker_tx);
        // println!("HERE 2.{}", id);

        // Each thread will send its id via the channel
        // let child = thread::spawn(move || {
        //     // The thread takes ownership over `thread_tx`
        //     // Each thread queues a message in the channel
        //     thread_tx.send(id).unwrap();

        //     // Sending is a non-blocking operation, the thread will continue
        //     // immediately after sending its message
        //     println!("thread {} finished", id);
        // });

        children.push(f);
    }

    let listen_f = listen(rx);
    // children.push(listen_f);
    let sender_fs = join_all(children);
    let res = futures::join!(sender_fs, listen_f); // NOTE: Order matters!

    // let res = listen(rx).await;

    // Here, all the messages are collected
    // let mut ids = Vec::with_capacity(NWORKERS as usize);
    // for _ in 0..NWORKERS {
    //     // The `recv` method picks a message from the channel
    //     // `recv` will block the current thread if there are no messages available
    //     ids.push(rx.recv());
    // }
    // // Wait for the threads to complete any remaining work
    // for child in children {
    //     // child.join().expect("oops! the child thread panicked");
    // }
    // println!("HERE 4");

    println!("{:?}", res);
}

// let futures = domains
// .into_iter()
// .map(|d| async move { Domain(d.0).scan() })
// .collect::<Vec<_>>();
// let results = join_all(futures).await;
// for res in results {
// if let Ok(output) = match res {
//     Ok(info) => serde_json::to_string(&info),
//     Err(err) => serde_json::to_string(&err),
// } {
//     println!("{}", output);
// }
// }
