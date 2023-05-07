use std::sync::Arc;
use tokio::{
    io::AsyncReadExt,
    sync::{Mutex, MutexGuard},
};

use server::tasks::TaskHandler;
use tokio::{io::AsyncWriteExt, net::TcpListener};

use tracing::{error, info};

const ADDR: &str = "localhost:2500";

// Add some test tasks.
fn setup_test_task_handler(task_handler: &mut MutexGuard<'_, TaskHandler<'_>>) {
    task_handler.add_section("Test");
    task_handler.select_section("Test");
    task_handler.add_task("Test-task1".to_owned(), "Hi".to_owned(), false);

    task_handler.add_task("Test-task2".to_owned(), "Hi".to_owned(), false);
    task_handler.add_task("Test-task3".to_owned(), "Hi".to_owned(), false);
    task_handler.add_task("Test-task4".to_owned(), "Hi".to_owned(), false);

    task_handler.add_section("Testing");
    task_handler.select_section("Testing");

    task_handler.add_task("Test-task5".to_owned(), "Hi".to_owned(), false);
    task_handler.add_task("Test-task6".to_owned(), "Hi".to_owned(), false);
    task_handler.add_task("Test-task7".to_owned(), "Hi".to_owned(), false);
    task_handler.add_task("Test-task8".to_owned(), "Hi".to_owned(), false);
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().without_time().init();

    let task_handler = Arc::new(Mutex::new(TaskHandler::default())).clone();

    setup_test_task_handler(&mut task_handler.lock().await);

    let listener = TcpListener::bind(ADDR).await.unwrap();

    let server = async move {
        while let Ok((mut stream, _socket_addr)) = listener.accept().await {
            let task_handler = task_handler.clone();
            tokio::spawn(async move {
                info!("New connection");

                let mut map = task_handler.lock().await;
                let mut buf: [u8; 128] = [0; 128];

                if let Err(e) = stream.read(&mut buf).await {
                    error!("Error reading stream: {e}");
                    return;
                }

                let data: Vec<String> = String::from_utf8_lossy(&buf)
                    .split('|')
                    .map(|v| v.to_string())
                    .collect();

                info!("Command \"{}\"", &data[0].clone());

                let response = match data[0].as_str() {
                    "list" => {
                        info!("Listing tasks");

                        let task_map = map.get_tasks().unwrap();

                        task_map
                            .iter()
                            .map(|v| format!("{} - {}\n", v.0, v.1))
                            .collect::<String>()
                    }

                    "add_task" => {
                        info!("Adding a task");

                        map.add_task(
                            data[1].clone(),
                            data[2].clone(),
                            data[3].clone().parse().unwrap(),
                        );

                        String::from("Succeeded")
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
