use std::error::Error;
use std::fs::read;
use std::sync::Arc;
use std::time::Duration;
use openssl::pkey::PKey;
use openssl::x509::X509;
use serde_json::{from_str, json, Value};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::sleep;
use tokio_rustls::rustls::pki_types::{CertificateDer, DnsName, PrivateKeyDer, PrivatePkcs8KeyDer, ServerName};
use tokio_rustls::rustls::{ClientConfig, RootCertStore};
use tokio_rustls::TlsConnector;

const KEY_PATH: &str = "/etc/brski/continuous_assurance/";

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut root_store = RootCertStore::empty();
    root_store.add(CertificateDer::from(
        X509::from_pem(read(format!("{}/ca.crt", KEY_PATH))?.as_slice())?.to_der()?
    ))?;

    let config = ClientConfig::builder()
        .with_root_certificates(root_store)
        .with_client_auth_cert(
            vec![CertificateDer::from(
                X509::from_pem(read(format!("{}/router.crt", KEY_PATH))?.as_slice())?.to_der()?
            )],
            PrivateKeyDer::from(PrivatePkcs8KeyDer::from(
                PKey::private_key_from_pem(read(format!("{}/router.key", KEY_PATH))?.as_slice())?.private_key_to_pkcs8()?
            ))
        )?;

    let address;
    let port;
    'outer: loop {
        let output = std::process::Command::new("avahi-browse").args(["-r", "_brski._tcp", "-t", "-p"]).output()?;
        let output = String::from_utf8(output.stdout)?;
        for line in output.split("\n") {
            let service: Vec<&str> = line.split(";").collect();
            if service[0] == "=" && service[2] == "IPv4" && service[3] == "brski-registrar-CA-monitor" {
                address = String::from(service[7]);
                port = String::from(service[8]);
                break 'outer;
            }
        }
        println!("Service not found, retrying in 10 seconds");
        sleep(Duration::from_secs(10)).await;
    }

    let server_name = ServerName::DnsName(DnsName::try_from("registrar")?);
    let connector = TlsConnector::from(Arc::new(config.clone()));
    let stream = TcpStream::connect(format!("{}:{}", address, port)).await?;
    let mut stream = connector.connect(server_name, stream).await?;
    println!("Connected");

    loop {
        let mut received = Vec::new();
        loop {
            let mut buf = [0u8; 64];
            let length = stream.read(&mut buf).await.expect("Error receiving message from router");
            received.append(&mut buf[..length].to_vec());
            if buf[length-1] == 0u8 {
                received.pop();
                break;
            }
        }
        let json = from_str::<Value>(String::from_utf8(received)?.as_str())?;
        println!("Command received: {}", json);
        let mut args = Vec::new();
        for item in json["Revoke"].as_array().expect("Error parsing arguments") {
            if let Some(str) = item.as_str() {
                args.push(str);
            }
        }
        let output = std::process::Command::new("/etc/hostapd/CA/local_revoke_serial_multiple_args.sh")
            .args(args).output().expect("Error calling LocalRevoke");
        let message = [json!({
                "Stdout": String::from_utf8(output.stdout).expect("Error parsing local revoke output"),
                "Stderr": String::from_utf8(output.stderr).expect("Error parsing local revoke output")
            }).to_string().as_bytes(),
            [b'\0'].as_slice()].concat();
        stream.write_all(message.as_slice()).await.expect("Error sending message to registrar");
    }
}