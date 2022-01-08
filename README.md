# rust-tcp-book

Book: https://techbookfest.org/product/6562563816947712

# 3.5.9 動作確認

```console
$ sudo ip netns exec host2 nc -l 10.0.1.1 40000
```

```console
$  sudo ip netns exec host1 ./target/debug/examples/echoclient 10.0.1.1 40000
[examples/echoclient.rs:9] addr = 10.0.1.1
[examples/echoclient.rs:9] port = 40000
[src/tcp.rs:116] "begin recv thread" = "begin recv thread"
[src/tcp.rs:248] "source addr" = "source addr"
[src/tcp.rs:248] ip = "10.0.0.1"
[src/socket.rs:211] "sent" = "sent"
[src/socket.rs:211] &tcp_packet =
        src: 54220
        dst: 40000
        flag: SYN
        payload_len: 0
[src/tcp.rs:75] "synsent handler" = "synsent handler"
[src/socket.rs:211] "sent" = "sent"
[src/socket.rs:211] &tcp_packet =
        src: 54220
        dst: 40000
        flag: ACK
        payload_len: 0
[src/tcp.rs:96] "status: synsend ->" = "status: synsend ->"
[src/tcp.rs:96] &socket.status = ESTABLISHED
[src/tcp.rs:60] &event = Some(
    TCPEvent {
        sock_id: SockID(
            10.0.0.1,
            10.0.1.1,
            54220,
            40000,
        ),
        kind: ConnectionCompleted,
    },
)
```
