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

    config
        .listen
        .endpoints
        .push("tcp/0.0.0.0:7447".parse().unwrap());

    let session = zenoh::open(config.clone()).await.unwrap();
    let expr_id = session.declare_expr("/resource/name").await.unwrap();

    session.declare_publication(expr_id).await.unwrap();

    let mut subscriber = session.subscribe("/resource/name").await.unwrap();
    let mut i = 0;

    loop {
        tokio::select! {
            sample = subscriber.receiver().recv_async() => if let Ok(sample) = sample {
                let msg = String::from_utf8_lossy(&sample.value.payload.contiguous()).to_string();
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
