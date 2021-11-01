use std::error::Error;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::{env, str, thread};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let addr = &args[1];
    echo_server(addr)?;
    Ok(())
}

fn echo_server(address: &str) -> Result<(), Box<dyn Error>> {
    // リッスンモードのソケットを指定のアドレスで作成する。
    let listener = TcpListener::bind(address)?;
    loop {
        // メインスレッドをブロックし、クライアントからのコネクション確立要求を待機する。
        // TCPのスリーウェイハンドシェイクを通してコネクションが確立されたらブロックを解除する。
        let (mut stream, _) = listener.accept()?;
        // 新たにスレッドが生成され、メインスレッドはループに戻る。
        thread::spawn(move || {
            let mut buffer = [0u8; 1024];
            loop {
                // スレッドをブロックし、データの受信を待機する。
                // 受信を行うとブロックを解除し、確認応答をクライアントに送信する。
                let nbytes = stream.read(&mut buffer).unwrap();
                if nbytes == 0 {
                    return;
                }
                print!("{}", str::from_utf8(&buffer[..nbytes]).unwrap());
                // クライアントにデータを送信する。
                // クライアントからは確認応答を受信する。
                stream.write_all(&buffer[..nbytes]).unwrap();
            }
        });
    }
}

