[![Build and test](https://github.com/HeilAsuka/Sigil/actions/workflows/build-and-test.yaml/badge.svg)](https://github.com/HeilAsuka/Sigil/actions/workflows/build-and-test.yaml)

# Sigil

A TCP forwarding tool   
To proxy all tcp traffic on localhost port 8080 to remote host 10.0.1.100 port 5900 you can run:   
```sigil -l 127.0.0.1:8080 -r 10.0.1.100:5900```   
This crate use [smol](https://github.com/smol-rs/smol) as async runtime.

# TODO

support UDP
