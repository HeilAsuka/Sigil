use anyhow::{Result, Ok};
use structopt::StructOpt;
use tokio::{
    io::AsyncWriteExt,
    net::{TcpListener, TcpStream, ToSocketAddrs},
};

#[derive(StructOpt, Debug)]
#[structopt(name = "Sigil", about = "A TCP forwarding tool.")]
/// A TCP forwarding tool
///
/// To proxy all tcp traffic on localhost port 8080 to remote host 10.0.1.100 port 5900 you can run:
///
/// sigil -l 127.0.0.1:8080 -r 10.0.1.100:5900
struct Cli {
    #[structopt(short = "l", long = "local", help = "Local address and port")]
    /// Specify the listening address i.e. 127.0.0.1:8080
    local: String,
    #[structopt(short = "r", long = "remote", help = "Remote address and port")]
    /// Specify the remote address to proxy to i.e. 10.0.1.100:5900
    remote: String,
    // #[structopt(
    //     short = "t",
    //     long = "threads",
    //     help = "Number of threads to use",
    //     default_value = "4"
    // )]
    // /// Specify the number of threads to use, default value is 4
    // threads: usize,
}

fn parse_cli() -> Cli {
    Cli::from_args_safe().unwrap_or_else(|e| {
        println!("{}", e.message);
        e.exit();
    })
}
#[tokio::main]
async fn main() -> Result<()> {
    let args = parse_cli();
    let remote_addr = args.remote.to_owned();
    let local_addr = args.local.to_owned();
    start_tcp_forwarding(&local_addr, &remote_addr).await?;
    Ok(())
}

async fn start_tcp_forwarding(localaddr: &str, remote_addr: &str) -> Result<()> {
    let locallistener = TcpListener::bind(localaddr).await?;
    loop {
        let (incoming, _) = locallistener.accept().await?;
        let remote_addr = remote_addr.to_owned();
        tokio::spawn(async move{
            if let Err(e) = tcp_forwarding(incoming, remote_addr).await {
                println!("{}", e);
            };
        });
    }
}

async fn tcp_forwarding<S: ToSocketAddrs>(mut incoming: TcpStream, remote_addr: S) -> Result<()> {
    let mut outgoing = TcpStream::connect(remote_addr).await?;
    let (mut ri, mut wi) = incoming.split();
    let (mut ro, mut wo) = outgoing.split();
    let incoming_to_outgoing = async {
        tokio::io::copy(&mut ri, &mut wo).await?;
        wo.shutdown().await?;
        Ok(())
    };
    let outgoing_to_incoming = async {
        tokio::io::copy(&mut ro, &mut wi).await?;
        wi.shutdown().await?;
        Ok(())
    };
    tokio::try_join!(incoming_to_outgoing, outgoing_to_incoming)?;
    Ok(())
}
