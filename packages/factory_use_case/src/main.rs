use std::io::{Read, Write};
use std::process::Stdio;
use rand::{Rng, thread_rng};
use warp::Filter;
use warp::hyper::body::Bytes;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let args: Vec<_> = std::env::args().collect();
    let default = String::from(".");
    let path = args.get(1).unwrap_or(&default);

    let route = warp::any()
        .and(warp::filters::body::bytes())
        .map(|csr: Bytes| {
            std::fs::write("v3.ext", "basicConstraints=CA:FALSE").unwrap();
            let serial = thread_rng().gen_range(10000..=65535).to_string();
            let command = std::process::Command::new("openssl")
                .args(["x509", "-req", "-set_serial", &serial, "-CA", "mpr.crt", "-CAkey", "mpr.key", "-days", "365", "-sha256", "-extfile", "v3.ext"])
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()
                .unwrap();
            command.stdin.unwrap().write(csr.as_ref()).unwrap();
            let mut output = String::new();
            command.stdout.unwrap().read_to_string(&mut output).unwrap();
            std::fs::remove_file("v3.ext").unwrap();
            output
        });

    warp::serve(route)
        .tls()
        .cert_path(format!("{}/mpr.crt", path))
        .key_path(format!("{}/mpr.key", path))
        .run(([192, 168, 1, 114], 4000)).await;
}