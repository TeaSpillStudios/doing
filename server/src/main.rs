// use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::task_handler::{Task, TaskHandler};
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

// Add some test tasks.
fn setup_test_task_handler(task_handler: &mut TaskHandler) {
    task_handler.select_section("Test");
    task_handler.add_task("Test-task1", "Hi", false);

    task_handler.add_task("Test-task2", "Hi", false);
    task_handler.add_task("Test-task3", "Hi", false);
    task_handler.add_task("Test-task4", "Hi", false);

    task_handler.add_section("Testing");
    task_handler.select_section("Testing");

    task_handler.add_task("Test-task5", "Hi", false);
    task_handler.add_task("Test-task6", "Hi", false);
    task_handler.add_task("Test-task7", "Hi", false);
    task_handler.add_task("Test-task8", "Hi", false);
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let mut task_handler = TaskHandler::default();

    setup_test_task_handler(&mut task_handler);

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
