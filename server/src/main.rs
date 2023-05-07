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
fn setup_test_task_handler(task_handler: &mut MutexGuard<'_, TaskHandler>) {
    task_handler.add_and_select_section("ToDo".to_owned());

    task_handler.add_task("Test-task1".to_owned(), "A test".to_owned(), false);
    task_handler.add_task("Test-task2".to_owned(), "Another test".to_owned(), false);
    task_handler.add_task("Test-task3".to_owned(), "A test task".to_owned(), false);
    task_handler.add_task("Test-task4".to_owned(), "Task handling ".to_owned(), false);

    task_handler.add_and_select_section("Testing".to_owned());

    task_handler.add_task("Test-task5".to_owned(), "Testing tasks".to_owned(), false);
    task_handler.add_task("Test-task6".to_owned(), "Do extra things".to_owned(), false);
    task_handler.add_task("Test-task7".to_owned(), "A test task".to_owned(), false);
    task_handler.add_task("Test-task8".to_owned(), "Handle tasks".to_owned(), false);
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
                let mut map = task_handler.lock().await;
                let mut buf: [u8; 128] = [0; 128];

                if let Err(e) = stream.read(&mut buf).await {
                    error!("Error reading stream: {e}");
                    return;
                }

                let data: Vec<String> = String::from_utf8_lossy(&buf)
                    .split('|')
                    .map(|v| v.trim_matches(char::from(0)).to_string())
                    .collect();

                info!("Got command \"{}\"", &data[0].as_str());

                let response = match data[0].as_str() {
                    "add_section" => {
                        map.add_section(data[1].clone());
                        String::from("Succeeded")
                    }

                    "list_sections" => {
                        info!("Listing sections");

                        map.list_sections()
                            .iter()
                            .map(|v| format!("{v}"))
                            .collect::<String>()
                    }

                    "add_and_select_section" => {
                        map.add_and_select_section(data[1].clone());
                        String::from("Succeeded")
                    }

                    "add_task" => {
                        map.add_task(
                            data[1].clone(),
                            data[2].clone(),
                            data[3].clone().parse().unwrap(),
                        );

                        String::from("Succeeded")
                    }

                    "set_task_completion" => {
                        map.set_task_completion(data[1].clone(), data[2].parse().unwrap());
                        String::from("Succeeded")
                    }

                    "is_section_completed" => map.is_section_completed().to_string(),

                    "select_section" => {
                        map.select_section(data[1].clone());
                        String::from("Succeeded")
                    }

                    "remove_section" => {
                        map.remove_section(data[1].clone());
                        String::from("Succeeded")
                    }

                    "remove_task" => {
                        map.remove_task(data[1].clone());
                        String::from("Succeeded")
                    }

                    "list" => {
                        let task_map = map.get_tasks().unwrap();

                        task_map
                            .iter()
                            .map(|v| format!("{} - {}\n", v.0, v.1))
                            .collect::<String>()
                    }

                    _ => format!("Invalid command: \"{}\"", data[0]),
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
