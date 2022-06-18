[![Build and test](https://github.com/HeilAsuka/Sigil/actions/workflows/build-and-test.yaml/badge.svg)](https://github.com/HeilAsuka/Sigil/actions/workflows/build-and-test.yaml)

# Sigil

A Port forwarding tool   
To proxy all tcp or udp traffic on localhost port 8080 to remote host 10.0.1.100 port 5900 you can run:   
```sigil -l 127.0.0.1:8080 -r 10.0.1.100:5900```   
This crate use [tokio](https://github.com/tokio-rs/tokio) as async runtime.

# TODO

-[x] support UDP
