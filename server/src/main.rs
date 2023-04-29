// use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::task_handler::Task;
use tokio::net::TcpListener;

use log::{error, info};

// use log::info;

mod task_handler;
// use task_handler::{Task, TaskHandler};

const ADDR: &str = "localhost:2500";

impl std::fmt::Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.description)
    }
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let listener = TcpListener::bind(ADDR).await.unwrap();

    let server = async move {
        while let Ok((mut stream, _socket_addr)) = listener.accept().await {
            tokio::spawn(async move {
                let (mut reader, mut writer) = stream.split();

                match tokio::io::copy(&mut reader, &mut writer).await {
                    Ok(amount) => {
                        info!("Wrote {} bytes", amount);
                    }
                    Err(err) => {
                        error!("IO error {:?}", err);
                    }
                }
            });
        }
    };

    info!("Server running on {ADDR}");

    server.await;
}
