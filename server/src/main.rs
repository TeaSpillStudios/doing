use std::sync::Arc;
use tokio::{
    io::AsyncReadExt,
    sync::{Mutex, MutexGuard},
};

use server::tasks::TaskHandler;
use tokio::{io::AsyncWriteExt, net::TcpListener};

use log::{error, info};

const ADDR: &str = "localhost:2500";

// Add some test tasks.
fn setup_test_task_handler(task_handler: &mut MutexGuard<'_, TaskHandler<'_>>) {
    task_handler.add_section("Test");
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

    let task_handler = Arc::new(Mutex::new(TaskHandler::default())).clone();

    setup_test_task_handler(&mut task_handler.lock().await);

    let listener = TcpListener::bind(ADDR).await.unwrap();

    let server = async move {
        while let Ok((mut stream, _socket_addr)) = listener.accept().await {
            let task_handler = task_handler.clone();
            tokio::spawn(async move {
                let map = task_handler.lock().await;

                println!("Current tasks:");

                for task in map.get_tasks().unwrap() {
                    println!("    {} - {}", task.0, task.1);
                }

                let mut buf: [u8; 128] = [0; 128];

                if let Err(e) = stream.read(&mut buf).await {
                    error!("Error reading stream: {e}");
                    return;
                }

                let data: Vec<String> = String::from_utf8_lossy(&buf)
                    .split('|')
                    .map(|v| v.to_string())
                    .collect();

                let response = match data[0].as_str() {
                    "list" => {
                        let task_map = map.get_tasks().unwrap();

                        task_map
                            .iter()
                            .map(|v| format!("{} - {}\n", v.0, v.1))
                            .collect::<String>()
                    }

                    _ => String::from("Invalid command."),
                };

                if let Err(e) = stream.write(response.as_bytes()).await {
                    error!("Error writing to stream: {e}");
                    return;
                }
            });
        }
    };

    info!("Server running on {ADDR}");

    server.await;
}
