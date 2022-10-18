use clap::Parser;
use smoltcp::phy::{TunTapInterface, Medium};
use url::Url;

mod dns;
mod ethernet;
mod http;

#[derive(Parser, Debug)]
#[command(author, version, about = "Manual get for network requests.", long_about = None)]
struct Args {
    #[arg(short, long, required = true)]
    url: String,

    #[arg(short, long, required = true)]
    tap_device: String,

    #[arg(short, long, default_value = "1.1.1.1")]
    dns_server: String,
}

fn main() {
    let args = Args::parse();

    let url = Url::parse(&args.url).expect("error: unable to parse URL");

    if url.scheme() != "http" {
        eprintln!("error: only HTTP protocol supported");
        return;
    }

    let tap = TunTapInterface::new(&args.tap_device, Medium::Ethernet)
        .expect("error: unable to use tap-device as a network interface");

    let domain_name = url.host_str().expect("domain name rqeuired");

    let _dns_server: std::net::Ipv4Addr = args.dns_server.parse()
        .expect("error: unable to parse dns server as an IPv4 address");
    
    let addr = dns::resolve(&args.dns_server, domain_name).unwrap().unwrap();

    let mac = ethernet::MacAddress::new().into();

    http::get(tap, mac, addr, url).unwrap();
}
