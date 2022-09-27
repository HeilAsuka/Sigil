use std::net::SocketAddr;

use anyhow::Result;
use clap::Parser;

use tokio::io::copy_bidirectional;
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs, UdpSocket};

/// A TCP forwarding tool
///
/// To proxy all tcp traffic on localhost port 8080 to remote host 10.0.1.100 port 5900 you can run:
///
/// sigil -l 127.0.0.1:8080 -r 10.0.1.100:5900
#[derive(Parser)]
#[clap(name = "sigil")]
#[clap(author = "HeilAsuka <heilasuka911@gmail.com>")]
#[clap(version = "0.1.0")]
struct Args {
    /// Specify the listening address i.e. 127.0.0.1:8080
    #[clap(short, long)]
    local: String,
    /// Specify the remote address to proxy to i.e. 10.0.1.100:5900
    #[clap(short, long)]
    remote: String,
}

fn parse_cli() -> Args {
    Args::try_parse().unwrap_or_else(|e: clap::Error| {
        println!("{}", e);
        e.exit();
    })
}
#[tokio::main]
async fn main() -> Result<()> {
    let args = parse_cli();
    let remote_addr = args.remote.to_owned();
    let local_addr = args.local.to_owned();
    let _a = tokio::join!(
        start_tcp_forwarding(&local_addr, &remote_addr),
    );
    Ok(())
}

async fn start_tcp_forwarding(local_addr: &str, remote_addr: &str) -> Result<()> {
    let locallistener = TcpListener::bind(local_addr).await?;
    loop {
        let (incoming, _) = locallistener.accept().await?;
        let remote_addr = remote_addr.to_owned();
        tokio::spawn(async move {
            if let Err(e) = tcp_forwarding(incoming, remote_addr).await {
                println!("{}", e);
            };
        });
    }
}

async fn tcp_forwarding<S: ToSocketAddrs>(mut incoming: TcpStream, remote_addr: S) -> Result<()> {
    let mut outgoing = TcpStream::connect(remote_addr).await?;
    copy_bidirectional(&mut incoming, &mut outgoing).await?;
    Ok(())
}
