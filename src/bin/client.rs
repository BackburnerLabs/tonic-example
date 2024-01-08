use anyhow::*;
use clap::Parser;
use tonic_example::{echo_client::EchoClient, EchoRequest};
use tonic::transport::{Certificate, Channel, ClientTlsConfig};

#[derive(Parser, Debug)]
struct Opt {
    /// Server to connect to
    #[clap(long, default_value = "https://localhost:3030")]
    server: String,
    /// Message to send
    message: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let opt = Opt::parse();

    let pem = std::fs::read_to_string("sample.pem")?;
    let ca = Certificate::from_pem(pem);

    let tls = ClientTlsConfig::new()
        .ca_certificate(ca)
        .domain_name("testserver.com");

    let channel = Channel::from_static("http://localhost:3030")
        .tls_config(tls)?
        .connect()
        .await?;

    let mut client = EchoClient::new(channel);

    let res = client
        .echo(EchoRequest {
            message: opt.message,
        })
        .await
        .context("Unable to send echo request")?;

    println!("{:?}", res);

    Ok(())
}
