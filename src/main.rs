use smol::future::zip;
use smol::io;
use smol::net::{AsyncToSocketAddrs, SocketAddr, TcpListener, TcpStream};
use structopt::StructOpt;

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
}

fn parse_cli() -> Cli {
    Cli::from_args_safe().unwrap_or_else(|e| {
        println!("{}", e.message);
        e.exit();
    })
}
fn set_thread_number<T: ToString>(thread_num: T) {
    std::env::set_var("SMOL_THREADS", thread_num.to_string());
}
fn main() -> io::Result<()> {
    let args = parse_cli();
    set_thread_number(4);
    let remote_addr = args.remote.parse::<SocketAddr>().unwrap();
    smol::block_on(async {
        let tcp_listener: TcpListener = TcpListener::bind(args.local).await?;
        loop {
            match tcp_listener.accept().await {
                Ok((tcpstream, _)) => {
                    let tcp_forwarding = tcp_forwarding(tcpstream, remote_addr);
                    smol::spawn(tcp_forwarding).detach();
                }
                Err(e) => println!("{:?}", e),
            }
        }
    })
}

async fn tcp_forwarding<S: AsyncToSocketAddrs>(
    tcpstream: TcpStream,
    remote_addr: S,
) -> io::Result<()> {
    tcpstream.set_nodelay(true)?;
    let remote_addr: SocketAddr = remote_addr.to_socket_addrs().await.unwrap().next().unwrap();
    let outbound = TcpStream::connect(remote_addr).await?;
    outbound.set_nodelay(true)?;
    let (mut ri, mut wi): (io::ReadHalf<TcpStream>, io::WriteHalf<TcpStream>) =
        io::split(tcpstream);
    let (mut ro, mut wo): (io::ReadHalf<TcpStream>, io::WriteHalf<TcpStream>) = io::split(outbound);
    let inbound_to_outbound = io::copy(&mut ri, &mut wo);
    let outbound_to_inbound = io::copy(&mut ro, &mut wi);
    let (inbound_to_outbound, outbound_to_inbound) =
        zip(inbound_to_outbound, outbound_to_inbound).await;
    match (inbound_to_outbound, outbound_to_inbound) {
        (Ok(_), Ok(_)) => Ok(()),
        (Err(e), _) => Err(e),
        (_, Err(e)) => Err(e),
    }
}
