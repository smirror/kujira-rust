use std::sync::mpsc;
use std::thread;
use std::time::Duration;

// 1秒おきにメッセージを送信する関数 --- (*1)
fn sleep_sender(name: &str, sender: mpsc::Sender<String>) {
    for i in 1..=5 {
        let msg = format!("{}: {}", name, i);
        sender.send(msg).unwrap(); // 送信
        thread::sleep(Duration::from_millis(1000));
    }
    sender.send("quit".to_string()).unwrap();
}

fn main() {
    // スレッド間通信用のチャンネルを用意 --- (*2)
    let (tx, rx) = mpsc::channel::<String>();

    // スレッド1を生成 --- (*3)
    let sender = tx.clone();
    thread::spawn(|| {
        sleep_sender("1", sender)
    });
    // スレッド2を生成
    let sender = tx.clone();
    thread::spawn(|| {
        sleep_sender("2", sender)
    });

    let sender = tx.clone();
    thread::spawn(|| {
        sleep_sender("3", sender)
    });

    let sender = tx.clone();
    thread::spawn(|| {
        sleep_sender("4", sender)
    });

    let sender = tx.clone();
    thread::spawn(|| {
        sleep_sender("5", sender)
    });
    let sender = tx.clone();
    thread::spawn(|| {
        sleep_sender("6", sender)
    });
    let sender = tx.clone();
    thread::spawn(|| {
        sleep_sender("7", sender)
    });
    let sender = tx.clone();
    thread::spawn(|| {
        sleep_sender("8", sender)
    });


    let sender = tx.clone();
    thread::spawn(|| {
        sleep_sender("9", sender)
    });
    let sender = tx.clone();
    thread::spawn(|| {
        sleep_sender("10", sender)
    });
    let sender = tx.clone();
    thread::spawn(|| {
        sleep_sender("11", sender)
    });
    let sender = tx.clone();
    thread::spawn(|| {
        sleep_sender("12", sender)
    });

    let sender = tx.clone();
    thread::spawn(|| {
        sleep_sender("13", sender)
    });
    let sender = tx.clone();
    thread::spawn(|| {
        sleep_sender("14", sender)
    });
    let sender = tx.clone();
    thread::spawn(|| {
        sleep_sender("15", sender)
    });

    // スレッドからのメッセージを繰り返し受ける --- (*4)
    loop {
        let buf = rx.recv().unwrap();
        println!("[受信] {}", buf);
        if buf == "quit" { break; }
    }
}
