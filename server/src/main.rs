mod task_handler;
use task_handler::*;

fn main() {
    pretty_env_logger::init();

    let mut _task_handler = TaskHandler::default();
}
