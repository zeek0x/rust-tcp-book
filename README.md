# rust-tcp-book

Book: https://techbookfest.org/product/6562563816947712

# 3.5.9 動作確認

netcatでサーバを起動する。

```console
$ sudo ip netns exec host2 nc -l 10.0.1.1 40000
```

クラインアントを実行する。

```console
$  sudo ip netns exec host1 ./target/debug/examples/echoclient 10.0.1.1 40000
```

クライアント側の標準出力は以下の通りになる。

```console
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

# 3.6.5 動作確認

サーバを起動する。

```console
$ sudo ip netns exec host1 ./target/debug/examples/echoserver 10.0.0.1 40000
```

クライアントにはnetcatを用いる。

```console
$ sudo ip netns exec host2 nc 10.0.0.1 40000
```

サーバ側の標準出力は以下の通りになる。

```console
[src/tcp.rs:186] "begin recv thread" = "begin recv thread"
[examples/echoserver.rs:16] "listening..." = "listening..."
[src/tcp.rs:80] "listen handler" = "listen handler"
[src/socket.rs:218] "sent" = "sent"
[src/socket.rs:218] &tcp_packet =
        src: 40000
        dst: 51814
        flag: SYN ACK
        payload_len: 0
[src/tcp.rs:109] "status: listen -> " = "status: listen -> "
[src/tcp.rs:109] &connection_socket.status = SYNRCVD
[src/tcp.rs:122] "synrcvd handler" = "synrcvd handler"
[src/tcp.rs:132] "status: synrcvd ->" = "status: synrcvd ->"
[src/tcp.rs:132] &socket.status = ESTABLISHED
[src/tcp.rs:60] &event = Some(
    TCPEvent {
        sock_id: SockID(
            10.0.0.1,
            0.0.0.0,
            40000,
            0,
        ),
        kind: ConnectionCompleted,
    },
)
[examples/echoserver.rs:19] "accepted!" = "accepted!"
[examples/echoserver.rs:19] connected_socket.1 = 10.0.1.1
[examples/echoserver.rs:19] connected_socket.3 = 51814
```

次に、ToyTCp同士でコネクションを構築する。

```console
$ sudo ip netns exec host1 ./target/debug/examples/echoserver 10.0.0.1 40000
[examples/echoserver.rs:16] "listening..." = "listening..."
[src/tcp.rs:186] "begin recv thread" = "begin recv thread"
[src/tcp.rs:80] "listen handler" = "listen handler"
[src/tcp.rs:80] "listen handler" = "listen handler"
[src/tcp.rs:80] "listen handler" = "listen handler"
[src/tcp.rs:80] "listen handler" = "listen handler"
[src/tcp.rs:80] "listen handler" = "listen handler"
[src/socket.rs:218] "sent" = "sent"
[src/socket.rs:218] &tcp_packet =
        src: 40000
        dst: 47006
        flag: SYN ACK
        payload_len: 0
[src/tcp.rs:109] "status: listen -> " = "status: listen -> "
[src/tcp.rs:109] &connection_socket.status = SYNRCVD
[src/tcp.rs:122] "synrcvd handler" = "synrcvd handler"
[src/tcp.rs:132] "status: synrcvd ->" = "status: synrcvd ->"
[src/tcp.rs:132] &socket.status = ESTABLISHED
[src/tcp.rs:60] &event = Some(
    TCPEvent {
        sock_id: SockID(
            10.0.0.1,
            0.0.0.0,
            40000,
            0,
        ),
        kind: ConnectionCompleted,
    },
)
[examples/echoserver.rs:19] "accepted!" = "accepted!"
[examples/echoserver.rs:19] connected_socket.1 = 10.0.1.1
[examples/echoserver.rs:19] connected_socket.3 = 47006
```

```console
$ sudo ip netns exec host2 ./target/debug/examples/echoclient 10.0.0.1 40000
[src/tcp.rs:186] "begin recv thread" = "begin recv thread"
[src/tcp.rs:351] "source addr" = "source addr"
[src/tcp.rs:351] ip = "10.0.1.1"
[src/socket.rs:218] "sent" = "sent"
[src/socket.rs:218] &tcp_packet =
        src: 47006
        dst: 40000
        flag: SYN
        payload_len: 0
[src/tcp.rs:145] "synsent handler" = "synsent handler"
[src/socket.rs:218] "sent" = "sent"
[src/socket.rs:218] &tcp_packet =
        src: 47006
        dst: 40000
        flag: ACK
        payload_len: 0
[src/tcp.rs:166] "status: synsend ->" = "status: synsend ->"
[src/tcp.rs:166] &socket.status = ESTABLISHED
[src/tcp.rs:60] &event = Some(
    TCPEvent {
        sock_id: SockID(
            10.0.1.1,
            10.0.0.1,
            47006,
            40000,
        ),
        kind: ConnectionCompleted,
    },
)
```
