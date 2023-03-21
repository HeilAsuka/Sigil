use anyhow::Result;
use clap::Parser;

use tokio::io::{copy_bidirectional, AsyncReadExt, AsyncWriteExt}; // import AsyncReadExt and AsyncWriteExt traits
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};

/// Command line arguments for Sigil.
#[derive(Parser)]
#[clap(name = "sigil")]
#[clap(author = "HeilAsuka <heilasuka911@gmail.com>")]
#[clap(version = "0.1.0")]
struct SigilArgs {
    /// Specify the listening address i.e. 127.0.0.1:8080
    #[clap(short, long)]
    listening_addr: String,
    /// Specify the remote address to proxy to i.e. 10.0.1.100:5900
    #[clap(short, long)]
    remote_addr: String,
}

/// Parse command line arguments.
fn parse_args() -> SigilArgs {
    SigilArgs::try_parse().unwrap_or_else(|e: clap::Error| {
        println!("{}", e);
        e.exit();
    })
}

/// The entry point for the Sigil application.
#[tokio::main]
async fn main() -> Result<()> {
    let args = parse_args();
    let remote_addr = args.remote_addr.to_owned();
    let local_addr = args.listening_addr.to_owned();
    let _a = tokio::join!(start_tcp_forwarding(&local_addr, &remote_addr),); // start the TCP forwarding server
    Ok(())
}

/// Start a TCP forwarding server.
async fn start_tcp_forwarding(listening_addr: &str, remote_addr: &str) -> Result<()> {
    let listener = TcpListener::bind(listening_addr).await?; // create a TCP listener
    loop {
        let (incoming, _) = listener.accept().await?; // accept a new TCP connection
        let remote_addr = remote_addr.to_owned();
        tokio::spawn(async move {
            if let Err(e) = tcp_forwarding(incoming, remote_addr).await { // start the TCP forwarding
                println!("{}", e);
            }
        });
    }
}

/// Perform TCP forwarding between a local client and a remote server.
async fn tcp_forwarding<S: ToSocketAddrs>(mut incoming: TcpStream, remote_addr: S) -> Result<()> {
    let mut outgoing = TcpStream::connect(remote_addr).await?; // connect to the remote server
    tokio::select! {
        result = copy_bidirectional(&mut incoming, &mut outgoing) => result?, // copy from incoming stream to outgoing stream
        result = copy_bidirectional(&mut outgoing, &mut incoming) => result?, // copy from outgoing stream to incoming stream
    }
    Ok(())
}
