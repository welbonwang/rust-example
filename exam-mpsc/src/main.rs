use std::sync::mpsc;
use std::thread;

fn main() {
    // 创建一个mpsc通道
    let (sender, receiver) = mpsc::channel();

    // 创建一个新线程，发送消息到通道
    thread::spawn(move || {
        let message = String::from("Hello from sender!");
        sender.send(message).unwrap();
    });

    // 接收来自通道的消息
    let received = receiver.recv().unwrap();
    println!("Received: {}", received);
}