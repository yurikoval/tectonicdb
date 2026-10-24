extern crate tdb_cli;
extern crate tdb_server_core;

use std::fs;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tdb_server_core::async_std::task;

async fn connect(host: &str, port: &str) -> tdb_cli::client::TectonicClient {
    for _ in 0..100 {
        if let Ok(client) = tdb_cli::client::TectonicClient::new(host, port) {
            return client;
        }
        task::sleep(Duration::from_millis(10)).await;
    }
    panic!("server did not become ready");
}

#[test]
fn it_works() {
    let host = "127.0.0.1";
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dtf_folder = std::env::temp_dir().join(format!(
        "tectonicdb-integration-{}-{}",
        std::process::id(),
        unique
    ));

    let settings = Arc::new(tdb_server_core::settings::Settings {
        autoflush: false,
        dtf_folder: dtf_folder.to_string_lossy().into_owned(),
        flush_interval: 1000,
        granularity: 1000,
        q_capacity: 1000,
        influx: None,
    });

    task::block_on(async move {
        let listener = tdb_server_core::async_std::net::TcpListener::bind((host, 0))
            .await
            .unwrap();
        let port = listener.local_addr().unwrap().port().to_string();
        let server = task::spawn(async move {
            tdb_server_core::server::run_server_with_listener(listener, settings)
                .await
                .unwrap()
        });

        let cli = connect(host, &port).await;
        tdb_cli::benchmark(cli, 10_000);

        let mut cli = connect(host, &port).await;
        cli.use_db("benchmark").unwrap();

        let mut count = String::new();
        for _ in 0..1_000 {
            count = cli.cmd("COUNT ALL IN MEM\n").unwrap();
            if count == "10000" {
                break;
            }
            task::sleep(Duration::from_millis(10)).await;
        }
        assert_eq!(count, "10000");

        server.cancel().await;
        let _ = fs::remove_dir_all(dtf_folder);
    });
}
