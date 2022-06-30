use zenoh::prelude::*;

fn main() {
    let _ = env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .try_init();
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(start());
}

pub async fn start() {
    let mut config = config::peer();
    config.set_local_routing(Some(false)).unwrap();

    // config
    //     .connect
    //     .endpoints
    //     .push("tls/peer0:7447".parse().unwrap());

    // config
    //     .connect
    //     .endpoints
    //     .push("quic/peer2:7449".parse().unwrap());
    config
        .listen
        .endpoints
        .push("tcp/0.0.0.0:7447".parse().unwrap());

    // config
    //     .transport
    //     .link
    //     .tls
    //     .set_root_ca_certificate(Some("./tls/minica.pem".to_string()))
    //     .unwrap();

    // config
    //     .transport
    //     .link
    //     .tls
    //     .set_server_certificate(Some("./tls/peer1/cert.pem".to_string()))
    //     .unwrap();

    // config
    //     .transport
    //     .link
    //     .tls
    //     .set_server_private_key(Some("./tls/peer1/key.pem".to_string()))
    //     .unwrap();

    let session = zenoh::open(config.clone()).await.unwrap();
    let expr_id = session.declare_expr("/resource/name").await.unwrap();

    session.declare_publication(expr_id).await.unwrap();

    let mut subscriber = session.subscribe("/resource/name").await.unwrap();
    let mut i = 0;

    loop {
        tokio::select! {
            sample = subscriber.receiver().recv_async() => if let Ok(sample) = sample {
                let msg = String::from_utf8_lossy(&sample.value.payload.contiguous()).to_string();
                i = msg.split(": ").last().unwrap().parse::<usize>().unwrap();
                println!(
                    "Received : {:?}",
                    &msg
                );
            },
            _ = tokio::time::sleep(std::time::Duration::from_millis(1000)) => {
                session.put(expr_id, format!("peer1: {}", i)).await.unwrap();
            },
        };
        i += 1;
    }
}
