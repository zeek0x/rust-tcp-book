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

## 3.7.3 動作確認

サーバにはnetcatを用いる。

```console
$ sudo ip netns exec host2 nc -l 10.0.1.1 40000
```

クライアントを実行する。

```console
$ sudo ip netns exec host1 ./target/debug/examples/echoclient 10.0.1.1 40000
...
[src/tcp.rs:166] "status: synsend ->" = "status: synsend ->"
[src/tcp.rs:166] &socket.status = ESTABLISHED
[src/tcp.rs:60] &event = Some(
    TCPEvent {
        sock_id: SockID(
            10.0.0.1,
            10.0.1.1,
            47013,
            40000,
        ),
        kind: ConnectionCompleted,
    },
)
hello
[src/socket.rs:218] "sent" = "sent"
[src/socket.rs:218] &tcp_packet =
        src: 47013
        dst: 40000
        flag: ACK PSH
        payload_len: 6
[src/tcp.rs:247] "not implemented state" = "not implemented state"
hoge
[src/socket.rs:218] "sent" = "sent"
[src/socket.rs:218] &tcp_packet =
        src: 47013
        dst: 40000
        flag: ACK PSH
        payload_len: 5
[src/tcp.rs:247] "not implemented state" = "not implemented state"
```

netcat側に送信した文字列が出力されるはずだが、何も出力されない。

```console
# Expected
hello
hoge

# Actual
```

tcpdumpのログ

```console
$ sudo ip netns exec host1 tcpdump -l
tcpdump: verbose output suppressed, use -v or -vv for full protocol decode
listening on host1-veth1, link-type EN10MB (Ethernet), capture size 262144 bytes
15:00:17.183995 IP 10.0.1.1.40000 > 10.0.0.1.49039: Flags [F.], seq 4256132755, ack 2086109783, win 64240, length 0
15:00:18.556665 IP 10.0.0.1.43360 > 10.0.1.1.40000: Flags [S], seq 400946233, win 4380, length 0
15:00:18.556698 IP 10.0.1.1.40000 > 10.0.0.1.43360: Flags [S.], seq 1775177938, ack 400946234, win 64240, options [mss 1460], length 0
15:00:18.557123 IP 10.0.0.1.43360 > 10.0.1.1.40000: Flags [.], ack 1, win 4380, length 0
15:00:18.560197 IP 10.0.0.1.47013 > 10.0.1.1.40000: Flags [S], seq 653014473, win 4380, length 0
15:00:18.560219 IP 10.0.1.1.40000 > 10.0.0.1.47013: Flags [S.], seq 1906374430, ack 653014474, win 64240, options [mss 1460], length 0
15:00:18.560310 IP 10.0.0.1.47013 > 10.0.1.1.40000: Flags [.], ack 1, win 4380, length 0
15:00:23.511566 IP 10.0.0.1.47013 > 10.0.1.1.40000: Flags [P.], seq 1:7, ack 1, win 4380, length 6
15:00:23.511592 IP 10.0.1.1.40000 > 10.0.0.1.47013: Flags [.], ack 7, win 64234, length 0
15:00:23.840444 IP 10.0.1.1.40000 > 10.0.0.1.49039: Flags [F.], seq 0, ack 1, win 64240, length 0
15:00:25.177250 IP 10.0.0.1.47013 > 10.0.1.1.40000: Flags [P.], seq 7:12, ack 1, win 4380, length 5
15:00:25.177278 IP 10.0.1.1.40000 > 10.0.0.1.47013: Flags [.], ack 12, win 64229, length 0
```
